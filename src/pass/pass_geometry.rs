use log::error;
use crate::commands::prelude::RenderCommand;
use crate::context::PassContext;
use crate::graph::{Pass, PassAttachment};
use crate::sort_key::{PassLayer, SortKey, SortKeyBitMap, SortKeyBuilder};

/// Forward geometry pass. Draws all DrawCall / DrawIndirect commands
/// whose sort_key layer matches `self.layer`.
pub struct GeometryPass {
    pub name:              &'static str,
    /// Heirarchy of ui
    pub layer:             PassLayer,

    pub color_attachments: Vec<PassAttachment>,
    pub depth_attachment:  Option<PassAttachment>,
    /// Optional background fill colour, draws over if empty
    pub clear_color:       Option<[f64; 4]>,
    /// Resets depth texture before frame, use in first pass or something.
    pub clear_depth:       bool,
}

impl GeometryPass {
    pub fn opaque(name: &'static str) -> Self {
        Self {
            name, layer: PassLayer::Opaque,
            color_attachments: vec![], depth_attachment: None,
            clear_color: Some([0.0, 0.0, 0.0, 1.0]),
            clear_depth: true,
        }
    }

    pub fn transparent(name: &'static str) -> Self {
        Self {
            name, layer: PassLayer::Transparent,
            color_attachments: vec![], depth_attachment: None,
            clear_color: None, clear_depth: false,
        }
    }

    pub fn with_clear_color(mut self, r: f64, g: f64, b: f64, a: f64) -> Self {
        self.clear_color = Some([r, g, b, a]); self
    }
}
impl Pass for GeometryPass {
    fn name(&self) -> &str { self.name }
    fn reads(&self) -> &[PassAttachment] { &[] }
    fn writes(&self) -> &[PassAttachment] { &self.color_attachments }
    ///
    fn execute(&self, ctx: &mut PassContext<'_>) {
        let frame = ctx.surface.acquire().expect("GeometryPass: acquire failed");
        //how we want to use the frame texture
        let frame_view = frame.texture.create_view(&Default::default());//TODO! look into other params instead of default
        //what we do with the texture before drawing onto it
        let colour_op = wgpu::Operations {
            load: match self.clear_color {
                //If background colour exists, use it
                Some(c) => wgpu::LoadOp::Clear(wgpu::Color { r: c[0], g: c[1], b: c[2], a: c[3] }),
                //Else, draw on top
                None    => wgpu::LoadOp::Load,
                //DontCare also exists if your overwritting everything
            },
            store: wgpu::StoreOp::Store
        };
        //Try to get a view of a mip texture, resolve_target is the texture the msaa applied image will go to
        let (color_view, resolve_target) = match ctx.surface.msaa_view() {
            //if an msaa view exists for that surface (mip level > 1), we draw to the mip level
            Some(msaa) => (msaa, Some(&frame_view)),
            //else, lets just draw directly on the frame
            None       => (&frame_view, None),
        };

        let mut render_pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(self.name),
            //where fragment shader colour outputs go
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                //texture you render to
                view: color_view,
                //This is the final image if msaa exists, if it does the algorithm is applied to this texture.
                resolve_target,
                //What we do before drawing.
                ops: colour_op,
                //Don't worry about this lil bro unless it's a 3d texture
                depth_slice: None
            })],
            //A stencil is a 2D texture that contains the final image after the fragment shader
            //Allows you to do post processing effects such as disable rendering for specific areas
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                //depth texture, describes how deep each pixel is
                view: ctx.surface.depth_view(),
                depth_ops: Some(wgpu::Operations {
                    load:  if self.clear_depth { wgpu::LoadOp::Clear(1.0) } else { wgpu::LoadOp::Load },
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes:    None,
            //gives you information on how many vertices of an object are visible
            occlusion_query_set: None,
            // Render to multiple views, for something like VR
            multiview_mask: None,
        });
        //TODO! Make this more orgasmic.
        let layer_mask  = 0xFF_u64 << 56;
        let layer_bits  = SortKeyBuilder::new().layer(self.layer).build();
        let mut current_pipeline = None;

        // Commands are already sorted by sort_key, just filter for this layer.
        for cmd in ctx.commands {
            let sort_key = match cmd {
                RenderCommand::Draw(d) => d.sort_key,
                RenderCommand::DrawIndirect { sort_key, .. } => *sort_key,
                _ => continue,
            };
            if (sort_key & layer_mask) != layer_bits { continue; }

            match cmd {
                RenderCommand::Draw(call) => {
                    if current_pipeline != Some(call.pipeline) {
                        //TODO! throw rendere rror if handle is invalid
                        if let Some(p) = ctx.resources.get_pipeline(call.pipeline)
                            .and_then(|p| p.as_render())
                        {
                            render_pass.set_pipeline(p);
                            current_pipeline = Some(call.pipeline);
                        }
                    }
                    //initialize bind groups
                    for (i, &bg) in call.bind_groups.iter().enumerate() {
                        if let Some(b) = ctx.resources.get_bind_group(bg) {
                            render_pass.set_bind_group(i as u32, &b.bind_group, &[]);
                        }
                    }
                    //TODO! replace push constants with immediates:
                    /*
                    if call.push_constants.iter().any(|&b| b != 0) {
                        render_pass.set_immediates(0, );
                    }
                     */

                    if let Some(mesh) = ctx.resources.get_mesh(call.mesh) {
                        if let Some(s) = call.scissor {
                            // only render to part of screen
                            render_pass.set_scissor_rect(s.x, s.y, s.width, s.height);
                        }
                        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                        //if there is an index buffer, use that to draw else unindexed
                        match &mesh.index_buffer {
                            Some(ib) => {
                                render_pass.set_index_buffer(ib.slice(..), mesh.index_format.unwrap());
                                render_pass.draw_indexed(0..mesh.element_count, 0, call.instances.clone());
                            }
                            None => render_pass.draw(0..mesh.element_count, call.instances.clone()),
                        }
                    }
                }

                RenderCommand::DrawIndirect {
                    pipeline, mesh, bind_groups, push_constants,
                    indirect_buffer, indirect_offset, draw_count, ..
                } => {
                    if current_pipeline != Some(*pipeline) {
                        if let Some(p) = ctx.resources.get_pipeline(*pipeline)
                            .and_then(|p| p.as_render())
                        {
                            render_pass.set_pipeline(p);
                            current_pipeline = Some(*pipeline);
                        }
                    }
                    for (i, &bg) in bind_groups.iter().enumerate() {
                        if let Some(b) = ctx.resources.get_bind_group(bg) {
                            render_pass.set_bind_group(i as u32, &b.bind_group, &[]);
                        }
                    }
                    if let (Some(ind), Some(mesh)) = (
                        ctx.resources.get_buffer(*indirect_buffer),
                        ctx.resources.get_mesh(*mesh),
                    ) {
                        render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                        render_pass.multi_draw_indirect(&ind.buffer, *indirect_offset, *draw_count);
                    }
                }
                _ => {}
            }
        }
    }
}

