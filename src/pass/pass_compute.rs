use bytemuck::cast_slice;
use wgpu::{BufferAddress, ComputePassDescriptor};
use wgpu::wgc::command;
use crate::commands::prelude::RenderCommand;
use crate::context::{PassContext};
use crate::graph::{Pass, PassAttachment};

///Holdes/Uses all Compute(Indirect) commands
struct ComputePass<'a> {

    name : &'a str,
    reads: Vec<PassAttachment>,
    writes: Vec<PassAttachment>,

}

impl<'a> ComputePass<'a> {

    fn new(name: &'a str) -> Self { Self {name, reads: Vec::new(), writes: Vec::new()} }
    fn with_reads(self, reads: Vec<PassAttachment>) -> Self { Self {name: self.name, reads, writes: self.writes} }
    fn with_writes(self, writes: Vec<PassAttachment>) -> Self { Self {name: self.name, reads: self.reads, writes} }

}

impl<'a> Pass for ComputePass<'a> {

    fn name(&self) -> &str { return self.name }
    fn reads(&self) -> &[PassAttachment] { &self.reads }
    fn writes(&self) -> &[PassAttachment] { &self.writes }

    fn execute(&self, ctx: &mut PassContext<'_>) {

        let mut compute_pass = ctx.encoder.begin_compute_pass(&ComputePassDescriptor::default());

        for command in ctx.commands.iter() {

            match command {

                RenderCommand::Compute(call) => {

                    let pipeline = ctx.resources.get_pipeline(call.pipeline).unwrap();

                    compute_pass.set_pipeline(pipeline.as_compute().unwrap());

                    for(i,  bind_group)in call.bind_groups.iter().enumerate() {

                        let cached_bind_group = ctx.resources.get_bind_group(*bind_group).unwrap();
                        compute_pass.set_bind_group(i as u32, Some(&cached_bind_group.bind_group), &[])

                    }



                    compute_pass.dispatch_workgroups(call.dispatch[0], call.dispatch[1], call.dispatch[2])

                }

                RenderCommand::ComputeIndirect{
                    indirect_buffer,
                    indirect_offset,
                    pipeline,
                    bind_groups,
                    ..
                } => {

                    let pipeline = ctx.resources.get_pipeline(*pipeline).unwrap();

                    compute_pass.set_pipeline(pipeline.as_compute().unwrap());

                    for(i,  bind_group)in bind_groups.iter().enumerate() {

                        let cached_bind_group = ctx.resources.get_bind_group(*bind_group).unwrap();
                        compute_pass.set_bind_group(i as u32, Some(&cached_bind_group.bind_group), &[])

                    }

                    //compute_pass.set_immediates(offset, data) <-- TODO

                    let gpu_buffer = ctx.resources.get_buffer(*indirect_buffer).unwrap();
                    compute_pass.dispatch_workgroups_indirect(&gpu_buffer.buffer, *indirect_offset)

                }
                _ => { }

            }

        }

    }

}