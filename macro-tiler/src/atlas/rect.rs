use {serde::Deserialize};

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct AtlasRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize
}