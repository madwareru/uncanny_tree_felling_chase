use super::atlas_serialization::OrcSprite;
use crate::components::VisualLookDirection;

#[derive(Copy, Clone)]
pub enum AnimationState {
    Idle,
    Hitting
}

#[derive(Copy, Clone)]
pub enum OrcAnimation {
    SmallRed,
    HugeRed,
    SmallBlue,
    HugeBlue
}

pub enum LookFlipsSpriteHorizontally { No, Yes }

/// each 10 ms we would be doing our tick
pub const FIXED_TIME_STEP_TICK_LENGTH_MS: f32 = 10.0;

/// we are playing back animation with 25 fps (which is 40 ms or 4 ticks in our case)
pub const TICKS_PER_FRAME_ANIM_SPRITES: usize = 4;

pub fn acquire_orc_animation_info(
    anim: &OrcAnimation,
    state: &AnimationState,
    direction: &VisualLookDirection
) -> &'static (LookFlipsSpriteHorizontally, &'static &'static [OrcSprite]) {
    match (anim, state) {
        (OrcAnimation::SmallRed, AnimationState::Idle) => &RED_ORC_SMALL_IDLE_SEQUENCES[*direction as usize],
        (OrcAnimation::SmallRed, AnimationState::Hitting) => &RED_ORC_SMALL_HIT_SEQUENCES[*direction as usize],
        (OrcAnimation::HugeRed, AnimationState::Idle) => &RED_ORC_HUGE_IDLE_SEQUENCES[*direction as usize],
        (OrcAnimation::HugeRed, AnimationState::Hitting) => &RED_ORC_HUGE_HIT_SEQUENCES[*direction as usize],
        (OrcAnimation::SmallBlue, AnimationState::Idle) => &BLUE_ORC_SMALL_IDLE_SEQUENCES[*direction as usize],
        (OrcAnimation::SmallBlue, AnimationState::Hitting) => &BLUE_ORC_SMALL_HIT_SEQUENCES[*direction as usize],
        (OrcAnimation::HugeBlue, AnimationState::Idle) => &BLUE_ORC_HUGE_IDLE_SEQUENCES[*direction as usize],
        (OrcAnimation::HugeBlue, AnimationState::Hitting) => &BLUE_ORC_HUGE_HIT_SEQUENCES[*direction as usize],
    }
}

pub const RED_ORC_SMALL_HIT_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &RED_ORC_SMALL_HIT), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_SMALL_HIT), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_SMALL_HIT_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &RED_ORC_SMALL_HIT_BACK), // LookDirection::NorthWest
];

pub const BLUE_ORC_SMALL_HIT_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_SMALL_HIT), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_SMALL_HIT), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_SMALL_HIT_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_SMALL_HIT_BACK), // LookDirection::NorthWest
];

pub const RED_ORC_HUGE_HIT_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &RED_ORC_HUGE_HIT), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_HUGE_HIT), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_HUGE_HIT_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &RED_ORC_HUGE_HIT_BACK), // LookDirection::NorthWest
];

pub const BLUE_ORC_HUGE_HIT_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_HUGE_HIT), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_HUGE_HIT), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_HUGE_HIT_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_HUGE_HIT_BACK), // LookDirection::NorthWest
];

pub const RED_ORC_SMALL_IDLE_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &RED_ORC_SMALL_IDLE), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_SMALL_IDLE), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_SMALL_IDLE_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &RED_ORC_SMALL_IDLE_BACK), // LookDirection::NorthWest
];

pub const BLUE_ORC_SMALL_IDLE_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_SMALL_IDLE), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_SMALL_IDLE), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_SMALL_IDLE_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_SMALL_IDLE_BACK), // LookDirection::NorthWest
];

pub const RED_ORC_HUGE_IDLE_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &RED_ORC_HUGE_IDLE), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_HUGE_IDLE), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &RED_ORC_HUGE_IDLE_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &RED_ORC_HUGE_IDLE_BACK), // LookDirection::NorthWest
];

pub const BLUE_ORC_HUGE_IDLE_SEQUENCES: &[(LookFlipsSpriteHorizontally, &&[OrcSprite])] = &[
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_HUGE_IDLE), // LookDirection::SouthEast
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_HUGE_IDLE), // LookDirection::SouthWest
    (LookFlipsSpriteHorizontally::Yes, &BLUE_ORC_HUGE_IDLE_BACK), // LookDirection::NorthEast
    (LookFlipsSpriteHorizontally::No, &BLUE_ORC_HUGE_IDLE_BACK), // LookDirection::NorthWest
];

macro_rules! sequence_const{
    { const $name:ident: &[$type:ident] = $id:ident ++ [$($lit:literal),*] } => {
        paste::paste! {
            const $name: &[$type] = &[
                $($type :: [<$id $lit>]),*
            ];
        }
    }
}

sequence_const! {
    const RED_ORC_SMALL_HIT: &[OrcSprite] = RedOrcSmall_Hit_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const RED_ORC_HUGE_HIT: &[OrcSprite] = RedOrcHuge_Hit_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const RED_ORC_SMALL_HIT_BACK: &[OrcSprite] = RedOrcSmall_HitBack_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const RED_ORC_HUGE_HIT_BACK: &[OrcSprite] = RedOrcSmall_HitBack_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const BLUE_ORC_SMALL_HIT: &[OrcSprite] = BlueOrcSmall_Hit_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const BLUE_ORC_HUGE_HIT: &[OrcSprite] = BlueOrcHuge_Hit_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const BLUE_ORC_SMALL_HIT_BACK: &[OrcSprite] = BlueOrcSmall_HitBack_ ++ [0, 1, 2, 3, 4, 5, 6]
}

sequence_const! {
    const BLUE_ORC_HUGE_HIT_BACK: &[OrcSprite] = BlueOrcSmall_HitBack_ ++ [0, 1, 2, 3, 4, 5, 6]
}

const RED_ORC_SMALL_IDLE: &[OrcSprite] = &[OrcSprite::RedOrcSmall_Idle, OrcSprite::RedOrcSmall_Idle];
const RED_ORC_SMALL_IDLE_BACK: &[OrcSprite] = &[OrcSprite::RedOrcSmall_IdleBack, OrcSprite::RedOrcSmall_IdleBack];
const BLUE_ORC_SMALL_IDLE: &[OrcSprite] = &[OrcSprite::BlueOrcSmall_Idle, OrcSprite::BlueOrcSmall_Idle];
const BLUE_ORC_SMALL_IDLE_BACK: &[OrcSprite] = &[OrcSprite::BlueOrcSmall_IdleBack, OrcSprite::BlueOrcSmall_IdleBack];
const RED_ORC_HUGE_IDLE: &[OrcSprite] = &[OrcSprite::RedOrcHuge_Idle, OrcSprite::RedOrcHuge_Idle];
const RED_ORC_HUGE_IDLE_BACK: &[OrcSprite] = &[OrcSprite::RedOrcHuge_IdleBack, OrcSprite::RedOrcHuge_IdleBack];
const BLUE_ORC_HUGE_IDLE: &[OrcSprite] = &[OrcSprite::BlueOrcHuge_Idle, OrcSprite::BlueOrcHuge_Idle];
const BLUE_ORC_HUGE_IDLE_BACK: &[OrcSprite] = &[OrcSprite::BlueOrcHuge_IdleBack, OrcSprite::BlueOrcHuge_IdleBack];