use super::*;
use table_consts::*;

mod attack_lw3;
mod special_n;
mod special_s;
mod cshot;
mod supermissile;

pub fn install() {
    smashline::install_agent_init_callbacks!(samus_init);
    attack_lw3::install();
    special_n::install();
    special_s::install();
    cshot::install();
    supermissile::install();
}

unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object = fighter.battle_object;

    // Disable Samus flash shift if Samus can't chain another flash shift and it's either on cooldown or has already been used in this airtime
    let can_chain = VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_CHAIN_TIMER) > 0.0
                    && VarModule::get_int(object, vars::samus::instance::FLASHSHIFT_CHAIN_COUNT) < vl::param_flashshift::chain_max_num;

    if !can_chain 
    && (VarModule::get_float(object, vars::samus::instance::FLASHSHIFT_COOLDOWN_TIMER) > 0.0
        || VarModule::is_flag(object, vars::samus::instance::FLASHSHIFT_USED)) {
        false.into()
    }
    else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object = fighter.battle_object;
    let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let next_status = fighter.global_table[STATUS_KIND].get_i32();
    let current_status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let current_motion = MotionModule::motion_kind(fighter.module_accessor);
    let stick_x = fighter.global_table[STICK_X].get_f32();
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
            fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), true.into());
        }
    }

    // Give speedboost back if samus does aerial shinespark into the ground
    if current_motion == hash40("attack_dash")
    && next_status == *FIGHTER_STATUS_KIND_LANDING
    && stick_x * lr > 0.75 {
        opff::speedboost::speedboost_start(boma);
        fighter.change_status(FIGHTER_STATUS_KIND_RUN.into(), true.into());
    }

    // Give speedboost back if samus does ballspark into the ground
    if current_motion == hash40("special_air_lw_shinespark")
    && next_status == *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_GROUND_LW {
        opff::speedboost::speedboost_start(boma);
    }
    true.into()
}

#[smashline::fighter_init]
fn samus_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = utility::get_kind(boma);
        // set the callbacks on fighter init
        if kind == *FIGHTER_KIND_SAMUS {
            fighter.global_table[USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}