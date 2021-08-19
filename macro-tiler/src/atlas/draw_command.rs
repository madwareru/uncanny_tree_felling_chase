use {super::rect_handle::*};

#[derive(Copy, Clone)]
pub struct DrawCommand {
    pub handle: AtlasRectHandle,
    pub x: f32,
    pub y: f32,
    pub draw_params: DrawParams
}

pub struct AtlasDrawCommandBuilder {
    draw_params: DrawParams
}

impl Having<DrawPivot> for AtlasDrawCommandBuilder {
    fn having(self, new_sub_field: DrawPivot) -> Self {
        Self{ draw_params: self.draw_params.having(new_sub_field) }
    }
}

impl Having<DrawFlip> for AtlasDrawCommandBuilder {
    fn having(self, new_sub_field: DrawFlip) -> Self {
        Self{ draw_params: self.draw_params.having(new_sub_field) }
    }
}

impl Having<DrawRotation> for AtlasDrawCommandBuilder {
    fn having(self, new_sub_field: DrawRotation) -> Self {
        Self{ draw_params: self.draw_params.having(new_sub_field) }
    }
}

impl Having<DrawSizeOverride> for AtlasDrawCommandBuilder {
    fn having(self, new_sub_field: DrawSizeOverride) -> Self {
        Self{ draw_params: self.draw_params.having(new_sub_field) }
    }
}

impl Having<DrawColorOverride> for AtlasDrawCommandBuilder {
    fn having(self, new_sub_field: DrawColorOverride) -> Self {
        Self{ draw_params: self.draw_params.having(new_sub_field) }
    }
}

impl AtlasDrawCommandBuilder {
    pub fn build(self, handle: AtlasRectHandle, x: f32, y: f32) -> DrawCommand {
        DrawCommand {
            handle, x, y, draw_params: self.draw_params
        }
    }
}

pub fn builder() -> AtlasDrawCommandBuilder {
    AtlasDrawCommandBuilder {
        draw_params: Default::default()
    }
}