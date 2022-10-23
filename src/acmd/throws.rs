use super::*;

pub fn install() {
    install_acmd_scripts!(
        game_catch,
        game_catchdash,
        game_catchturn,
        game_catchattack,
        effect_catchattack,
        effect_catchwait
    );
}

#[acmd_script(agent = "samus", script = "game_catch" , category = ACMD_GAME)]
unsafe fn game_catch(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 11.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ false);
    }
}

#[acmd_script(agent = "samus", script = "game_catchdash" , category = ACMD_GAME)]
unsafe fn game_catchdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 16.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("throw"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ false);
    }
}

#[acmd_script(agent = "samus", script = "game_catchturn" , category = ACMD_GAME)]
unsafe fn game_catchturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 17.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ true);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {    
        CATCH(fighter, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 7.5, -18.0, Some(0.0), Some(7.5), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(boma, /*CanCatchRebound*/ false);
    }
}

#[acmd_script(agent = "samus", script = "game_catchattack" , category = ACMD_GAME)]
unsafe fn game_catchattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 361, 100, 30, 0, 7.2, 0.0, 8.0, 10.0, None, None, None, 2.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(boma, true, false);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script(agent = "samus", script = "effect_catchattack" , category = ACMD_EFFECT)]
unsafe fn effect_catchattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 6, 17.5, -3, 15, -20, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, 1.9, 0.05, 2.7); // Purple effects
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("handl"), 0, 0 , 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0); // Purple effects
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("handl"), 0, 0 , 0, 0, 0, 0, 0.12, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0); // Purple effects
    }
}

#[acmd_script(agent = "samus", script = "effect_catchwait" , category = ACMD_EFFECT)]
unsafe fn effect_catchwait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("handl"), 0, 0 , 0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0); // Purple effects
        EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("handl"), 0, 0 , 0, 0, 0, 0, 0.12, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0); // Purple effects
    }
}