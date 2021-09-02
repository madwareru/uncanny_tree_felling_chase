use macroquad::prelude::*;
use std::cmp::Ordering;
use macro_tiler::atlas::draw_command::DrawCommand;
use macro_tiler::atlas::rect_handle::{DrawSizeOverride, DrawParams};
use std::slice::IterMut;

#[derive(Copy, Clone)]
pub enum RenderLayer {
    TileMap,
    TileMapOverlay,
    MapObjects,
    Custom(usize)
}

#[derive(Copy, Clone)]
pub enum UiRenderLayer {
    Background,
    Buttons,
    ButtonSelection,
    Glyphs,
    Custom(usize)
}

pub struct SceneCompositor {
    layers: Vec<Vec<DrawCommand>>,
    ui_layers: Vec<Vec<DrawCommand>>,
    post_processing_material: Option<macroquad::material::Material>,
    screen_h: u32,
    screen_w: u32,
    render_target: RenderTarget
}

impl SceneCompositor {
    pub fn new() -> Self {
        let screen_w = screen_width().trunc() as u32;
        let screen_h = screen_height().trunc() as u32;
        let render_target = render_target(screen_w, screen_h);
        Self {
            layers: vec![Vec::new(); 16],
            ui_layers: vec![Vec::new(); 16],
            post_processing_material: None,
            screen_w,
            screen_h,
            render_target
        }
    }

    pub fn enqueue(&mut self, layer: RenderLayer, command: DrawCommand) {
        let layer = match layer {
            RenderLayer::TileMap => 0,
            RenderLayer::TileMapOverlay => 1,
            RenderLayer::MapObjects => 2,
            RenderLayer::Custom(offset) => 3 + offset
        };
        if self.layers.capacity() <= layer {
            let diff = self.layers.capacity() - layer + 1;
            self.layers.extend_from_slice(&vec![Vec::new(); diff]);
        }
        self.layers[layer].push(command);
    }

    pub fn set_postprocessing_material(&mut self, mat: macroquad::material::Material) {
        self.post_processing_material = Some(mat);
    }

    pub fn clear_postprocessing_material(&mut self) {
        self.post_processing_material = None;
    }

    pub fn enqueue_ui(&mut self, layer: UiRenderLayer, command: DrawCommand) {
        let layer = match layer {
            UiRenderLayer::Background => 0,
            UiRenderLayer::Buttons => 1,
            UiRenderLayer::ButtonSelection => 2,
            UiRenderLayer::Glyphs => 3,
            UiRenderLayer::Custom(offset) => 4 + offset
        };
        if self.ui_layers.capacity() <= layer {
            let diff = self.ui_layers.capacity() - layer + 1;
            self.ui_layers.extend_from_slice(&vec![Vec::new(); diff]);
        }
        self.ui_layers[layer].push(command);
    }

    pub fn flush_drawing_queue(&mut self, scene_width: usize, scene_height: usize, draw_scale: f32) {
        let dpi_scale = macro_tiler::utility::with_context(|ctx| ctx.dpi_scale());

        let start_x = screen_width() / dpi_scale;
        let start_x = (start_x - (scene_width) as f32 * draw_scale) / 2.0;

        let start_y = screen_height() / dpi_scale;
        let start_y = (start_y - (scene_height) as f32 * draw_scale) / 2.0;

        match self.post_processing_material {
            None => {
                set_default_camera();
                clear_background(Color::new(0.12, 0.1, 0.15, 1.00));
                flush_layers(draw_scale, dpi_scale, start_x, start_y, self.layers.iter_mut());
                flush_layers(draw_scale, dpi_scale, start_x, start_y, self.ui_layers.iter_mut());
            }
            Some(material) => {
                let screen_w = screen_width().trunc() as u32;
                let screen_h = screen_height().trunc() as u32;
                if self.screen_w != screen_w || self.screen_h != screen_h {
                    self.render_target = render_target(screen_w, screen_h);
                    self.screen_w = screen_w;
                    self.screen_h = screen_h;
                }

                set_camera(&Camera2D {
                    zoom: vec2(1.0 / (screen_width() * 0.5), 1.0 / (screen_height() * 0.5)),
                    target: vec2(screen_width() * 0.5, screen_height() * 0.5),
                    render_target: Some(self.render_target),
                    ..Default::default()
                });
                clear_background(Color::new(0.12, 0.1, 0.15, 1.00));
                flush_layers(draw_scale, dpi_scale, start_x, start_y, self.layers.iter_mut());

                set_default_camera();

                material.set_uniform("time", get_time() as f32);
                gl_use_material(material);
                draw_texture_ex(
                    self.render_target.texture,
                    0.,
                    0.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(
                            screen_width(),
                            screen_height()
                        )),
                        ..Default::default()
                    },
                );
                gl_use_default_material();
                flush_layers(draw_scale, dpi_scale, start_x, start_y, self.ui_layers.iter_mut());
            }
        }
    }
}

fn flush_layers(
    draw_scale: f32,
    dpi_scale: f32,
    start_x: f32,
    start_y: f32,
    layers_mut: IterMut<Vec<DrawCommand>>)
{
    for layer in layers_mut {
        layer.sort_by(|lhs, rhs| {
            let (dy, dx) = (lhs.y - rhs.y, lhs.x - rhs.x);
            if dy.abs() < 0.0001 {
                if dx.abs() < 0.0001 {
                    Ordering::Equal
                } else if dx < 0.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else if dy < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        for command in layer.iter() {
            let draw_params_scaled = DrawParams {
                size_override: match command.draw_params.size_override {
                    DrawSizeOverride::None => {
                        DrawSizeOverride::ScaledUniform(draw_scale * dpi_scale)
                    },
                    DrawSizeOverride::ScaledUniform(scale_initial) => {
                        DrawSizeOverride::ScaledUniform(scale_initial * draw_scale * dpi_scale)
                    },
                    DrawSizeOverride::ScaledNonUniform(scale_initial) => {
                        DrawSizeOverride::ScaledNonUniform(
                            [
                                scale_initial.x * draw_scale * dpi_scale,
                                scale_initial.y * draw_scale * dpi_scale
                            ].into()
                        )
                    },
                    DrawSizeOverride::Exact(scale_initial) => {
                        DrawSizeOverride::Exact(
                            [
                                scale_initial.x * draw_scale * dpi_scale,
                                scale_initial.y * draw_scale * dpi_scale
                            ].into()
                        )
                    }
                },
                ..command.draw_params
            };
            command.handle.draw(
                (start_x + command.x * draw_scale) * dpi_scale,
                (start_y + command.y * draw_scale) * dpi_scale,
                &draw_params_scaled
            );
        }
        layer.clear();
    }
}