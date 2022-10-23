use super::*;

pub fn install() {
    install_acmd_scripts!(
        samus_cshot_shot_game,
        samus_supermissile_ready_game,
        samus_supermissile_straight_game,
        samus_supermissile_straight_effect,
        samus_supermissile_sburst_game
    );
}

#[acmd_script( agent = "samus_cshot", script = "game_shoot" , category = ACMD_GAME)]
unsafe fn samus_cshot_shot_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        let charge = WorkModule::get_float(fighter.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
        if charge < 0.1 {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 0, 0, 0, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 28.0, 40, 50, 0, 46, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ENERGY);
            attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        }
    }
}

#[acmd_script( agent = "samus_supermissile", script = "game_ready" , category = ACMD_GAME)]
unsafe fn samus_supermissile_ready_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 85, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
		AttackModule::enable_safe_pos(boma);
	}
    
}

#[acmd_script( agent = "samus_supermissile", script = "game_straight" , category = ACMD_GAME)]
unsafe fn samus_supermissile_straight_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
        let angle = VarModule::get_float(fighter.battle_object, vars::supermissile::instance::ANGLE);
        // funny downward missile spike
        if angle < vl::param_supermissile::spike_min_angle {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 85, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 85, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        }
    }
    
}

#[acmd_script( agent = "samus_supermissile", script = "effect_straight" , category = ACMD_EFFECT)]
unsafe fn samus_supermissile_straight_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
        let angle = VarModule::get_float(fighter.battle_object, vars::supermissile::instance::ANGLE);
		EFFECT_FOLLOW(fighter, Hash40::new("samus_missile_straight"), Hash40::new("top"), 0, 0, -1, -angle, 0, 0, 1, true);
    }
    
}

#[acmd_script( agent = "samus_supermissile", script = "game_sburst" , category = ACMD_GAME)]
unsafe fn samus_supermissile_sburst_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

	if is_excute(fighter) {
		ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 75, 65, 0, 70, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x199c462b5d));
    }

}