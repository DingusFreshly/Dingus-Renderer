use std::collections::{HashMap, VecDeque};
use wgpu::{CommandEncoder, Device, Queue, Surface};
use crate::commands::prelude::RenderCommand;
use crate::context::{PassContext, SurfaceContext};
use crate::error::RendererError;
use crate::graph::Pass;
use crate::registry::res_registry::ResourceRegistry;

/// Provides api to sort the command queue, and to flush
pub struct RenderGraph {
    passes: Vec<Box<dyn Pass>>,
    sorted_order: Vec<usize>,
    dirty: bool
}

impl RenderGraph {
    pub fn new() -> Self {Self {passes: Vec::new(), sorted_order: vec![], dirty: true}}
    pub fn add_pass<P: Pass + 'static>(&mut self, pass: P) { self.passes.push(Box::new(pass)); }
    pub fn remove_pass<P:Pass + 'static>(&mut self, name: &str) {self.passes.retain(|p| {p.name() != name})}
    pub fn has_pass(&self, name: &str) -> bool { self.passes.iter().any(|p| p.name() == name) }
    ///Uses khans algorithm to resolve dependancy issues, returns `RendererError` if there is a cyclic dependency.
    pub fn build(&mut self) -> Result<(), RendererError> {
        let n = self.passes.len();
        if n == 0 { self.sorted_order.clear(); self.dirty = false; return Ok(()); }

        /*Example
        letters are pass indexes, numbers are texture dependencies
        A:
            reads  1
        B:
            writes 1
        need to encode that A depends on B
        iterate through, and collect the textures each command depends on

        let reads = Vec[
            A: [1]
            B: []
        ]
        Iterate through again, this time looking at what each pass writes too
        let dependency = Vec [
            A: [B]
            B: []

        ]
         */
        //maps texture handles to the pass indexes that write to it
        let mut reads = HashMap::new();
        for (i, pass) in self.passes.iter().enumerate() {
            for att in pass.reads() {
                reads
                    .entry(att.handle)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
        //pass -> list of passes that depend on it
        //pass -> list of passes that read what i write
        //loop through (pass, passes_i_write_to), and see what passes share my writes
        let mut adj = vec![vec![]; n];
        let mut in_deg = vec![0;n];
        //get all the textures every pass writes, and mark the passes that read that texture as dependant on it

        for (i, pass) in self.passes.iter().enumerate() {
            for write in pass.writes() {
                if let Some(texture_viewers) = reads.get(&write.handle) {
                    for &reader in texture_viewers {
                        if reader != i {
                            adj[i].push(reader);     // writer → reader
                            in_deg[reader] += 1;     // reader depends on writer
                        }
                    }
                }
            }
        }

        let mut queue : VecDeque<usize> = (0..n).filter(|&i| in_deg[i] == 0).collect();
        let mut order = Vec::with_capacity(n);

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &nb in &adj[node] {
                in_deg[nb] -= 1;
                if in_deg[nb] == 0 { queue.push_back(nb); }
            }
        }

        if order.len() != n {
            let culprit = (0..n)
                .find(|&i| in_deg[i] > 0)
                .map(|i| self.passes[i].name().to_owned())
                .unwrap_or_else(|| "unknown".into());
            return Err(RendererError::RenderGraphCycle(culprit));
        }

        self.sorted_order = order;
        self.dirty = false;
        Ok(())
    }
    ///Creates a pass context and executes all the passes
    pub(crate) fn execute(
        &self,
        encoder: &mut CommandEncoder,
        resources: &ResourceRegistry,
        surface: &SurfaceContext,
        commands: &[RenderCommand],
        frame_idx: u64,
        device: &Device,
        queue: &Queue
    ) {
        for &idx in &self.sorted_order {
            let pass = &self.passes[idx];

            let mut ctx = PassContext {
                encoder,
                resources,
                surface,
                commands,
                frame_idx,
                device,
                queue
            };
            pass.execute(&mut ctx);

            encoder.pop_debug_group();
        }
    }
}