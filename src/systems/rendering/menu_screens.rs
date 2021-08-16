use crate::core_subsystems::types::{GlobalContext, MenuScreen};
use crate::core_subsystems::rendering::{DrawCommand, DrawCommandExtra, Pivot};
use crate::systems::rendering::ui_shared::{render_ui_background, render_idle_button_background, render_clicked_button_background, render_hover_button_background};
use crate::components::{UiRect, MenuBackgroundTag, SignalButton, PlayGameSignal, ExitGameSignal, MenuScreenElement, ChoosePlayerFractionSignal, GoToMainMenuSignal, PauseGameSignal, UnpauseGameSignal, ChooseUnitTypeDuringLanding, ReplayGameSignal};
use crate::core_subsystems::peek_utils::peek_tile;
use macroquad::input::{is_mouse_button_down, MouseButton};

pub fn system(ctx: &GlobalContext) {
    if let Some(menu_screen) = ctx.game_state.borrow().get_menu_screen() {
        for (_, (_, menu_screen_element, rect)) in ctx.world.borrow().query::<(
            &MenuBackgroundTag,
            &MenuScreenElement,
            &UiRect
        )>().iter() {
            if menu_screen != menu_screen_element.menu_screen {
                continue;
            }
            render_ui_background(ctx, rect);
        }

        handle_signal_buttons::<PlayGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<ExitGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<ChoosePlayerFractionSignal>(ctx, menu_screen);
        handle_signal_buttons::<GoToMainMenuSignal>(ctx, menu_screen);
        handle_signal_buttons::<PauseGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<UnpauseGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<ChooseUnitTypeDuringLanding>(ctx, menu_screen);
        handle_signal_buttons::<ReplayGameSignal>(ctx, menu_screen);

        fn handle_signal_buttons<TSignal: 'static + Copy + Clone + Send + Sync>(
            ctx: &GlobalContext,
            menu_screen: MenuScreen
        ) {
            for (_, (menu_screen_element, signal_button, rect)) in ctx.world.borrow().query::<(
                &MenuScreenElement,
                &SignalButton<TSignal>,
                &UiRect
            )>().iter() {
                if menu_screen != menu_screen_element.menu_screen {
                    continue;
                }
                let peeked_tile = peek_tile(ctx);
                if peeked_tile.0 >= rect.top_left.0 && peeked_tile.0 <= rect.bottom_right.0 &&
                    peeked_tile.1 >= rect.top_left.1 && peeked_tile.1 <= rect.bottom_right.1 {
                    if is_mouse_button_down(MouseButton::Left) {
                        render_clicked_button_background(ctx, rect);
                    } else {
                        render_hover_button_background(ctx, rect);
                    }
                } else {
                    render_idle_button_background(ctx, rect);
                }
                ctx.scene_compositor.borrow_mut().enqueue(
                    DrawCommand {
                        tex: ctx.ui_atlas_texture,
                        subrect: signal_button.glyph_sub_rect,
                        x: ctx.atlas_definition.tile_width as f32 * (0.5 + (rect.top_left.0 + rect.bottom_right.0) as f32 / 2.0),
                        y: ctx.atlas_definition.tile_height as f32 * (0.5 + (rect.top_left.1 + rect.bottom_right.1) as f32 / 2.0),
                        scale: 2.0,
                        drawing_extra: DrawCommandExtra::DrawWithPivot {
                            pivot: Pivot::Relative {rel_x: 0.5, rel_y: 0.5}
                        },
                        sorting_layer: 4
                    }
                )
            }
        }
    }
}