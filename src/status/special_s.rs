use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_status_scripts!(
        pre_special_s,
        main_special_s,
        end_special_s
    );
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);

    PostureModule::set_stick_lr(boma, 0.0);
    PostureModule::update_rot_y_lr(boma);
    MotionModule::change_motion(boma, Hash40::new("special"), 0.0, 1.0, false, 0.0, false, false);

    // Prevent previous flashshift's chain timer from running out before new timer starts at end of this flashshift
    VarModule::set_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER, 999.0);
    VarModule::inc_int(object, vars::samus::instance::FLASHSHIFT_CHAIN_COUNT);

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(main_special_s_loop as *const () as _))
}

unsafe extern "C" fn main_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if situation != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }

    if situation == *SITUATION_KIND_AIR {
        let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let is_turn_dash = cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0;
        let is_jump = cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0;
        if (touch_right || touch_left) && (is_turn_dash || is_jump) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
        }
    }

    let stick_x = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(boma);
    let speed_mul = 0.8 + 0.2 * stick_x;
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        speed_mul.powi(2)
    );
    MotionModule::set_rate(boma, 1.0 / speed_mul);

    if MotionModule::is_end(fighter.module_accessor) {
        if situation == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn end_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);

    VarModule::set_float(object, vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER, vl::param_flashshift::cooldown);
    VarModule::set_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER, vl::param_flashshift::chain_frame);

    0.into()
}