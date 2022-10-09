use super::*;

pub mod samus {
    pub mod instance {
        // flags
            pub const MORPHBALL_STALL_USED : i32 = 0x0100;
            pub const FLASHSHIFT_USED : i32 = 0x0101;
            pub const SHINESPARK_USED : i32 = 0x0102;
            pub const SPEEDBOOST_ACTIVE : i32 = 0x0103;
            pub const IS_SWING: i32 = 0x0104;

        // ints
            pub const BOMB_COUNTER : i32 = 0x0100;
            pub const FLASHSHIFT_CHAIN_COUNT : i32 = 0x0101;
            pub const SHINESPARK_STORED_EFFECT_HANDLE : i32 = 0x0102;
            pub const SPEEDBOOST_STATUS : i32 = 0x0103;
            
        // floats
            pub const AIM_ANGLE : i32 = 0x0100;
            pub const FLASHSHIFT_COOLDOWN_TIMER : i32 = 0x0101;
            pub const FLASHSHIFT_CHAIN_TIMER : i32 = 0x0102;
            pub const SHINESPARK_TIMER : i32 = 0x0103;
            pub const GBEAM_ANGLE: i32 = 0x0104;
            pub const HANG_PREV_POS_X: i32 = 0x0105;
            pub const HANG_PREV_POS_Y: i32 = 0x0106;
            pub const SWING_SPEED_X: i32 = 0x0107;
    }
}

pub mod supermissile {
    pub mod instance {
        // floats
            pub const ANGLE : i32 = 0x0100;
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset,
        agent_reset
    );
}

#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    CustomVarManager::reset_var_module(fighter.battle_object, false);
    VarModule::reset(fighter.battle_object, VarModule::RESET_ALL);
}

#[agent_reset]
fn agent_reset(fighter: &mut L2CFighterBase) {
    CustomVarManager::reset_var_module(fighter.battle_object, false);
    VarModule::reset(fighter.battle_object, VarModule::RESET_ALL);
}
