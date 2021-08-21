use crate::core_subsystems::types::{GlobalContext, GameState, BattleState, Fraction};
use std::cell::Ref;
use std::borrow::Borrow;
use crate::core_subsystems::units_serialization::OrcType;
use crate::core_subsystems::peek_utils::peek_tile_ext;
use crate::core_subsystems::player_landing_info::MapFieldOccupation;
use macroquad::input::{MouseButton, is_mouse_button_down};
use crate::components::{Position, Direction, Orc, SignalTag, SpareBudgetSignal, GrowBudgetSignal, Animator, VisualLookDirection};
use std::ops::DerefMut;
use crate::core_subsystems::animation_configuration::{OrcAnimation, AnimationState};

pub fn system(ctx: &GlobalContext) {
    if let GameState::Battle {
        internal_state: BattleState::PlayerLanding {
            budget,
            current_minion_is_big
        },
        fraction
    } = *ctx.game_state.borrow() {
        let (peek_x, peek_y) = peek_tile_ext(ctx, 0.5);

        if peek_x < 2 ||
            peek_y < 2 ||
            peek_x >= (ctx.tilemap.borrow().w * 2) as i32 - 2 ||
            peek_y >= (ctx.tilemap.borrow().h * 2) as i32 - 2
        {
            return;
        }

        let peek_x = peek_x as usize;
        let peek_y = peek_y as usize;

        let current_occupation = {
            ctx.landing_occupation_data
                .borrow()
                .get(peek_x, peek_y)
        };

        if current_occupation == MapFieldOccupation::Blocked {
            return;
        }

        match current_occupation {
            MapFieldOccupation::Unoccupied
            if is_mouse_button_down(MouseButton::Left) => {
                let config = if current_minion_is_big {
                    ctx.units_config.orcs[&OrcType::HugeOrc]
                } else {
                    ctx.units_config.orcs[&OrcType::SmallOrc]
                };
                if budget >= config.price {
                    ctx.world.borrow_mut().spawn(
                        (
                            SignalTag,
                            SpareBudgetSignal(config.price)
                        )
                    );
                    let spawned = ctx.world.borrow_mut().spawn(
                        (
                            Position {
                                x: (peek_x * ctx.atlas_scheme.tile_width / 2 +
                                    ctx.atlas_scheme.tile_width / 4) as i32,
                                y: (peek_y * ctx.atlas_scheme.tile_height / 2 +
                                    ctx.atlas_scheme.tile_height / 4) as i32,
                            },
                            Direction {
                                dir_x: config.movement_speed,
                                dir_y: 0,
                            },
                            Orc {
                                is_huge: current_minion_is_big,
                                fraction,
                            },
                            VisualLookDirection::SouthEast,
                            Animator {
                                animation: match (current_minion_is_big, fraction) {
                                    (false, Fraction::Red) => OrcAnimation::SmallRed,
                                    (true, Fraction::Red) => OrcAnimation::HugeRed,
                                    (false, Fraction::Blue) => OrcAnimation::SmallBlue,
                                    (true, Fraction::Blue) => OrcAnimation::HugeBlue
                                },
                                state: AnimationState::Idle,
                                current_frame: 0,
                                current_ticks: 0
                            }
                        )
                    );
                    *ctx.landing_occupation_data.borrow_mut()
                        .take_mut(peek_x, peek_y) = MapFieldOccupation::MinionLanded {
                        is_big: current_minion_is_big,
                        entity: spawned,
                    };
                }
            }
            MapFieldOccupation::MinionLanded { is_big, entity,  }
            if is_mouse_button_down(MouseButton::Right) => {
                let price = if is_big {
                    ctx.units_config.orcs[&OrcType::HugeOrc].price
                } else {
                    ctx.units_config.orcs[&OrcType::SmallOrc].price
                };
                ctx.enqueue_to_remove(entity);
                *ctx.landing_occupation_data.borrow_mut()
                    .take_mut(peek_x, peek_y) = MapFieldOccupation::Unoccupied;
                ctx.world.borrow_mut().spawn(
                    (
                        SignalTag,
                        GrowBudgetSignal(price)
                    )
                );
            }
            _ => {}
        }
    }
}