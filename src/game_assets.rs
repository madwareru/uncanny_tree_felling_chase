use {
    macroquad::prelude::*,
    macroquad::miniquad::{TextureParams, TextureFormat, TextureWrap},
    ron::de::from_reader,
    std::sync::Arc
};

const ATLAS_DEFINITION_BYTES: &[u8] = include_bytes!("../assets/uncanny_atlas.ron");
const ATLAS_BYTES: &[u8] = include_bytes!("../assets/uncanny_atlas.png");

pub struct GameAssets {
    pub atlas_definition: Arc<crate::atlas_serialization::AtlasDefinition>,
    pub atlas_texture: Texture2D
}

impl GameAssets {
    pub(crate) fn load() -> Self {
        let ctx: &mut macroquad::prelude::miniquad::Context = {
            let InternalGlContext {
                quad_context: ctx, ..
            } = unsafe { get_internal_gl() };
            ctx
        };

        let img = image::load_from_memory(ATLAS_BYTES)
            .unwrap_or_else(|e| panic!("{}", e))
            .to_rgba8();

        let img_w = img.width();
        let img_h = img.height();

        let atlas_texture = Texture2D::from_miniquad_texture(
            miniquad::Texture::from_data_and_format(
                ctx,
                &img.into_raw(),
                TextureParams {
                    format: TextureFormat::RGBA8,
                    wrap: TextureWrap::Clamp,
                    filter: FilterMode::Nearest,
                    width: img_w,
                    height: img_h,
                },
            )
        );

        Self {
            atlas_texture,
            atlas_definition: Arc::new(from_reader(ATLAS_DEFINITION_BYTES).unwrap())
        }
    }
}