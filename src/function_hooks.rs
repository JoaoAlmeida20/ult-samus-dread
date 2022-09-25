use super::*;
use table_consts::*;

pub fn install() {
    skyline::install_hooks!(
        is_enable_transition_term_hook,
        change_status_request_from_script_hook,
        get_param_float_hook
    );
}

#[skyline::hook(replace=WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let fighter_kind = utility::get_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let object = utils::get_battle_object_from_id((*boma).battle_object_id);

        // Disable Samus flash shift on certain conditions
        if fighter_kind == *FIGHTER_KIND_SAMUS
        && flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
            let can_chain = VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) > 0.0
                            && VarModule::get_int(object, vars::samus::instance::FLASHSHIFT_CHAIN_COUNT) < vl::param_flashshift::chain_max_num;

            // If it's on cooldown or has already been used in this airtime and Samus can't chain another flash shift
            if (VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER) > 0.0
            || VarModule::is_flag(object, vars::samus::instance::FLASHSHIFT_USED))
            && !can_chain {
                return false;
            }
        }
    }   
    original!()(boma, flag)
}

#[skyline::hook(replace=StatusModule::change_status_request_from_script)]
unsafe fn change_status_request_from_script_hook(boma: &mut BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
    let mut next_status = status_kind;
    let mut clear_buffer = arg3;

    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(boma) == *FIGHTER_KIND_SAMUS {       
        let object = utils::get_battle_object_from_id((*boma).battle_object_id);
        let current_status = StatusModule::status_kind(boma);
        let current_motion = MotionModule::motion_kind(boma);
        let stick_x = ControlModule::get_stick_x(boma);
        let lr = PostureModule::lr(boma);

        if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
            // Disable speedboost if:
            // 1. about to wavedash
            if (current_status == *FIGHTER_STATUS_KIND_JUMP_SQUAT
                && next_status == *FIGHTER_STATUS_KIND_ESCAPE_AIR)
            // 2. in runbrake (and not as a stepping stone into crouch or dtilt)
            || (current_status == *FIGHTER_STATUS_KIND_RUN_BRAKE
                && ![*FIGHTER_STATUS_KIND_SQUAT, *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&next_status))
            // 3. about to double jump
            || next_status == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                opff::speedboost::speedboost_end(boma);
            }

            // Buffer run if holding forward instead of walk during certain statuses so that speedboost can be maintained
            if [*FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW,
            *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_AIR_LW].contains(&current_status)
            && next_status == *FIGHTER_STATUS_KIND_WALK {
                next_status = *FIGHTER_STATUS_KIND_RUN;
            }
        }
        // Give speedboost back if samus does aerial shinespark into the ground
        if current_motion == hash40("attack_dash")
        && next_status == *FIGHTER_STATUS_KIND_LANDING
        && stick_x * lr > 0.75 {
            opff::speedboost::speedboost_start(boma);
            next_status = *FIGHTER_STATUS_KIND_RUN;
        }
        // Give speedboost back if samus does ballspark into the ground
        if current_motion == hash40("special_air_lw_shinespark")
        && next_status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW {
            opff::speedboost::speedboost_start(boma);
        }
    }
    original!()(boma, next_status, clear_buffer)
}

#[skyline::hook(offset=0x4e53C0)]
pub unsafe fn get_param_float_hook(x0 /*boma*/: u64, x1 /*param_type*/: u64, x2 /*param_hash*/: u64) -> f32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let object = utils::get_battle_object_from_id((*boma).battle_object_id);

    if utility::get_category(boma_reference) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && utility::get_kind(boma_reference) == *FIGHTER_KIND_SAMUS {
        if VarModule::is_flag(object, vars::samus::instance::SPEEDBOOST_ACTIVE) {
            if [hash40("dash_speed"),
            hash40("air_speed_x_stable"),
            hash40("jump_speed_x_max"),
            hash40("passive_wall_jump_x_speed")].contains(&x1) {
                return vl::param_speedboost::speed_max;
            }
            if x1 == hash40("run_accel_add") {
                return vl::param_speedboost::speed_max;
            }
            if x1 == hash40("param_special_lw") {
                if [hash40("sp_lw_ar_vx_mul"), hash40("sp_lw_gr_vx_mul")].contains(&x2) {
                    let walk_speed_max = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
                    return vl::param_speedboost::speed_max / walk_speed_max;
                }
                else if [hash40("sp_lw_ar_ax_mul"), hash40("sp_lw_gr_ax_mul")].contains(&x2) {
                    let air_speed_x_stable = WorkModule::get_param_float(boma, hash40("air_speed_x_stable"), 0);
                    return vl::param_speedboost::speed_max / air_speed_x_stable;
                }
            }
        }
        if x1 == hash40("param_special_lw") 
        && x2 == hash40("sp_lw_ar_vy0")
        && VarModule::is_flag(object, vars::samus::instance::MORPHBALL_STALL_USED) {
            return 0.0;
        }

        // Do this through function hook instead of editing fighter_param.prc as to facilitate using this mod with bigger ones that make way more changes to fighter_param
        if x1 == hash40("landing_attack_air_frame_hi") {
            return 10.0;
        }
    }
    original!()(x0, x1, x2)
}
