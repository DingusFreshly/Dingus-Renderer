use crate::handle::aliases::PipelineHandle;

/// The broad rendering phase a draw call belongs to.
/// Organizes the passes.
/// Lower values execute first.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PassLayer {
    ///Use for things like skybox
    Background  = 0,
    ///Most solid geometry
    Opaque      = 1,
    AlphaTest   = 2,
    Transparent = 3,
    Particles   = 4,
    PostProcess = 5,
    Ui          = 6,
}

pub enum SortKeyBitMap {
    Depth = 0,
    MaterialHash = 24,
    PipelineId = 40,
    PassLayer = 56
}

/// 64-bit sort key bit layout:
///
///  [63..56] PassLayer   (8  bits) : rendering phase
///  [55..40] Pipeline ID (16 bits) : minimise pipeline state changes
///  [39..24] Material Hash    (16 bits) : minimise bind group changes
///  [23..0 ] Depth       (24 bits) : front-to-back or back-to-front
pub struct SortKey;



pub struct SortKeyBuilder {
    key: u64,
}
impl SortKeyBuilder {
    pub fn layer(mut self, layer: PassLayer) -> Self {
        self.key = (self.key & 0x00FF_FFFF_FFFF_FFFF) | ((layer as u64) << SortKeyBitMap::PassLayer as u64);
        self
    }

    pub fn pipeline(mut self, h: PipelineHandle) -> Self {
        let p = (h.slot_index as u64) & 0xFFFF;
        self.key = (self.key & 0xFF00_0000_FFFF_FFFF) | (p << SortKeyBitMap::PipelineId as u64);
        self
    }

    pub fn material(mut self, hash: u16) -> Self {
        self.key = (self.key & 0xFFFF_FF00_00FF_FFFF) | ((hash as u64) << SortKeyBitMap::MaterialHash as u64);
        self
    }

    /// Near-to-far depth encoding. Use for opaque geometry to maximize early-Z rejection.
    pub fn depth_near_to_far(mut self, z: f32) -> Self {
        let d = ((z / 10_000.0).clamp(0.0, 1.0) * 0xFF_FFFF as f32) as u64;
        self.key = (self.key & 0xFFFF_FFFF_FF00_0000) | d;
        self
    }

    /// Far-to-near depth encoding. Use for transparent/particle layers for correct blending.
    pub fn depth_far_to_near(mut self, z: f32) -> Self {
        let d = ((z / 10_000.0).clamp(0.0, 1.0) * 0xFF_FFFF as f32) as u64;
        self.key = (self.key & 0xFFFF_FFFF_FF00_0000) | (0xFF_FFFF - d);
        self
    }

    pub fn build(self) -> u64 { self.key }
}