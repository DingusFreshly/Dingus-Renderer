use crate::graph::access::PassAttachment;
use crate::context::PassContext;

pub trait Pass: Send + Sync {
    fn name(&self)   -> &str;
    fn reads(&self)  -> &[PassAttachment];
    fn writes(&self) -> &[PassAttachment];
    fn execute(&self, ctx: &mut PassContext<'_>);
}