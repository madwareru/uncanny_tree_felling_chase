use {macroquad::prelude::*, super::rect::*};

#[derive(Copy, Clone)]
pub enum DrawFlip {
    None,
    FlipX,
    FlipY,
    FlipXY,
}

impl Default for DrawFlip {
    fn default() -> Self { Self::None }
}

#[derive(Copy, Clone)]
pub enum DrawPivot {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Relative(Vec2),
    Absolute(Vec2),
}

impl Default for DrawPivot {
    fn default() -> Self { Self::TopLeft }
}

#[derive(Copy, Clone)]
pub enum DrawRotation {
    None,
    RotateAroundPivot(f32),
    RotateAroundPt(Vec2, f32),
}

impl Default for DrawRotation {
    fn default() -> Self { Self::None }
}

#[derive(Copy, Clone)]
pub struct DrawMaterialOverride {
    material: Option<Material>,
}
impl Default for DrawMaterialOverride {
    fn default() -> Self { Self { material: None } }
}
impl DrawMaterialOverride {
    pub fn new(material: Material) -> Self {
        Self { material: Some(material) }
    }
}

#[derive(Copy, Clone)]
pub enum DrawSizeOverride {
    None,
    ScaledUniform(f32),
    ScaledNonUniform(Vec2),
    Exact(Vec2),
}

impl Default for DrawSizeOverride {
    fn default() -> Self { Self::None }
}

#[derive(Copy, Clone)]
pub enum DrawColorOverride {
    None,
    Alpha(f32),
    Tint(Color),
}

impl Default for DrawColorOverride {
    fn default() -> Self { Self::None }
}

pub trait Having<TSubField> {
    fn having(self, new_sub_field: TSubField) -> Self;
}

#[derive(Copy, Clone, Default)]
pub struct DrawParams {
    pub pivot: DrawPivot,
    pub flip: DrawFlip,
    pub rotation: DrawRotation,
    pub size_override: DrawSizeOverride,
    pub color_override: DrawColorOverride,
    pub material_override: DrawMaterialOverride,
}

impl Having<DrawPivot> for DrawParams {
    fn having(self, new_sub_field: DrawPivot) -> Self {
        Self { pivot: new_sub_field, ..self }
    }
}

impl Having<DrawFlip> for DrawParams {
    fn having(self, new_sub_field: DrawFlip) -> Self {
        Self { flip: new_sub_field, ..self }
    }
}

impl Having<DrawRotation> for DrawParams {
    fn having(self, new_sub_field: DrawRotation) -> Self {
        Self { rotation: new_sub_field, ..self }
    }
}

impl Having<DrawSizeOverride> for DrawParams {
    fn having(self, new_sub_field: DrawSizeOverride) -> Self {
        Self { size_override: new_sub_field, ..self }
    }
}

impl Having<DrawColorOverride> for DrawParams {
    fn having(self, new_sub_field: DrawColorOverride) -> Self {
        Self { color_override: new_sub_field, ..self }
    }
}

impl Having<DrawMaterialOverride> for DrawParams {
    fn having(self, new_sub_field: DrawMaterialOverride) -> Self {
        Self { material_override: new_sub_field, ..self }
    }
}

#[derive(Copy, Clone)]
pub struct AtlasRectHandle(pub(crate) Texture2D, pub(crate) AtlasRect);

impl AtlasRectHandle {
    pub fn draw(&self, x: f32, y: f32, draw_params: &DrawParams) {
        let dest_size = match draw_params.size_override {
            DrawSizeOverride::None => [self.1.w as f32, self.1.h as f32].into(),
            DrawSizeOverride::ScaledUniform(scale) => {
                [self.1.w as f32 * scale, self.1.h as f32 * scale].into()
            }
            DrawSizeOverride::ScaledNonUniform(scale) => {
                [self.1.w as f32 * scale.x, self.1.h as f32 * scale.y].into()
            }
            DrawSizeOverride::Exact(size) => size
        };
        let (flip_x, flip_y) = match draw_params.flip {
            DrawFlip::None => (false, false),
            DrawFlip::FlipX => (true, false),
            DrawFlip::FlipY => (false, true),
            DrawFlip::FlipXY => (true, true)
        };
        let (true_x, true_y) = match draw_params.pivot {
            DrawPivot::TopLeft => (x, y),
            DrawPivot::TopRight => (x - dest_size.x, y),
            DrawPivot::BottomLeft => (x, y - dest_size.y),
            DrawPivot::BottomRight => (x - dest_size.x, y - dest_size.y),
            DrawPivot::Relative(relative_pivot) => (
                x - dest_size.x * relative_pivot.x,
                y - dest_size.y * relative_pivot.y
            ),
            DrawPivot::Absolute(absolute_pivot) => (x - absolute_pivot.x, y - absolute_pivot.y)
        };

        let (pivot, rotation) = match draw_params.rotation {
            DrawRotation::None => (None, 0.0),
            DrawRotation::RotateAroundPivot(angle) => (Some([x, y].into()), angle),
            DrawRotation::RotateAroundPt(pt, angle) => (Some(pt), angle),
        };

        let color = match draw_params.color_override {
            DrawColorOverride::None => WHITE,
            DrawColorOverride::Alpha(alpha) => Color::new(1.0, 1.0, 1.0, alpha),
            DrawColorOverride::Tint(tint) => tint
        };

        match draw_params.material_override {
            DrawMaterialOverride { material: Some(mat) } => {
                gl_use_material(mat);
                self.draw_raw(
                    true_x, true_y,
                    flip_x, flip_y,
                    dest_size,
                    pivot,
                    rotation,
                    color,
                );
                gl_use_default_material();
            },
            DrawMaterialOverride { material: None } => {
                self.draw_raw(
                    true_x, true_y,
                    flip_x, flip_y,
                    dest_size,
                    pivot,
                    rotation,
                    color,
                );
            }
        }

        self.draw_raw(
            true_x, true_y,
            flip_x, flip_y,
            dest_size,
            pivot,
            rotation,
            color,
        );
    }

    fn draw_raw(&self,
                x: f32, y: f32,
                flip_x: bool, flip_y: bool,
                dest_size: Vec2,
                pivot: Option<Vec2>,
                rotation: f32,
                color: Color,
    ) {
        draw_texture_ex(
            self.0,
            x, y,
            color,
            DrawTextureParams {
                flip_x,
                flip_y,
                source: Some(Rect::new(
                    self.1.x as f32, self.1.y as f32,
                    self.1.w as f32, self.1.h as f32,
                )),
                dest_size: Some(dest_size),
                pivot,
                rotation,
            },
        )
    }
}