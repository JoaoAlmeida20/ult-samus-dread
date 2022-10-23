use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_status_scripts!(
        exit_special_n,
        main_special_n_h
    );
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn exit_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Prevents losing charge if you switch to missiles during neutral special startup
    if fighter.global_table[STATUS_KIND] == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2G
    || fighter.global_table[STATUS_KIND] == FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A {
        return 0.into();
    }
    original!(fighter)
}

#[status_script(agent = "samus", status = FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_H, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_special_n_h(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(boma, Hash40::new("special_n_h"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(boma, Hash40::new("special_air_n_h"), 0.0, 1.0, false, 0.0, false, false);
    }

    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_BUTTON);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_FLY_NEXT);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);

    ControlModule::set_add_jump_mini_button_life(boma, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(main_special_n_h_loop as *const () as _))
}

unsafe extern "C" fn main_special_n_h_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let prev_situation = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if prev_situation == *SITUATION_KIND_AIR
    && situation == *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_n_h"), -1.0, 1.0, 0.0, false, false);
    }
    else if prev_situation == *SITUATION_KIND_GROUND
    && situation == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_n_h"), -1.0, 1.0, 0.0, false, false);
    }

    if fighter.global_table[PAD_FLAG].get_i32() & (*FIGHTER_PAD_FLAG_SPECIAL_TRIGGER | *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) != 0 {
        if situation == *SITUATION_KIND_AIR {
            MotionModule::change_motion(boma, Hash40::new("special_air_n_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(boma, Hash40::new("special_n_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_F.into(), true.into());
        return 1.into();
    }

    if situation == *SITUATION_KIND_GROUND {
        // Removed cat2 escape, escape_f, escape_b checks from vanilla (spotdodge, rolls)

        // christian jump cancelling
        if fighter.sub_check_button_jump().get_bool() || fighter.sub_check_button_frick().get_bool() {
            WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_JUMP, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
            return 1.into();
        }

        if fighter.sub_check_command_guard().get_bool()  {
            if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_GROUND_GUARD, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
            else {
                WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            }
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
            return 1.into();
        }
    }
    else {
        if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_GUARD_TRIGGER != 0 {
            // in vanilla would be a bunch of airdodge check logic but we just want tapping shield to cancel into an actionable state
            WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
            fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_C.into(), true.into());
            return 1.into();
        }

        let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        let jump_count_max = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
            if jump_count < jump_count_max
            && ControlModule::is_enable_flick_jump(boma) {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL) {
                    WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                }
                else {
                    WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                }
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into());
                return 1.into();
            }
        }
        else if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON != 0 {
            if jump_count < jump_count_max {
                if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON) {
                    WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_AIR_JUMP_AERIAL, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                }
                else {
                    WorkModule::set_int(boma, *FIGHTER_SAMUS_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_SAMUS_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                }
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into());
                return 1.into();
            }
        }
    }

    let cshot_charge_max_frame = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("cshot_charge_frame"));
    if fighter.global_table[IS_STOPPING].get_bool() {
        let cshot_charge_current_frame = WorkModule::get_int(boma, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT) as f32;
        let charge_fraction = cshot_charge_current_frame / cshot_charge_max_frame;
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x26b38955ef), charge_fraction);
    }
    else {
        WorkModule::inc_int(boma, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        let cshot_charge_current_frame = WorkModule::get_int(boma, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT) as f32;

        if cshot_charge_current_frame >= cshot_charge_max_frame {
            WorkModule::set_int(boma, cshot_charge_max_frame as i32, *FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                fighter.change_status(FIGHTER_SAMUS_STATUS_KIND_SPECIAL_N_E.into(), false.into());
                return 1.into();
            }
        }
        else {
            let charge_fraction = cshot_charge_current_frame / cshot_charge_max_frame;
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x26b38955ef), charge_fraction);
        }
    }
    0.into()
}