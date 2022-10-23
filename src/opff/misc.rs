use super::*;
use table_consts::*;

pub unsafe fn frame(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    spin_jump(fighter, boma);
    beam(fighter, boma);
    swing(fighter, boma);
    flashshift(fighter, boma);
    missile_changes(fighter, boma);
    aim_armcannon(fighter, boma);
    var_resets(fighter, boma);
}

unsafe fn spin_jump(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let motion = MotionModule::motion_kind(boma);
    let motion_frame = MotionModule::frame(boma);
    let stick_x = ControlModule::get_stick_x(boma);
    let lr = PostureModule::lr(boma);
    
    if [hash40("jump_f"), hash40("jump_f_mini")].contains(&motion)
    && motion_frame <= 1.0
    && stick_x * lr > 0.5 {
        MotionModule::change_motion(boma, Hash40::new("jump_f_spin"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn beam(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);
    let motion_frame = MotionModule::frame(boma);
    let is_button_trigger_special = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL);

    if status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F {
        let charge_frame = WorkModule::get_int(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_COUNT);

        if charge_frame < 10
        && 10.0 <= motion_frame
        && motion_frame <= 30.0
        && is_button_trigger_special {
            let hash = 
                if situation == *SITUATION_KIND_GROUND {
                    Hash40::new("special_n_f")
                }
                else {
                    Hash40::new("special_air_n_f")
                };
            ArticleModule::generate_article_enable(boma, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, false, -1);
            WorkModule::on_flag(boma, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
            MotionModule::change_motion(boma, hash, 0.0, 1.0, false, 0.0, false, false);
        }
    }

}

unsafe fn swing(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let status = StatusModule::status_kind(boma);
    let situation = StatusModule::situation_kind(boma);
    let lr = PostureModule::lr(boma);
    let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);

    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let prev_pos_x = VarModule::get_float(fighter.battle_object, vars::samus::instance::HANG_PREV_POS_X);
    let prev_pos_y = VarModule::get_float(fighter.battle_object, vars::samus::instance::HANG_PREV_POS_Y);
    VarModule::set_float(fighter.battle_object, vars::samus::instance::HANG_PREV_POS_X, pos_x);
    VarModule::set_float(fighter.battle_object, vars::samus::instance::HANG_PREV_POS_Y, pos_y);

    let real_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let real_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    println!("real_speed_x: {}", real_speed_x);
    println!("real_speed_y: {}", real_speed_y);

    if VarModule::is_flag(fighter.battle_object, vars::samus::instance::IS_SWING) {
        if situation == *SITUATION_KIND_AIR
        && real_speed_x.abs() > air_speed_x_stable {
            let speed_x = VarModule::get_float(fighter.battle_object, vars::samus::instance::SWING_SPEED_X);
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_x.abs().max(air_speed_x_stable)
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_x.abs().max(air_speed_x_stable)
            );
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::samus::instance::IS_SWING);
            if situation == *SITUATION_KIND_AIR {
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    air_speed_x_stable
                );
                sv_kinetic_energy!(
                    set_limit_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                    WorkModule::get_param_float(boma, hash40("common"), hash40("air_speed_x_limit"))
                );
            }
        }
    }

    if status == *FIGHTER_STATUS_KIND_AIR_LASSO_HANG
    && ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_GUARD) {
        let speed_x = (pos_x - prev_pos_x).clamp(-2.5, 2.5);
        let speed_y = (pos_y - prev_pos_y).max(0.0);
        println!("speed_x: {}", speed_x);
        println!("speed_y: {}", speed_y);

        StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_FALL, true);
        VarModule::set_float(fighter.battle_object, vars::samus::instance::SWING_SPEED_X, speed_x);
        VarModule::on_flag(fighter.battle_object, vars::samus::instance::IS_SWING);
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x.abs().max(air_speed_x_stable)
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x.abs().max(air_speed_x_stable)
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
    }
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
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
        }
    }
}
 
unsafe fn missile_changes(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let situation = StatusModule::situation_kind(boma);
    let status = StatusModule::status_kind(boma);
    let motion_frame = MotionModule::frame(boma);

    // Since flash shift is side b, keep super missiles by holding attack during initial frames of neutral b
    if status == *FIGHTER_STATUS_KIND_SPECIAL_N
    && 5.0 <= motion_frame
    && motion_frame <= 6.0
    && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
        || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)) {
        if situation == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A.into(), true.into());
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
            && motion_frame >= 10.0);

    // Rotation is flipped around for aerial super missiles for some reason
    let flip_y = if status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        -1.0
    }
    else {
        1.0
    };

    let mut clavicler_rot = Vector3f::zero();
    let mut claviclel_rot = Vector3f::zero();
    let mut neck_rot = Vector3f::zero();
    let mut bust_rot = Vector3f::zero();

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

        let neck_offset = angle.clamp(-30.0, 30.0);
        neck_rot.z -= neck_offset;
        fighter.set_joint_rotate("neck", neck_rot);

        let clavicler_offset = angle.clamp(-45.0, 45.0) * flip_y;
        if charge_shot_condition {
            clavicler_rot.x += clavicler_offset;
            fighter.set_joint_rotate("clavicler", clavicler_rot);

            // Left arm would start clipping under -18.0
            let claviclel_offset = angle.clamp(-18.0, 45.0);
            claviclel_rot.x -= claviclel_offset;
            fighter.set_joint_rotate("claviclel", claviclel_rot);
        }
        else {
            clavicler_rot.z -= clavicler_offset;
            fighter.set_joint_rotate("clavicler", clavicler_rot);
        }
        if angle.abs() > 45.0
        && situation == *SITUATION_KIND_AIR {
            let bust_offset = angle - (45.0 * angle.signum());
            if charge_shot_condition {
                bust_rot.z -= bust_offset;
                fighter.set_joint_rotate("waist", bust_rot);
            }
            else {
                bust_rot.y -= bust_offset;
                fighter.set_joint_rotate("bust", bust_rot);
            }
        }
    }
    // Interpolate back to default rotations
    else {
        let prev_angle = VarModule::get_float(object, vars::samus::instance::AIM_ANGLE);
        let angle = 0.0_f32.clamp(prev_angle - 8.0, prev_angle + 8.0);
        VarModule::set_float(object, vars::samus::instance::AIM_ANGLE, angle);

        if angle != 0.0_f32 {
            let neck_offset = angle.clamp(-30.0, 30.0);
            neck_rot.z -= neck_offset;
            fighter.set_joint_rotate("neck", neck_rot);

            if angle.abs() > 45.0
            && situation == *SITUATION_KIND_AIR {
                let bust_offset = angle - (45.0 * angle.signum());
                if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
                *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C].contains(&status) {
                    bust_rot.z -= bust_offset;
                    fighter.set_joint_rotate("waist", bust_rot);
                }
                else {
                    bust_rot.y -= bust_offset;
                    fighter.set_joint_rotate("bust", bust_rot);
                }
            }

            let clavicler_offset = angle.clamp(-45.0, 45.0) * flip_y;
            if [*FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C].contains(&status) {
                clavicler_rot.x += clavicler_offset;
                fighter.set_joint_rotate("clavicler", clavicler_rot);

                let claviclel_offset = angle.clamp(-18.0, 45.0);
                claviclel_rot.x -= claviclel_offset;
                fighter.set_joint_rotate("claviclel", claviclel_rot);
            }
            else {
                clavicler_rot.z -= clavicler_offset;
                fighter.set_joint_rotate("clavicler", clavicler_rot);
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
