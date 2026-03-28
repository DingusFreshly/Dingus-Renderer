use wgpu::ComputePassDescriptor;
use wgpu::wgc::command;
use crate::commands::prelude::RenderCommand;
use crate::context::{PassContext};
use crate::graph::{Pass, PassAttachment};

///Holdes/Uses all Compute(Indirect) commands
struct ComputePass {

    name : &'a str,
    reads: Vec<PassAttachment>,
    writes: Vec<PassAttachment>,

}

impl Pass for ComputePass {

    fn new(name: str) -> Self { Self {name, reads: Vec::new(), writes: Vec::new()} }
    fn with_reads(self, reads: Vec<PassAttachment>) -> Self { Self {name: self.name, reads, writes: self.writes} }
    fn with_writes(self, writes: Vec<PassAttachment>) -> Self { Self {name: self.name, reads: self.reads, writes} }

    fn execute(ctx: PassContext) {

        let compute_pass = ctx.encoder.begin_compute_pass(ComputePassDescriptor::default());

        let pipeline = None;

        for command in ctx.commands.iter() {

            match command {

                RenderCommand::Compute(call) => {

                    compute_pass.set_pipeline(call.pipeline);
                    compute_pass.set_bind_group(call.bind_groups);

                    compute_pass.dispatch_workgroups()

                }

                RenderCommand::ComputeIndirect{
                        indirect_buffer,
                        indirect_offset,
                        pipeline,
                        bind_groups,
                    } => {

                    compute_pass.set_pipeline(pipeline);
                    compute_pass.set_bind_group(bind_groups);
                    //compute_pass.set_immediates(offset, data) <-- TODO

                    compute_pass.dispatch_workgroups_indirect(&indirect_buffer, indirect_offset)

                }
                _ => { }

            }

        }

    }

}