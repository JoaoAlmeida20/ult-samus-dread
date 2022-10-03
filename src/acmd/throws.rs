use super::*;

pub fn install() {
    install_acmd_scripts!(
        game_catch,
        game_catchdash,
        game_catchturn
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