use super::*;

pub fn install() {
    install_acmd_scripts!(
        jump_front_mini_effect,
        jump_back_mini_effect,
        jump_front_spin_effect,
        jump_front_spin_sound
    );
}

#[acmd_script( agent = "samus", script = "effect_jumpfrontmini" , category = ACMD_EFFECT)]
unsafe fn jump_front_mini_effect(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

#[acmd_script( agent = "samus", script = "effect_jumpbackmini" , category = ACMD_EFFECT)]
unsafe fn jump_back_mini_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
       }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump"), Hash40::new("rot"), 0, -6, 0, 0, 180, 0, 0.6, true);
    }
}

#[acmd_script( agent = "samus", script = "effect_jumpfrontspin" , category = ACMD_EFFECT)]
unsafe fn jump_front_spin_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_jump_jet"), false, true);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_jump"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.65, true);
    }
}

#[acmd_script( agent = "samus", script = "sound_jumpfrontspin" , category = ACMD_SOUND)]
unsafe fn jump_front_spin_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;

    frame(lua_state, 7.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_samus_jump02"));
    }
}