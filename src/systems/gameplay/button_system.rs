use crate::core_subsystems::types::{GlobalContext, MenuScreen};
use crate::components::{UiRect, SignalButton, PlayGameSignal, ExitGameSignal, MenuScreenElement, ChoosePlayerFractionSignal, GoToMainMenuSignal, PauseGameSignal, UnpauseGameSignal, ChooseUnitTypeDuringLanding, ReplayGameSignal, SignalCommand, FinishPlayerLandingSignal, ClearAllUnitsSignal};
use crate::core_subsystems::peek_utils::peek_tile;
use macroquad::input::{MouseButton, is_mouse_button_released};

pub fn system(ctx: &GlobalContext) {
    if let Some(menu_screen) = ctx.game_state.borrow().get_menu_screen() {
        handle_signal_button_clicks::<PlayGameSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<ExitGameSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<ChoosePlayerFractionSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<GoToMainMenuSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<PauseGameSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<UnpauseGameSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<ChooseUnitTypeDuringLanding>(ctx, menu_screen);
        handle_signal_button_clicks::<ReplayGameSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<FinishPlayerLandingSignal>(ctx, menu_screen);
        handle_signal_button_clicks::<ClearAllUnitsSignal>(ctx, menu_screen);

        fn handle_signal_button_clicks<TSignal: 'static + Copy + Clone + Send + Sync + Into<SignalCommand>>(
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
                    if is_mouse_button_released(MouseButton::Left) {
                        ctx.signal_command_buffer
                            .borrow_mut()
                            .push_back(signal_button.signal_to_send.into());
                    }
                }
            }
        }
    }
}