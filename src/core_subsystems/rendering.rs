use macroquad::prelude::*;
use crate::core_subsystems::atlas_serialization::*;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub enum Pivot {
    Relative { rel_x: f32, rel_y: f32 },
    Absolute { rel_x: f32, rel_y: f32 },
}

#[derive(Copy, Clone)]
pub enum DrawCommandExtra {
    Draw,
    DrawWithPivot { pivot: Pivot }
}

#[derive(Copy, Clone)]
pub struct DrawCommand {
    pub tex: Texture2D,
    pub subrect: SubRect,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub drawing_extra: DrawCommandExtra,
    pub sorting_layer: i32,
}

pub struct SceneCompositor {
    commands: Vec<DrawCommand>,
}

impl SceneCompositor {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn enqueue(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }

    pub fn flush_drawing_queue(&mut self, scene_width: usize, scene_height: usize, draw_scale: f32) {
        self.commands.sort_by(|lhs, rhs| {
            match lhs.sorting_layer.cmp(&rhs.sorting_layer) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
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
                }
                Ordering::Greater => Ordering::Greater
            }
        });

        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        let start_x = screen_width() / ctx.dpi_scale();
        let start_x = (start_x - (scene_width) as f32 * draw_scale) / 2.0;

        let start_y = screen_height() / ctx.dpi_scale();
        let start_y = (start_y - (scene_height) as f32 * draw_scale) / 2.0;

        for command in self.commands.iter() {
            match command.drawing_extra {
                DrawCommandExtra::Draw => {
                    draw_subrect(
                        command.tex,
                        command.subrect,
                        start_x + command.x * draw_scale,
                        start_y + command.y * draw_scale,
                        command.scale * draw_scale,
                    )
                }
                DrawCommandExtra::DrawWithPivot { pivot } => {
                    draw_subrect_pivoted(
                        command.tex,
                        command.subrect,
                        start_x + command.x * draw_scale,
                        start_y + command.y * draw_scale,
                        command.scale * draw_scale,
                        0.0,
                        pivot,
                    )
                }
            }
        }
        self.commands.clear();
    }
}

fn draw_subrect_pivoted(
    tex: Texture2D,
    subrect: SubRect,
    x: f32,
    y: f32,
    scale: f32,
    rotation: f32,
    pivot: Pivot,
) {
    let InternalGlContext {
        quad_context: ctx, ..
    } = unsafe { get_internal_gl() };
    let (x, y) = match pivot {
        Pivot::Relative { rel_x, rel_y } => {
            (
                (x - subrect.width as f32 * scale * rel_x) * ctx.dpi_scale(),
                (y - subrect.height as f32 * scale * rel_y) * ctx.dpi_scale()
            )
        }
        Pivot::Absolute { rel_x, rel_y } => {
            (
                (x - scale * rel_x) * ctx.dpi_scale(),
                (y - scale * rel_y) * ctx.dpi_scale()
            )
        }
    };

    draw_texture_ex(
        tex,
        x, y,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(
                subrect.x as f32, subrect.y as f32,
                subrect.width as f32, subrect.height as f32,
            )),
            dest_size: Some([
                subrect.width as f32 * ctx.dpi_scale() * scale,
                subrect.height as f32 * ctx.dpi_scale() * scale
            ].into()),
            pivot: match pivot {
                Pivot::Relative { rel_x, rel_y } => {
                    Some([rel_x, rel_y].into())
                }
                Pivot::Absolute { rel_x, rel_y } => {
                    Some([subrect.x as f32 + rel_x, subrect.y as f32 + rel_y].into())
                }
            },
            rotation,
            ..Default::default()
        },
    );
}

fn draw_subrect(tex: Texture2D, subrect: SubRect, x: f32, y: f32, scale: f32) {
    draw_subrect_pivoted(
        tex,
        subrect,
        x,
        y,
        scale,
        0.0,
        Pivot::Absolute { rel_x: 0.0, rel_y: 0.0 },
    );
}