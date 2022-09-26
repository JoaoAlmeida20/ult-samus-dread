use super::*;
use table_consts::*;

pub unsafe fn frame(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    speedboost_main(fighter, boma);
    speedboost_jets_effect(fighter, boma);
    shinespark_storage(fighter, boma);
    shinespark_air(boma);
    sparks_effect(fighter, boma);
    check_resets(boma);
}

pub unsafe fn speedboost_start(boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion = MotionModule::motion_kind(boma);

    if !VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
        VarModule::on_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE);
        
        let handle;
        // Different speed line effects if in morphball
        if ![hash40("special_lw"),
        hash40("special_air_lw"),
        hash40("special_air_lw_shinespark")].contains(&motion)  {
            handle = EffectModule::req_follow(boma, Hash40::new("sys_attack_speedline"), Hash40::new("top"), &Vector3f{x: -2.5, y: 6.5, z: 0.0}, &Vector3f{x: 0.0, y: 180.0, z: 0.0}, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        else {
            handle = EffectModule::req_follow(boma, Hash40::new("sys_attack_speedline"), Hash40::new("top"), &Vector3f{x: -2.5, y: 3.0, z: 0.0}, &Vector3f{x: 0.0, y: 180.0, z: 0.0}, 0.9, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        EffectModule::set_rate_last(boma, 0.4);
        EffectModule::set_rgb(boma, handle, 0.2, 0.4, 10.0); // Blue
    }
}

pub unsafe fn speedboost_end(boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion = MotionModule::motion_kind(boma);
    let situation = StatusModule::situation_kind(boma);
    let frame = MotionModule::frame(boma);

    VarModule::off_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE);
    JostleModule::set_status(boma, true);

    // If samus is in morphball, reset the status to reset the speed params to regular values
    if [hash40("special_lw"),
    hash40("special_air_lw")].contains(&motion) {
        if situation == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, true);
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), frame, 1.0, 1.0);
        }
        else if situation == *SITUATION_KIND_AIR {
            StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw"), frame, 1.0, 1.0);
        }
    }
}

unsafe fn speedboost_main(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let status = StatusModule::status_kind(boma);
    let motion_frame = MotionModule::frame(boma);
    let stick_y = ControlModule::get_stick_y(boma);

    if *FIGHTER_STATUS_KIND_RUN == status
    && motion_frame > vl::param_speedboost::charge_frame
    && !VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
            speedboost_start(boma);
            PLAY_SE_REMAIN(fighter, Hash40::new("se_samus_special_n04"));
    }

    if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
        let speedboost_speed_max = vl::param_speedboost::speed_max;
        let run_speed_mul = speedboost_speed_max / WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
        if status != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            sv_kinetic_energy!(
                set_speed_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                run_speed_mul
            );
        }

        JostleModule::set_status(boma, false);

        // Allow crouch during run in speedboost so you can store shinespark
        if [*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE].contains(&status)
        && stick_y < WorkModule::get_param_float(boma, hash40("common"), hash40("squat_stick_y")) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SQUAT, false);
        }
    }
    else {
        if status != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
            sv_kinetic_energy!(
                set_speed_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                1.0
            );
        }
    }
}

unsafe fn speedboost_jets_effect(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let status = StatusModule::status_kind(boma);
    let motion = MotionModule::motion_kind(boma);

    // EFFECT_FOLLOW effects get killed on status change so we need to recall the jets effect on status changes
    if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
        // Spawn jets if in new status
        if status != VarModule::get_int(object, vars::samus::instance::SPEEDBOOST_STATUS)
        && ![hash40("special_lw"), hash40("special_air_lw")].contains(&motion) {
            EFFECT_FOLLOW(fighter, Hash40::new("samus_jump_jet"), Hash40::new("bust"), 0, 0, 0, -90.046, -90, 0, 1, true);
            EffectModule::set_rate_last(boma, 0.05);
            VarModule::set_int(object, vars::samus::instance::SPEEDBOOST_STATUS, status);
        }
        // Kill jets if morphball
        if VarModule::get_int(object, vars::samus::instance::SPEEDBOOST_STATUS) != -1
        && [hash40("special_lw"), hash40("special_air_lw")].contains(&motion) {
            EffectModule::kill_kind(boma, Hash40::new("samus_jump_jet"), false, false);
            VarModule::set_int(object, vars::samus::instance::SPEEDBOOST_STATUS, -1);
        }
    }
    // Kill jets if speedboost over
    else if !VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE)
    && VarModule::get_int(object, vars::samus::instance::SPEEDBOOST_STATUS) != -1 {
        EffectModule::kill_kind(boma, Hash40::new("samus_jump_jet"), false, false);
        VarModule::set_int(object, vars::samus::instance::SPEEDBOOST_STATUS, -1);
    }
}

