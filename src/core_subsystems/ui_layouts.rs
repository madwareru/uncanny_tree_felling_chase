use crate::core_subsystems::types::{GlobalContext, MenuScreen, Fraction};
use crate::components::{MenuScreenElement, MenuBackgroundTag, UiRect, SignalButton, PlayGameSignal, ExitGameSignal, Glyph, ChoosePlayerFractionSignal, GoToMainMenuSignal, NumberTag, BudgetTitleTag, BudgetDigitTag, FinishPlayerLandingSignal, SelectionTag, ChooseUnitTypeDuringLanding, ToggleButtonTag};
use crate::core_subsystems::atlas_serialization::{UiTile, RED_DIGIT_GLYPH_TILES};

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
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::PlayButtonLabel).unwrap()
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
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ExitButtonLabel).unwrap()
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
        Glyph { rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ChooseYourSideTitle).unwrap()},
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
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::RedFractionButtonLabel).unwrap()
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
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::BlueFractionButtonLabel).unwrap()
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
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::BackButtonLabel).unwrap()
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
        Glyph { rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::LandingTitle).unwrap()},
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2, 36),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2, 40)
        }
    ));
    ctx.world.borrow_mut().spawn((
        BudgetTitleTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        Glyph { rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::RedBudgetTitle).unwrap()},
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2, 40),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2, 41)
        }
    ));
    for i in 0..12 {
        let handle = ctx.ui_atlas.acquire_handle(
            &RED_DIGIT_GLYPH_TILES[0]
        ).unwrap();

        ctx.world.borrow_mut().spawn((
            BudgetDigitTag,
            MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
            NumberTag((11 - i) as u32),
            Glyph { rect_handle: handle},
            UiRect {
                top_left: (ctx.tilemap.borrow().w as i32 / 2 - 6 + i, 40),
                bottom_right: (ctx.tilemap.borrow().w as i32 / 2 - 6 + i, 45)
            }
        ));
    }
    ctx.world.borrow_mut().spawn((
        ToggleButtonTag,
        NumberTag(1),
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: ChooseUnitTypeDuringLanding{ new_minion_is_big: true },
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::HugeAxeIcon).unwrap(),

        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2 - 15, ctx.tilemap.borrow().h as i32),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2 - 12, ctx.tilemap.borrow().h as i32 + 3)
        }
    ));
    ctx.world.borrow_mut().spawn((
        ToggleButtonTag,
        NumberTag(0),
        SelectionTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: ChooseUnitTypeDuringLanding{ new_minion_is_big: false },
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::SmallAxeIcon).unwrap(),

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
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ReadyButtonLabel).unwrap(),
            
        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 / 2 + 7, ctx.tilemap.borrow().h as i32),
            bottom_right: (ctx.tilemap.borrow().w as i32 / 2 + 15, ctx.tilemap.borrow().h as i32 + 3)
        }
    ));
}