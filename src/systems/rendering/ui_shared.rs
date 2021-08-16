use crate::core_subsystems::types::GlobalContext;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra};
use crate::core_subsystems::atlas_serialization::{SubRect};
use crate::components::UiRect;

pub fn render_ui_selection(ctx: &GlobalContext, ui_rect: &UiRect) {
    let box_sub_tiles = &[
        ctx.atlas_definition.ui_selection_3x3_0,
        ctx.atlas_definition.ui_selection_3x3_1,
        ctx.atlas_definition.ui_selection_3x3_2,

        ctx.atlas_definition.ui_selection_3x3_3,
        ctx.atlas_definition.ui_selection_3x3_4,
        ctx.atlas_definition.ui_selection_3x3_5,

        ctx.atlas_definition.ui_selection_3x3_6,
        ctx.atlas_definition.ui_selection_3x3_7,
        ctx.atlas_definition.ui_selection_3x3_8
    ];
    let &UiRect {
        top_left: (left_tile, top_tile),
        bottom_right: (right_tile, bottom_tile)
    } = ui_rect;
    for j in top_tile..=bottom_tile {
        for i in left_tile..=right_tile {
            let draw_command = DrawCommand {
                tex: ctx.ui_atlas_texture,
                subrect: choose_subrect(box_sub_tiles, left_tile, top_tile, right_tile, bottom_tile, j, i),
                x: (ctx.atlas_definition.tile_height as i32 * i) as f32,
                y: (ctx.atlas_definition.tile_height as i32 * j) as f32,
                scale: 1.0,
                drawing_extra: DrawCommandExtra::Draw,
                sorting_layer: 4
            };
            ctx.scene_compositor.borrow_mut().enqueue(draw_command);
        }
    }
}

#[inline]
pub fn render_ui_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
        &[
            ctx.atlas_definition.ui_background_box_3x3_0,
            ctx.atlas_definition.ui_background_box_3x3_1,
            ctx.atlas_definition.ui_background_box_3x3_2,

            ctx.atlas_definition.ui_background_box_3x3_3,
            ctx.atlas_definition.ui_background_box_3x3_4,
            ctx.atlas_definition.ui_background_box_3x3_5,

            ctx.atlas_definition.ui_background_box_3x3_6,
            ctx.atlas_definition.ui_background_box_3x3_7,
            ctx.atlas_definition.ui_background_box_3x3_8
        ]
    )
}

#[inline]
pub fn render_idle_button_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
        &[
            ctx.atlas_definition.button_3x3_idle_0,
            ctx.atlas_definition.button_3x3_idle_1,
            ctx.atlas_definition.button_3x3_idle_2,

            ctx.atlas_definition.button_3x3_idle_3,
            ctx.atlas_definition.button_3x3_idle_4,
            ctx.atlas_definition.button_3x3_idle_5,

            ctx.atlas_definition.button_3x3_idle_6,
            ctx.atlas_definition.button_3x3_idle_7,
            ctx.atlas_definition.button_3x3_idle_8
        ]
    )
}

#[inline]
pub fn render_hover_button_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
        &[
            ctx.atlas_definition.button_3x3_hover_0,
            ctx.atlas_definition.button_3x3_hover_1,
            ctx.atlas_definition.button_3x3_hover_2,

            ctx.atlas_definition.button_3x3_hover_3,
            ctx.atlas_definition.button_3x3_hover_4,
            ctx.atlas_definition.button_3x3_hover_5,

            ctx.atlas_definition.button_3x3_hover_6,
            ctx.atlas_definition.button_3x3_hover_7,
            ctx.atlas_definition.button_3x3_hover_8
        ]
    )
}

#[inline]
pub fn render_clicked_button_background(ctx: &GlobalContext, ui_rect: &UiRect) {
    render_box(ctx, ui_rect,
        &[
            ctx.atlas_definition.button_3x3_clicked_0,
            ctx.atlas_definition.button_3x3_clicked_1,
            ctx.atlas_definition.button_3x3_clicked_2,

            ctx.atlas_definition.button_3x3_clicked_3,
            ctx.atlas_definition.button_3x3_clicked_4,
            ctx.atlas_definition.button_3x3_clicked_5,

            ctx.atlas_definition.button_3x3_clicked_6,
            ctx.atlas_definition.button_3x3_clicked_7,
            ctx.atlas_definition.button_3x3_clicked_8
        ]
    )
}

fn render_box(ctx: &GlobalContext, ui_rect: &UiRect, box_sub_tiles: &[SubRect]) {
    let &UiRect {
        top_left: (left_tile, top_tile),
        bottom_right: (right_tile, bottom_tile)
    } = ui_rect;
    for j in top_tile..=bottom_tile {
        for i in left_tile..=right_tile {
            let draw_command = DrawCommand {
                tex: ctx.ui_atlas_texture,
                subrect: choose_subrect(box_sub_tiles, left_tile, top_tile, right_tile, bottom_tile, j, i),
                x: (ctx.atlas_definition.tile_height as i32 * i) as f32,
                y: (ctx.atlas_definition.tile_height as i32 * j) as f32,
                scale: 1.0,
                drawing_extra: DrawCommandExtra::Draw,
                sorting_layer: 3
            };
            ctx.scene_compositor.borrow_mut().enqueue(draw_command);
        }
    }
}

fn choose_subrect(box_sub_tiles: &[SubRect], left_tile: i32, top_tile: i32, right_tile: i32, bottom_tile: i32, j: i32, i: i32) -> SubRect {
    match j {
        jt if jt == top_tile => {
            match i {
                t if t == left_tile => {
                    box_sub_tiles[0]
                },
                t if t == right_tile => {
                    box_sub_tiles[2]
                },
                _ => {
                    box_sub_tiles[1]
                }
            }
        },
        jt if jt == bottom_tile => {
            match i {
                t if t == left_tile => {
                    box_sub_tiles[6]
                },
                t if t == right_tile => {
                    box_sub_tiles[8]
                },
                _ => {
                    box_sub_tiles[7]
                }
            }
        },
        _ => {
            match i {
                t if t == left_tile => {
                    box_sub_tiles[3]
                },
                t if t == right_tile => {
                    box_sub_tiles[5]
                },
                _ => {
                    box_sub_tiles[4]
                }
            }
        }
    }
}