// Shinespark storage
unsafe fn shinespark_storage(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let status = StatusModule::status_kind(boma);

    // Decrement shinespark timer
    if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
        VarModule::sub_float(object, vars::samus::instance::SHINESPARK_TIMER, 1.0);
    }

    // Begin timer of 5 seconds and glow purple for storing shinespark with crouch
    if *FIGHTER_STATUS_KIND_SQUAT_WAIT == status
    && VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
        VarModule::set_float(object, vars::samus::instance::SHINESPARK_TIMER, vl::param_shinespark::storage_duration);
        VarModule::off_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE);
        PLAY_SE(fighter, Hash40::new("se_samus_escape_ex"));
        let cbm_t_vec1 = Vector4f{ /* Red */ x: 0.85, /* Green */ y: 0.85, /* Blue */ z: 0.85, /* Alpha */ w: 0.015};
        let cbm_t_vec2 = Vector4f{ /* Red */ x: 0.75, /* Green */ y: 0.25, /* Blue */ z: 0.925, /* Alpha */ w: 0.6};
        ColorBlendModule::set_main_color(boma, /* Brightness */ &cbm_t_vec1, /* Diffuse */ &cbm_t_vec2, 0.21, 2.2, 3, /* Display Color */ true);
    }

    // Shinespark stored purple aura
    if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0
    && VarModule::get_int(object, vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE) == -1 {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bust"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 2.0, true, 0, 0, 0, 0, 0, true, true) as u32;
        LAST_EFFECT_SET_COLOR(fighter, 5.15, 0.15, 1.0); // Purple effects
        VarModule::set_int(object, vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE, handle as i32);
    }
    // Kill purple aura
    else if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) == 0.0
    && VarModule::get_int(object, vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE) != -1 {
        let handle = VarModule::get_int(object, vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE) as u32;
        EffectModule::kill(boma, handle, false, false);
        VarModule::set_int(object, vars::samus::instance::SHINESPARK_STORED_EFFECT_HANDLE, -1);
    }
}

// Shinespark air
unsafe fn shinespark_air(boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let status = StatusModule::status_kind(boma);
    let is_button_on_special_all = 
        ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
        || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
        || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW2);

    if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0
    && is_button_on_special_all 
    && status == *FIGHTER_STATUS_KIND_ATTACK_AIR
    && motion_frame <= 6.0 {
        MotionModule::change_motion(boma, Hash40::new("attack_dash"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn sparks_effect(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion = MotionModule::motion_kind(boma);

    // Speedboost and shinespark stored random electric sparks
    if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE)
    || VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
        let rng = app::sv_math::rand(hash40("fighter"), 10);
        if rng == 0 {
            let rng2 = app::sv_math::rand(hash40("fighter"), 3);
            let morphball_offset;
            if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion) {
                morphball_offset = 6.0;
            }
            else {
                morphball_offset = 0.0;
            }
            if rng2 == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 14.7 - morphball_offset, 4.3, 0, 0, 0, 0.12, true);
                if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                    LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
                }
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0 - morphball_offset, 1.0, 0, 0, 0, 0.9, true);
            }
            else if rng2 == 1 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 3.5 - morphball_offset, -6.1, 0, 0, 0, 0.09, true);
                if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                    LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
                }
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0 - morphball_offset, 1.0, 0, 0, 0, 0.9, true);
            }
            else if rng2 == 2 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 8.4 - morphball_offset, 0.2, 0, 0, 0, 0.16, true);
                if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                    LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
                }
                LAST_EFFECT_SET_RATE(fighter, 3.0);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("top"), 0.0, 12.0 - morphball_offset, 1.0, 0, 0, 0, 0.9, true);
            }
            if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0 {
                LAST_EFFECT_SET_COLOR(fighter, 2.15, 0.15, 1.0); // Purple effects
            }
        }
    }
}

unsafe fn check_resets(boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let speedboost_speed_max = vl::param_speedboost::speed_max;
    let frame = MotionModule::frame(boma);
    let motion = MotionModule::motion_kind(boma);
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let lr = PostureModule::lr(boma);

    let min_speed_condition = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs() > 0.8 * speedboost_speed_max;
    let is_touch_wall = 
        GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32)
        || GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);

    let ground_conditions =
        [*FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_DASH,
        *FIGHTER_STATUS_KIND_RUN,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_WALL_JUMP].contains(&status);

    let run_brake_condition = 
        status == *FIGHTER_STATUS_KIND_RUN_BRAKE
        && frame < 6.0;

    let jumpsquat_condition =
        status == *FIGHTER_STATUS_KIND_JUMP_SQUAT
        && stick_x * lr > 0.75;

    let air_conditions =
        [*FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status)
        && situation == *SITUATION_KIND_AIR
        && (min_speed_condition
            || is_touch_wall);

    let ground_morphball_conditions =
        [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&status)
        && situation == *SITUATION_KIND_GROUND
        && !is_touch_wall
        && (frame <= 20.0
            || min_speed_condition);

    // Check if not in valid condition for having speedboost
    if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE)
    && !(ground_conditions
        || run_brake_condition
        || jumpsquat_condition
        || air_conditions
        || ground_morphball_conditions) {
        speedboost_end(boma);
    }

    if ![hash40("attack_dash"),
    hash40("special_air_lw_shinespark")].contains(&motion) {
        VarModule::off_flag(object, vars::samus::instance::SHINESPARK_USED);
    }

    // Reset storage on death
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE].contains(&status) {
        VarModule::set_float(object, vars::samus::instance::SHINESPARK_TIMER, 0.0)
    }

    // Disable color if neither shinespark storage/usage are active
    if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) <= 0.0
    && !VarModule::is_flag(object, vars::samus::instance::SHINESPARK_USED) {
        ColorBlendModule::cancel_main_color(boma, 0);
    }
}