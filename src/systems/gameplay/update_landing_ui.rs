use crate::core_subsystems::types::{GlobalContext, GameState, Fraction, BattleState, MenuScreen};
use crate::components::{
    BudgetDigitTag,
    Glyph,
    BudgetTitleTag,
    NumberTag,
    SelectionTag,
    MenuScreenElement,
    ToggleButtonTag
};

pub fn system(ctx: &GlobalContext) {
    if let GameState::Battle { fraction, internal_state } = *(ctx.game_state.borrow()) {
        let (budget, current_minion_is_big) = match internal_state {
            BattleState::PlayerLanding {
                budget,
                current_minion_is_big
            } => (budget, current_minion_is_big),
            _ => (0, false)
        };

        for (_, (_, glyph)) in ctx.world.borrow_mut().query_mut::<(
            &BudgetTitleTag,
            &mut Glyph,
        )>() {
            glyph.glyph_sub_rect = match fraction {
                Fraction::Red => ctx.atlas_definition.red_budget_title_subrect,
                Fraction::Blue => ctx.atlas_definition.blue_budget_title_subrect,
            }
        }

        for (_, (_, digit, glyph)) in ctx.world.borrow_mut().query_mut::<(
            &BudgetDigitTag,
            &NumberTag,
            &mut Glyph,
        )>() {
            let digit_value = (budget as usize / 10usize.pow(digit.0)) % 10;
            glyph.glyph_sub_rect = match fraction {
                Fraction::Red => ctx.atlas_definition.red_digit_glyph_subrects[digit_value],
                Fraction::Blue => ctx.atlas_definition.blue_digit_glyph_subrects[digit_value],
            }
        }

        let (old_selection_entity, old_selection_size) = ctx.world.borrow()
            .query::<(&NumberTag, &SelectionTag, &MenuScreenElement, &ToggleButtonTag)>()
            .iter()
            .next()
            .map(|(e, (&tag, _, menu_screen_element, _))|
                (
                    e,
                    if menu_screen_element.menu_screen == MenuScreen::PlayerLanding && tag.0 == 0 {
                        false
                    } else {
                        true
                    }
                )
            )
            .unwrap();

        if old_selection_size != current_minion_is_big {
            ctx.world.borrow_mut().remove::<(SelectionTag,)>(old_selection_entity).unwrap();

            let new_selection_entity = ctx.world.borrow()
                .query::<(&NumberTag, &MenuScreenElement, &ToggleButtonTag)>()
                .iter()
                .find_map(|(e, (&tag, menu_screen_element, _))|
                    if menu_screen_element.menu_screen == MenuScreen::PlayerLanding &&
                        (tag.0 == 0) == !current_minion_is_big {
                        Some(e)
                    } else {
                        None
                    }
                )
                .unwrap();

            ctx.world.borrow_mut().insert(new_selection_entity, (SelectionTag,)).unwrap();
        }

    }
}