use crate::core_subsystems::types::GlobalStorage;
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra};
use crate::core_subsystems::atlas_serialization::SubRect;

#[inline]
pub fn render_ui_background(
    global_storage: &mut GlobalStorage,
    left_tile: usize,
    right_tile: usize,
    top_tile: usize,
    bottom_tile: usize
) {
    render_box(
        global_storage,
        left_tile,
        right_tile,
        top_tile,
        bottom_tile,
        &[
            global_storage.atlas_definition.ui_background_box_3x3_0,
            global_storage.atlas_definition.ui_background_box_3x3_1,
            global_storage.atlas_definition.ui_background_box_3x3_2,

            global_storage.atlas_definition.ui_background_box_3x3_3,
            global_storage.atlas_definition.ui_background_box_3x3_4,
            global_storage.atlas_definition.ui_background_box_3x3_5,

            global_storage.atlas_definition.ui_background_box_3x3_6,
            global_storage.atlas_definition.ui_background_box_3x3_7,
            global_storage.atlas_definition.ui_background_box_3x3_8
        ]
    )
}

#[inline]
pub fn render_idle_button_background(
    global_storage: &mut GlobalStorage,
    left_tile: usize,
    right_tile: usize,
    top_tile: usize,
    bottom_tile: usize
) {
    render_box(
        global_storage,
        left_tile,
        right_tile,
        top_tile,
        bottom_tile,
        &[
            global_storage.atlas_definition.button_3x3_idle_0,
            global_storage.atlas_definition.button_3x3_idle_1,
            global_storage.atlas_definition.button_3x3_idle_2,

            global_storage.atlas_definition.button_3x3_idle_3,
            global_storage.atlas_definition.button_3x3_idle_4,
            global_storage.atlas_definition.button_3x3_idle_5,

            global_storage.atlas_definition.button_3x3_idle_6,
            global_storage.atlas_definition.button_3x3_idle_7,
            global_storage.atlas_definition.button_3x3_idle_8
        ]
    )
}

#[inline]
pub fn render_hover_button_background(
    global_storage: &mut GlobalStorage,
    left_tile: usize,
    right_tile: usize,
    top_tile: usize,
    bottom_tile: usize
) {
    render_box(
        global_storage,
        left_tile,
        right_tile,
        top_tile,
        bottom_tile,
        &[
            global_storage.atlas_definition.button_3x3_hover_0,
            global_storage.atlas_definition.button_3x3_hover_1,
            global_storage.atlas_definition.button_3x3_hover_2,

            global_storage.atlas_definition.button_3x3_hover_3,
            global_storage.atlas_definition.button_3x3_hover_4,
            global_storage.atlas_definition.button_3x3_hover_5,

            global_storage.atlas_definition.button_3x3_hover_6,
            global_storage.atlas_definition.button_3x3_hover_7,
            global_storage.atlas_definition.button_3x3_hover_8
        ]
    )
}

#[inline]
pub fn render_clicked_button_background(
    global_storage: &mut GlobalStorage,
    left_tile: usize,
    right_tile: usize,
    top_tile: usize,
    bottom_tile: usize
) {
    render_box(
        global_storage,
        left_tile,
        right_tile,
        top_tile,
        bottom_tile,
        &[
            global_storage.atlas_definition.button_3x3_clicked_0,
            global_storage.atlas_definition.button_3x3_clicked_1,
            global_storage.atlas_definition.button_3x3_clicked_2,

            global_storage.atlas_definition.button_3x3_clicked_3,
            global_storage.atlas_definition.button_3x3_clicked_4,
            global_storage.atlas_definition.button_3x3_clicked_5,

            global_storage.atlas_definition.button_3x3_clicked_6,
            global_storage.atlas_definition.button_3x3_clicked_7,
            global_storage.atlas_definition.button_3x3_clicked_8
        ]
    )
}

fn render_box(
    global_storage: &mut GlobalStorage,
    left_tile: usize,
    right_tile: usize,
    top_tile: usize,
    bottom_tile: usize,
    box_sub_tiles: &[SubRect]
) {
    for j in top_tile..=bottom_tile {
        for i in left_tile..=right_tile {
            let draw_command = DrawCommand {
                tex: global_storage.ui_atlas_texture,
                subrect: match j {
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
                },
                x: (global_storage.atlas_definition.tile_height * i) as f32,
                y: (global_storage.atlas_definition.tile_height * j) as f32,
                scale: 1.0,
                drawing_extra: DrawCommandExtra::Draw,
                sorting_layer: 2
            };
            global_storage.scene_compositor.enqueue(draw_command);
        }
    }
}