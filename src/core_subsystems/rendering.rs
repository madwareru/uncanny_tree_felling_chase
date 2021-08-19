use macroquad::prelude::*;
use std::cmp::Ordering;
use macro_tiler::atlas::draw_command::DrawCommand;
use macro_tiler::atlas::rect_handle::{DrawSizeOverride, DrawParams};

pub struct SceneCompositor {
    layers: Vec<Vec<DrawCommand>>
}

impl SceneCompositor {
    pub fn new() -> Self {
        Self {
            layers: vec![Vec::new(); 2048]
        }
    }

    pub fn enqueue(&mut self, layer: usize, command: DrawCommand) {
        assert!(layer < 2048);
        self.layers[layer].push(command);
    }

    pub fn flush_drawing_queue(&mut self, scene_width: usize, scene_height: usize, draw_scale: f32) {
        let dpi_scale = macro_tiler::utility::with_context(|ctx| ctx.dpi_scale());

        let start_x = screen_width() / dpi_scale;
        let start_x = (start_x - (scene_width) as f32 * draw_scale) / 2.0;

        let start_y = screen_height() / dpi_scale;
        let start_y = (start_y - (scene_height) as f32 * draw_scale) / 2.0;

        for layer in self.layers.iter_mut() {
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
}