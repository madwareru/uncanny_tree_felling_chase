use crate::core_subsystems::types::{GlobalContext, MenuScreen};
use crate::systems::rendering::ui_shared::{
    render_ui_background,
    render_idle_button_background,
    render_clicked_button_background,
    render_hover_button_background,
    render_ui_selection
};
use crate::components::{UiRect, MenuBackgroundTag, SignalButton, PlayGameSignal, ExitGameSignal, MenuScreenElement, ChoosePlayerFractionSignal, GoToMainMenuSignal, PauseGameSignal, UnpauseGameSignal, ChooseUnitTypeDuringLanding, ReplayGameSignal, Glyph, SelectionTag, FinishPlayerLandingSignal};
use crate::core_subsystems::peek_utils::peek_tile;
use macroquad::input::{is_mouse_button_down, MouseButton};
use macro_tiler::atlas::rect_handle::{Having, DrawPivot, DrawSizeOverride};

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

        for (_, (_, menu_screen_element, rect)) in ctx.world.borrow().query::<(
            &SelectionTag,
            &MenuScreenElement,
            &UiRect
        )>().iter() {
            if menu_screen != menu_screen_element.menu_screen {
                continue;
            }
            render_ui_selection(ctx, rect);
        }

        for (_, (glyph, menu_screen_element, rect)) in ctx.world.borrow().query::<(
            &Glyph,
            &MenuScreenElement,
            &UiRect
        )>().iter() {
            if menu_screen != menu_screen_element.menu_screen {
                continue;
            }
            let draw_command = macro_tiler::atlas::draw_command::builder()
                .having(DrawPivot::Relative([0.5, 0.5].into()))
                .having(DrawSizeOverride::ScaledUniform(2.0))
                .build(
                    glyph.rect_handle,
                    ctx.atlas_scheme.tile_width as f32 * (0.5 + (rect.top_left.0 + rect.bottom_right.0) as f32 / 2.0),
                    ctx.atlas_scheme.tile_height as f32 * (0.5 + (rect.top_left.1 + rect.bottom_right.1) as f32 / 2.0)
                );
            ctx.scene_compositor.borrow_mut().enqueue(6, draw_command);
        }

        handle_signal_buttons::<PlayGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<ExitGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<ChoosePlayerFractionSignal>(ctx, menu_screen);
        handle_signal_buttons::<GoToMainMenuSignal>(ctx, menu_screen);
        handle_signal_buttons::<PauseGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<UnpauseGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<ChooseUnitTypeDuringLanding>(ctx, menu_screen);
        handle_signal_buttons::<ReplayGameSignal>(ctx, menu_screen);
        handle_signal_buttons::<FinishPlayerLandingSignal>(ctx, menu_screen);

        fn handle_signal_buttons<TSignal: 'static + Copy + Clone + Send + Sync>(
            ctx: &GlobalContext,
            menu_screen: MenuScreen,
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

                let draw_command = macro_tiler::atlas::draw_command::builder()
                    .having(DrawPivot::Relative([0.5, 0.5].into()))
                    .having(DrawSizeOverride::ScaledUniform(2.0))
                    .build(
                        signal_button.rect_handle,
                        ctx.atlas_scheme.tile_width as f32 * (0.5 + (rect.top_left.0 + rect.bottom_right.0) as f32 / 2.0),
                        ctx.atlas_scheme.tile_height as f32 * (0.5 + (rect.top_left.1 + rect.bottom_right.1) as f32 / 2.0)
                    );
                ctx.scene_compositor.borrow_mut().enqueue(5, draw_command);
            }
        }
    }
}