use super::*;
use table_consts::*;

pub unsafe fn frame(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    flashshift(fighter, boma);
    missile_changes(fighter, boma);
    aim_armcannon(fighter, boma);
    nspecial_cancel(fighter, boma);
    var_resets(fighter, boma);
}

unsafe fn flashshift(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);

    if VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER) > 0.0 {
        VarModule::sub_float(object, vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER, 1.0);

        if VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER) <= 0.0 {
            if WorkModule::get_param_int(fighter.module_accessor, hash40("param_motion"), hash40("flip")) != 0 {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, 18.0, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                let lr = PostureModule::lr(boma);
                EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, 18.0, 2, 0, 0, 0, 1.0, true);
            }
            LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
            FighterUtil::flash_eye_info(boma);
        }
    }

    if VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) > 0.0 {
        VarModule::sub_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER, 1.0);
        if VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) <= 0.0 {
            VarModule::set_int(object, vars::samus::instance::FLASHSHIFT_CHAIN_COUNT, 0);
            VarModule::on_flag(object, vars::samus::instance::FLASHSHIFT_USED);
        }
    }
    
    if VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) > 0.0
    && VarModule::get_int(object, vars::samus::instance::FLASHSHIFT_CHAIN_COUNT) < 3 {
        // If in the middle of a flashshift chain, you can cancel any regular attack into a flashshift
        if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0
        && [*FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status) {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
        }
    }
}
 
unsafe fn missile_changes(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let situation = StatusModule::situation_kind(boma);
    let status = StatusModule::status_kind(boma);
    let motion_frame = MotionModule::frame(boma);

    // Since flash shift is side b, keep super missiles by holding attack during initial frames of neutral b
    if status == *FIGHTER_STATUS_KIND_SPECIAL_N
    && motion_frame <= 6.0
    && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)) {
        if situation == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A.into(), false.into());
        }
    }

    // Land cancel super missiles
    if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
    *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status)
    && situation == *SITUATION_KIND_GROUND
    && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

unsafe fn aim_armcannon(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let motion_frame = MotionModule::frame(boma);
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let stick_y = ControlModule::get_stick_y(boma);

    // Window for aiming arm cannon when shooting super missiles
    let super_missile_condition = 
        [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G,
        *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A].contains(&status)
        && 10.0 <= motion_frame
        && motion_frame <= 27.0;

    // Window for aiming arm cannon when shooting charge shots
    let charge_shot_condition = 
        status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H
        || (status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F
            && motion_frame <= 22.0)
        || (status == *FIGHTER_STATUS_KIND_SPECIAL_N
            && motion_frame <= 12.0);

    /* Window for aiming arm cannon when using grapple beam (zair and grab)
    let grapple_condition =
        (status == *FIGHTER_STATUS_KIND_AIR_LASSO
            && 6.0 <= motion_frame
            && motion_frame <= 31.0)
        || (status == *FIGHTER_STATUS_KIND_CATCH
            && 8.0 <= motion_frame
            && motion_frame <= 29.0)
        || (status == *FIGHTER_STATUS_KIND_CATCH_DASH
            && 15.0 <= motion_frame
            && motion_frame <= 28.0)
        || (status == *FIGHTER_STATUS_KIND_CATCH_TURN 
            && 15.0 <= motion_frame
            && motion_frame <= 30.0); */

    // Rotation is flipped around for grounded super missiles for some reason
    let flip_y = if status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G {
        -1.0
    }
    else {
        1.0
    };

    // Getting the default joint rotations from the current animation
    let mut armr_rot = Vector3f::zero();
    MotionModule::joint_local_rotation(boma, Hash40::new("armr"), &mut armr_rot);
    let mut arml_rot = Vector3f::zero();
    MotionModule::joint_local_rotation(boma, Hash40::new("arml"), &mut arml_rot);
    let mut shoulderr_rot = Vector3f::zero();
    MotionModule::joint_local_rotation(boma, Hash40::new("shoulderr"), &mut shoulderr_rot);
    let mut waist_rot = Vector3f::zero();
    MotionModule::joint_local_rotation(boma, Hash40::new("waist"), &mut waist_rot);

    if super_missile_condition
    || charge_shot_condition {
        let prev_angle = VarModule::get_float(object, vars::samus::instance::AIM_ANGLE);
        let angle = if stick_x != 0.0 {
            (stick_y / stick_x.abs()).atan().to_degrees()
        }
        else {
            stick_y * 90.0
        }.clamp(prev_angle - 15.0, prev_angle + 15.0);
        VarModule::set_float(object, vars::samus::instance::AIM_ANGLE, angle);

        let arm_offset = angle.clamp(-45.0, 45.0) * flip_y;

        if super_missile_condition {
            armr_rot.z += arm_offset;
            fighter.set_joint_rotate("armr", armr_rot);

            if angle.abs() > 45.0
            && situation == *SITUATION_KIND_AIR {
                let shoulderr_offset = angle - (45.0 * angle.signum());
                shoulderr_rot.z += shoulderr_offset;
                fighter.set_joint_rotate("shoulderr", shoulderr_rot);
            }
        }
        else if charge_shot_condition {
            armr_rot.z += arm_offset;
            fighter.set_joint_rotate("armr", armr_rot);
            if status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H {
                arml_rot.z += arm_offset;
                fighter.set_joint_rotate("arml", arml_rot);
            }
    
            if angle.abs() > 45.0
            && situation == *SITUATION_KIND_AIR {
                let waist_offset = (angle - (45.0 * angle.signum())) * -1.0;
                waist_rot.z += waist_offset;
                fighter.set_joint_rotate("waist", waist_rot);
            }
        }
    }
    // Interpolate back to default rotations
    else {
        let prev_angle = VarModule::get_float(object, vars::samus::instance::AIM_ANGLE);
        let angle = 0.0_f32.clamp(prev_angle - 8.0, prev_angle + 8.0);
        VarModule::set_float(object, vars::samus::instance::AIM_ANGLE, angle);

        if angle.abs() > 45.0
        && situation == *SITUATION_KIND_AIR {
            if status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
                let shoulderr_offset = angle - (45.0 * angle.signum());
                shoulderr_rot.z += shoulderr_offset;
                fighter.set_joint_rotate("shoulderr", shoulderr_rot);
            }
            else {
                let waist_offset = (angle - (45.0 * angle.signum())) * -1.0;
                waist_rot.z += waist_offset;
                fighter.set_joint_rotate("waist", waist_rot);
            }
        }
        let arm_offset = angle.clamp(-45.0, 45.0) * flip_y;
        armr_rot.z += arm_offset;
        fighter.set_joint_rotate("armr", armr_rot);
    }
}

unsafe fn nspecial_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);

    if status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C {
        if situation == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
        }
    }
}

unsafe fn var_resets(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);

    let death_statuses =
        [*FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY];

    if death_statuses.contains(&status)
    || situation != *SITUATION_KIND_AIR {
        VarModule::off_flag(object, vars::samus::instance::FLASHSHIFT_USED);
        VarModule::off_flag(object, vars::samus::instance::MORPHBALL_STALL_USED);
        VarModule::set_int(object, vars::samus::instance::BOMB_COUNTER, 0);
    }
}
