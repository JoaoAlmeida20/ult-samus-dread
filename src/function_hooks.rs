use super::*;
use table_consts::*;

pub fn install() {
    skyline::install_hooks!(
        get_param_float_hook
    );
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
