use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_status_scripts!(
        pre_attack_lw3,
        main_attack_lw3
    );
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn pre_attack_lw3(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND as u32, // originally *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_attack_lw3(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(main_attack_lw3_loop as *const () as _))
}

unsafe extern "C" fn main_attack_lw3_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);
    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if situation != *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(boma) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return 0.into();
            }
        }

        if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE) {
            if StatusModule::is_changing(boma)
            || (ComboModule::count(boma) < WorkModule::get_param_int(boma, hash40("s3_combo_max"), 0) as u64
                && WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)) {
                fighter.attack_s3_mtrans();
            }
        }

        if fighter.global_table[CURRENT_FRAME].get_f32() >= 7.0 {
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW.into(), false.into());
                MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("special_lw"), 6.0, 1.0, 1.0);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            fighter.sub_transition_group_check_ground_jump_mini_attack();
            fighter.sub_transition_group_check_ground_jump();
        }

        let mut motion_mul = 1.0;
        
        if fighter.global_table[CURRENT_FRAME].get_f32() >= 8.0
        && fighter.global_table[CURRENT_FRAME].get_f32() <= 20.0 {
            motion_mul *= 1.8;
            MotionModule::set_rate(boma, 0.7);
        }
        else {
            MotionModule::set_rate(boma, 1.0);
        }

        if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
            motion_mul *= 1.7;
            if fighter.global_table[CURRENT_FRAME].get_f32() >= 20.0
            && fighter.global_table[STICK_X].get_f32() * PostureModule::lr(boma) > 0.75 {
                fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), false.into());
            }
        }

        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            motion_mul
        );

        let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        if 0 < jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor)
            && fighter.sub_check_button_jump().get_bool() {
                let log = fighter.status_attack();
                let info = log[0x10f40d7b92u64].get_i64();
                let mot = MotionModule::motion_kind(fighter.module_accessor);
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EXPRESSION,
                    Hash40::new_raw(mot),
                    -1
                );
                WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
        if 1 == jump_attack_frame {
            if fighter.global_table[IS_STOPPING].get_bool()
            && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
                let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
    }
    0.into()
}