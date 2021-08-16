use crate::core_subsystems::types::{GlobalContext, MenuScreen, Fraction};
use crate::components::{MenuScreenElement, MenuBackgroundTag, UiRect, SignalButton, PlayGameSignal, ExitGameSignal, Glyph, ChoosePlayerFractionSignal, GoToMainMenuSignal, NumberTag, BudgetTitleTag, BudgetDigitTag, FinishPlayerLandingSignal, SelectionTag};

pub fn create_main_menu_screen(ctx: &GlobalContext) {
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        MenuBackgroundTag,
        UiRect {
            top_left: (20, 13),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 21, 24)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        SignalButton {
            signal_to_send: PlayGameSignal,
            glyph_sub_rect: ctx.atlas_definition.play_button_subrect
        },
        UiRect {
            top_left: (22, 15),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 23, 18)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        SignalButton {
            signal_to_send: ExitGameSignal,
            glyph_sub_rect: ctx.atlas_definition.exit_button_subrect
        },
        UiRect {
            top_left: (22, 19),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 23, 22)
        }
    ));
}

pub fn create_choose_fraction_screen(ctx: &GlobalContext) {
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        Glyph { glyph_sub_rect: ctx.atlas_definition.choose_your_side_title_subrect},
        UiRect {
            top_left: (12, 11),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 13, 12)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        MenuBackgroundTag,
        UiRect {
            top_left: (12, 13),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 13, 24)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: ChoosePlayerFractionSignal{ fraction: Fraction::Red },
            glyph_sub_rect: ctx.atlas_definition.red_button_subrect
        },
        UiRect {
            top_left: (22 - 8, 15),
            bottom_right: (37 - 8, 18)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: ChoosePlayerFractionSignal{ fraction: Fraction::Blue },
            glyph_sub_rect: ctx.atlas_definition.blue_button_subrect
        },
        UiRect {
            top_left: (22 + 8, 15),
            bottom_right: (37 + 8, 18)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: GoToMainMenuSignal,
            glyph_sub_rect: ctx.atlas_definition.back_button_subrect
        },
        UiRect {
            top_left: (22, 19),
            bottom_right: (37, 22)
        }
    ));
}

pub fn create_player_landing_screen(ctx: &GlobalContext) {
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        Glyph { glyph_sub_rect: ctx.atlas_definition.landing_title_subrect},
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2, 36),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2, 40)
        }
    ));
    ctx.world.borrow_mut().spawn((
        BudgetTitleTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        Glyph { glyph_sub_rect: ctx.atlas_definition.red_budget_title_subrect},
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2, 40),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2, 41)
        }
    ));
    for i in 0..12 {
        ctx.world.borrow_mut().spawn((
            BudgetDigitTag,
            MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
            NumberTag((11 - i) as u32),
            Glyph { glyph_sub_rect: ctx.atlas_definition.red_digit_glyph_subrects[0]},
            UiRect {
                top_left: (ctx.tilemap.borrow().w as i32 / 2 - 6 + i, 40),
                bottom_right: (ctx.tilemap.borrow().w as i32 / 2 - 6 + i, 45)
            }
        ));
    }
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: FinishPlayerLandingSignal,
            glyph_sub_rect: ctx.atlas_definition.huge_axe_icon,

        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2 - 15, ctx.tilemap.borrow().h as i32),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2 - 12, ctx.tilemap.borrow().h as i32 + 3)
        }
    ));
    ctx.world.borrow_mut().spawn((
        SelectionTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: FinishPlayerLandingSignal,
            glyph_sub_rect: ctx.atlas_definition.small_axe_icon,

        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2 - 11, ctx.tilemap.borrow().h as i32),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2 - 8, ctx.tilemap.borrow().h as i32 + 3)
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: FinishPlayerLandingSignal,
            glyph_sub_rect: ctx.atlas_definition.ready_button_subrect,
            
        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2 + 7, ctx.tilemap.borrow().h as i32),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2 + 15, ctx.tilemap.borrow().h as i32 + 3)
        }
    ));
}