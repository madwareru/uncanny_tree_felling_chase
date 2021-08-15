use {
    macroquad::prelude::*,
    macroquad::miniquad::{TextureParams, TextureFormat, TextureWrap},
    ron::de::from_reader,
    std::sync::Arc
};
use macroquad::miniquad::Context;

const ATLAS_DEFINITION_BYTES: &[u8] = include_bytes!("../assets/atlas_definition.ron");
const ATLAS_BYTES: &[u8] = include_bytes!("../assets/main_atlas.png");
const UI_ATLAS_BYTES: &[u8] = include_bytes!("../assets/user_interface_atlas.png");

pub struct GameAssets {
    pub atlas_definition: Arc<crate::core_subsystems::atlas_serialization::AtlasDefinition>,
    pub atlas_texture: Texture2D,
    pub ui_atlas_texture: Texture2D,
}

impl GameAssets {
    pub(crate) fn load() -> Self {
        let ctx: &mut macroquad::prelude::miniquad::Context = {
            let InternalGlContext {
                quad_context: ctx, ..
            } = unsafe { get_internal_gl() };
            ctx
        };

        let atlas_texture = GameAssets::load_png_texture(ctx, ATLAS_BYTES);
        let ui_atlas_texture = GameAssets::load_png_texture(ctx, UI_ATLAS_BYTES);

        Self {
            atlas_texture,
            ui_atlas_texture,
            atlas_definition: Arc::new(from_reader(ATLAS_DEFINITION_BYTES).unwrap())
        }
    }

    fn load_png_texture(ctx: &mut Context, bytes: &[u8]) -> Texture2D {
        let img = image::load_from_memory(bytes)
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
        atlas_texture
    }
}