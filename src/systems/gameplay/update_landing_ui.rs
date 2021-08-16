use crate::core_subsystems::types::{GlobalContext, GameState, Fraction, BattleState};
use crate::components::{BudgetDigitTag, Glyph, BudgetTitleTag, NumberTag};

pub fn system(ctx: &GlobalContext) {
    if let GameState::Battle { fraction, internal_state } = *(ctx.game_state.borrow()) {
        let budget = match internal_state {
            BattleState::PlayerLanding { budget, .. } => budget,
            _ => 0
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
    }
}