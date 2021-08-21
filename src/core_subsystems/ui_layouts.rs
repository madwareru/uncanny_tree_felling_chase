use crate::core_subsystems::types::{GlobalContext, MenuScreen, Fraction};
use crate::components::{MenuScreenElement, MenuBackgroundTag, UiRect, SignalButton, PlayGameSignal, ExitGameSignal, Glyph, ChoosePlayerFractionSignal, GoToMainMenuSignal, NumberTag, BudgetTitleTag, BudgetDigitTag, FinishPlayerLandingSignal, SelectionTag, ChooseUnitTypeDuringLanding, ToggleButtonTag, ReplayGameSignal, FadeAroundMouseTag, ClearAllUnitsSignal};
use crate::core_subsystems::atlas_serialization::{UiTile, RED_DIGIT_GLYPH_TILES};

pub fn create_main_menu_screen(ctx: &GlobalContext) {
    let x_center = ctx.tilemap.borrow().w as i32 / 2;
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        MenuBackgroundTag,
        UiRect {
            top_left: (x_center - 10, 13),
            bottom_right: (x_center + 10, 24),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        SignalButton {
            signal_to_send: PlayGameSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::PlayButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (x_center - 7, 15),
            bottom_right: (x_center + 7, 18),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::MainMenu },
        SignalButton {
            signal_to_send: ExitGameSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ExitButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (x_center - 7, 19),
            bottom_right: (x_center + 7, 22),
        }
    ));
}

pub fn create_choose_fraction_screen(ctx: &GlobalContext) {
    let bottom = ctx.tilemap.borrow().h as i32;
    let x_center = ctx.tilemap.borrow().w as i32 / 2;

    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        Glyph {
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ChooseYourSideTitle).unwrap(),
            transparency: 1.0,
        },
        UiRect {
            top_left: (x_center - 10, 11),
            bottom_right: (x_center + 10, 14),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        MenuBackgroundTag,
        UiRect {
            top_left: (x_center - 10, 10),
            bottom_right: (x_center + 10, bottom - 11),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: ChoosePlayerFractionSignal { fraction: Fraction::Red },
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::RedFractionButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (x_center - 8, 15),
            bottom_right: (x_center, 18),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: ChoosePlayerFractionSignal { fraction: Fraction::Blue },
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::BlueFractionButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (x_center + 1, 15),
            bottom_right: (x_center + 9, 18),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::FractionChoice },
        SignalButton {
            signal_to_send: GoToMainMenuSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::BackButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (x_center - 5, 19),
            bottom_right: (x_center + 6, 22),
        }
    ));
}

pub fn create_player_landing_screen(ctx: &GlobalContext) {
    let bottom = ctx.tilemap.borrow().h as i32;
    let x_center = ctx.tilemap.borrow().w as i32 / 2;

    ctx.world.borrow_mut().spawn((
        FadeAroundMouseTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        Glyph {
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::LandingTitle).unwrap(),
            transparency: 1.0,
        },
        UiRect {
            top_left: (0, 0),
            bottom_right: (ctx.tilemap.borrow().w as i32, ctx.tilemap.borrow().h as i32),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: GoToMainMenuSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::BackButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (1, bottom),
            bottom_right: (13, bottom + 3),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: ReplayGameSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ReplayButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (15, bottom),
            bottom_right: (27, bottom + 3),
        }
    ));
    ctx.world.borrow_mut().spawn((
        BudgetTitleTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        Glyph {
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::RedBudgetTitle).unwrap(),
            transparency: 1.0,
        },
        UiRect {
            top_left: (x_center, bottom - 1),
            bottom_right: (x_center, bottom + 1),
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
            Glyph { rect_handle: handle, transparency: 1.0 },
            UiRect {
                top_left: (x_center - 6 + i, bottom - 1),
                bottom_right: (x_center - 6 + i, bottom + 5),
            }
        ));
    }
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: ClearAllUnitsSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ClearIcon).unwrap(),

        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 - 28, bottom),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 25, bottom + 3),
        }
    ));
    ctx.world.borrow_mut().spawn((
        ToggleButtonTag,
        NumberTag(1),
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: ChooseUnitTypeDuringLanding { new_minion_is_big: true },
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::HugeAxeIcon).unwrap(),

        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 - 23, bottom),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 20, bottom + 3),
        }
    ));
    ctx.world.borrow_mut().spawn((
        ToggleButtonTag,
        NumberTag(0),
        SelectionTag,
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: ChooseUnitTypeDuringLanding { new_minion_is_big: false },
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::SmallAxeIcon).unwrap(),
        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 - 18, bottom),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 15, bottom + 3),
        }
    ));
    ctx.world.borrow_mut().spawn((
        MenuScreenElement { menu_screen: MenuScreen::PlayerLanding },
        SignalButton {
            signal_to_send: FinishPlayerLandingSignal,
            rect_handle: ctx.ui_atlas.acquire_handle(&UiTile::ReadyButtonLabel).unwrap(),
        },
        UiRect {
            top_left: (ctx.tilemap.borrow().w as i32 - 13, bottom),
            bottom_right: (ctx.tilemap.borrow().w as i32 - 2, bottom + 3),
        }
    ));
}