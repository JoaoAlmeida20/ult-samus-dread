use super::*;
use table_consts::*;

pub unsafe fn frame(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let motion = MotionModule::motion_kind(boma);

    if [hash40("special_lw"),
    hash40("special_air_lw")].contains(&motion) {
        main(fighter, boma);
        bomb(fighter, boma);
        springball(fighter, boma);
        ballspark(fighter, boma);
    }
    else {
        roll_cancel(fighter, boma);
    }
}

unsafe fn main(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);
    let is_button_trigger_special_all = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL);

    // Freeze motion rate if x speed is 0 so that ball doesn't roll if you're standing still
    if 20.0 <= motion_frame
    && motion_frame < 40.0 
    && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) == 0.0 {
        MotionModule::set_rate(boma, 0.0);
    }
    else {
        MotionModule::set_rate(boma, 1.0);
    }

    // Disable jostle because its just a silly little ball
    if 11.0 <= motion_frame
    && motion_frame < 40.0 {
        JostleModule::set_status(boma, false);
    }
    else {
        JostleModule::set_status(boma, true);
    }

    // Exit morphball by pressing Special
    if is_button_trigger_special_all
    && 20.0 <= motion_frame
    && motion_frame < 40.0 {
        if situation == *SITUATION_KIND_GROUND {
            StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, true);
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 40.0, 1.0, 1.0);
        }
        else if situation == *SITUATION_KIND_AIR {
            StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw"), 40.0, 1.0, 1.0);
        }
        // Have samus facing in the direction of the stick on exit because the direction from before morphball was activated may be unexpected
        PostureModule::set_stick_lr(boma, 0.0);
        PostureModule::update_rot_y_lr(boma);
    }
    
    // Stay in morphball after a bomb jump
    else if motion_frame >= 12.0
    && [*FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_G,
    *FIGHTER_SAMUS_STATUS_KIND_BOMB_JUMP_A].contains(&status) {
            if situation == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, false);
            }
            else if situation == *SITUATION_KIND_AIR {
                StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, false);
            }
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 20.0, 1.0, 1.0);
    }
    // Loop before end of vanilla animation
    else if 38.0 <= motion_frame
    && motion_frame < 40.0 {
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 20.0, 1.0, 1.0);
    }
}

unsafe fn bomb(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let is_button_trigger_attack_all =
        ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW);
    let bomb_max_req = WorkModule::get_param_int(boma, hash40("param_special_lw"), hash40("bomb_max_req"));

    // Place bomb by pressing Attack
    if is_button_trigger_attack_all
    && motion_frame < 40.0
    && VarModule::get_int(object, vars::samus::instance::BOMB_COUNTER) < vl::param_speciallw::bomb_max_num_airtime
    && (ArticleModule::get_active_num(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB) as i32) < bomb_max_req {
        ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, false, -1);
        ArticleModule::shoot_exist(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_BOMB, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        VarModule::inc_int(object, vars::samus::instance::BOMB_COUNTER);
    }
}

unsafe fn springball(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let situation = StatusModule::situation_kind(boma);
    let is_button_trigger_jump = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_JUMP);
    let num_used_jumps = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let jump_count_max = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);

    // Allow jumping and double jumping in morphball
    if is_button_trigger_jump
    && 11.0 < motion_frame
    && motion_frame < 40.0 {
        let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
        let mini_jump_y = WorkModule::get_param_float(boma, hash40("mini_jump_y"), 0);
        let jump_speed = Vector3f{x: 0.0, y: (air_accel_y * (mini_jump_y / (0.5 * air_accel_y)).sqrt()), z: 0.0};

        if situation == *SITUATION_KIND_GROUND {
            StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            KineticModule::add_speed(boma, &jump_speed);
            StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
            PLAY_SE(fighter, Hash40::new("se_samus_jump03"));
            EFFECT_FOLLOW(fighter, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);

        }
        else if situation == *SITUATION_KIND_AIR
        && num_used_jumps < jump_count_max {
            let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
            KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::add_speed(boma, &jump_speed);
            WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            PLAY_SE(fighter, Hash40::new("se_samus_jump03"));
            EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe fn ballspark(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let situation = StatusModule::situation_kind(boma);
    let is_button_trigger_guard = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD);

    // Ballspark by pressing Shield
    if VarModule::get_float(object, vars::samus::instance::SHINESPARK_TIMER) > 0.0
    && is_button_trigger_guard
    && 11.0 < motion_frame
    && motion_frame < 40.0 {
        VarModule::on_flag(object, vars::samus::instance::SHINESPARK_USED);
        VarModule::set_float(object, vars::samus::instance::SHINESPARK_TIMER, 0.0);
        if situation == *SITUATION_KIND_GROUND {
            StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            let hop_speed = Vector3f{x: 0.0, y: 0.45, z: 0.0};
            KineticModule::add_speed(boma, &hop_speed);
            StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW, true);
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw_shinespark"), 18.0, 1.0, 1.0);
        }
        else if situation == *SITUATION_KIND_AIR {
            MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_air_lw_shinespark"), 18.0, 1.0, 1.0);
        }
    }
}

unsafe fn roll_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let motion_frame = MotionModule::frame(boma);
    let status = StatusModule::status_kind(boma);
    let is_button_on_special = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL);
    
    let escape_f_condition =
        status == *FIGHTER_STATUS_KIND_ESCAPE_F
        && 13.0 <= motion_frame
        && motion_frame < 22.0;

    let escape_b_condition =
        status == *FIGHTER_STATUS_KIND_ESCAPE_B
        && 15.0 <= motion_frame
        && motion_frame < 24.0;

    if (escape_f_condition || escape_b_condition)
    && is_button_on_special {
        WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        StatusModule::change_status_request(boma, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW, false);
        MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 11.0, 1.0, 1.0);
    }
}