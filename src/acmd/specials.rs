use super::*;

pub fn install() {
    install_acmd_scripts!(
        special_lw_game,
        special_air_lw_game,
        special_lw_effect,
        special_lw_sound,
        special_game,
        special_effect,
        special_sound,
        special_expression,
        special_air_lw_shinespark_game,
        special_air_lw_shinespark_sound,
        special_air_lw_shinespark_effect,
    );
}

#[acmd_script( agent = "samus", script = "game_speciallw" , category = ACMD_GAME)]
unsafe fn special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let object = fighter.battle_object;

    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_sphere").hash as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);

        let bomb_burst_counter = VarModule::get_int(object, vars::samus::instance::BOMB_BURST_COUNTER);
        for n in 0..bomb_burst_counter {
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
            ArticleModule::shoot_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);    
        }
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
    }
}

#[acmd_script( agent = "samus", script = "game_specialairlw" , category = ACMD_GAME)]
unsafe fn special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);

    frame(lua_state, 11.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_sphere").hash as i64);
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        VarModule::on_flag(object, vars::samus::instance::MORPHBALL_STALL_USED);

        let bomb_burst_counter = VarModule::get_int(object, vars::samus::instance::BOMB_BURST_COUNTER);
        for n in 0..bomb_burst_counter {
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
            ArticleModule::shoot_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);    
        }
    }
    frame(lua_state, 44.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(boma, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
    }
}

#[acmd_script( agent = "samus", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT)]
unsafe fn special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_bomb_jump"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.48, true);
    }
    
}

#[acmd_script( agent = "samus", scripts = [ "sound_speciallw", "sound_specialairlw" ], category = ACMD_SOUND)]
unsafe fn special_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
    }
}

#[acmd_script( agent = "samus", script = "game_special" , category = ACMD_GAME)]
unsafe fn special_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.0);
    if is_excute(fighter) {    
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {    
        JostleModule::set_status(boma, true);
    }
}

#[acmd_script( agent = "samus", script = "effect_special" , category = ACMD_EFFECT)]
unsafe fn special_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EffectModule::req_follow(boma, Hash40::new("samus_cshot_hold"), Hash40::new("top"), &Vector3f{x: 0.0, y: 10.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 1.8, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        EffectModule::kill_kind(boma, Hash40::new("samus_cshot_hold"), false, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_attack_speedline"), Hash40::new("top"), &Vector3f{x: -5.0, y: 6.5, z: 0.0}, &Vector3f{x: 0.0, y: 180.0, z: 0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rate_last(boma, 0.4);
        EffectModule::set_rgb(boma, handle, 0.2, 0.4, 10.0); // Blue
    }
}

#[acmd_script( agent = "samus", script = "sound_special" , category = ACMD_SOUND)]
unsafe fn special_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);

    frame(lua_state, 3.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n02"));
        SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_samus_special_n02"), 0.75);
    }
}

#[acmd_script( agent = "samus", script = "expression_special" , category = ACMD_EXPRESSION)]
unsafe fn special_expression(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
}

#[acmd_script( agent = "samus", script = "game_specialairlw_shinespark" , category = ACMD_GAME)]
unsafe fn special_air_lw_shinespark_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let speed = vl::param_shinespark::ball_speed;
    let mut x_mul = 0.0;
    let mut y_mul = 0.0;

    frame(lua_state, 20.0);
    if is_excute(fighter) {
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::clear_speed_all(boma);
        MotionModule::set_rate(boma, 0.1);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        let stick_x = ControlModule::get_stick_x(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        PostureModule::set_stick_lr(boma, 0.0);
        PostureModule::update_rot_y_lr(boma);
        if stick_x.abs() > 0.93 {
            x_mul = stick_x.signum();
            y_mul = 0.0; 
        }
        else if stick_y.abs() > 0.93 {
            x_mul = 0.0;
            y_mul = stick_y.signum();
        }
        else {
            x_mul = stick_x.signum();
            y_mul = stick_y.signum();
        }
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy::set_accel(lua_state);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy::set_accel_x_add(lua_state);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy::set_accel_x_mul(lua_state);
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed, speed);
        sv_kinetic_energy::set_limit_speed(lua_state);
        let add_speed = Vector3f { x: speed * x_mul * PostureModule::lr(boma), y: speed * y_mul, z: 0.0 };
        KineticModule::add_speed(boma, &add_speed);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 120, 0, 80, 3.8, 0.0, 3.0, 0.0, None, None, None, 1.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        MotionModule::set_rate(boma, 1.6);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        let sub_speed = Vector3f { x: -speed * x_mul * PostureModule::lr(boma), y: -speed * y_mul, z: 0.0 };
        KineticModule::add_speed(boma, &sub_speed);
        AttackModule::clear_all(boma);
        MotionModule::set_rate(boma, 0.1);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        VarModule::off_flag(object, vars::samus::instance::SHINESPARK_USED);
        StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, false);
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 20.0, 1.0, 1.0);
    }
}

#[acmd_script( agent = "samus", script = "sound_specialairlw_shinespark" , category = ACMD_SOUND)]
unsafe fn special_air_lw_shinespark_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13eb2847e0));
        PLAY_SE(fighter, Hash40::new_raw(0x0e29ee1d3f));
        PLAY_SE_REMAIN(fighter, Hash40::new_raw(0x14614c32aa));
    }
}

#[acmd_script( agent = "samus", script = "effect_specialairlw_shinespark" , category = ACMD_EFFECT)]
unsafe fn special_air_lw_shinespark_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0 , 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0 , 0, 0, 0, 0, 0.6, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samus_screwattack"), Hash40::new("rot"), 0, 0 , 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samus_screwattack"), false, true);
    }
}