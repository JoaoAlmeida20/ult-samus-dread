use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_agent_frames!(
        samus_gbeam_frame,
        samus_bomb_frame
    );
}

#[smashline::weapon_frame(agent = WEAPON_KIND_SAMUS_GBEAM)]
pub fn samus_gbeam_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Samus
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SAMUS {
            let samus = utils::get_battle_object_from_id(owner_id);
            let samus_boma = &mut *(*samus).module_accessor;

            if [*FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_DASH,
            *FIGHTER_STATUS_KIND_CATCH_TURN].contains(&StatusModule::status_kind(samus_boma)) {
                if MotionModule::frame(samus_boma) <= 1.0 {
                    VarModule::set_float(samus, vars::samus::instance::GBEAM_ANGLE, 0.0);
                }
                let stick_x = ControlModule::get_stick_x(samus_boma);
                let stick_y = ControlModule::get_stick_y(samus_boma);
                let lr = PostureModule::lr(samus_boma);
                let stick_x_lr = 
                    if stick_x == 0.0_f32 {
                        stick_x
                    }
                    else {
                        stick_x * lr
                    };

                let prev_angle = VarModule::get_float(samus, vars::samus::instance::GBEAM_ANGLE);
                let angle = stick_y.atan2(stick_x_lr).to_degrees().clamp(prev_angle - 15.0, prev_angle + 15.0);
                VarModule::set_float(samus, vars::samus::instance::GBEAM_ANGLE, angle);

                weapon.set_joint_rotate("gbeam1", Vector3f::new(0.0, 0.0, -angle/6.0));
                weapon.set_joint_rotate("gbeam6", Vector3f::new(0.0, 0.0, -angle/6.0));
                weapon.set_joint_rotate("gbeam11", Vector3f::new(0.0, 0.0, -angle/6.0));
                weapon.set_joint_rotate("gbeam16", Vector3f::new(0.0, 0.0, -angle/6.0));
                weapon.set_joint_rotate("gbeam21", Vector3f::new(0.0, 0.0, -angle/6.0));
                weapon.set_joint_rotate("gbeam26", Vector3f::new(0.0, 0.0, -angle/6.0));
            }
        }
    }
}

#[smashline::weapon_frame(agent = WEAPON_KIND_SAMUS_BOMB)]
pub fn samus_bomb_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Samus
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SAMUS {
            let samus = utils::get_battle_object_from_id(owner_id);
            let samus_boma = &mut *(*samus).module_accessor;
            let samus_status = StatusModule::status_kind(samus_boma);

            // Five Bomb Drop
            if StatusModule::status_kind(boma) == *WEAPON_SAMUS_BOMB_STATUS_KIND_FALL
            && MotionModule::frame(boma) == 1.0
            && VarModule::get_int(samus, vars::samus::instance::FIVE_BOMB_DROP_COUNTER) > 0 {
                VarModule::dec_int(samus, vars::samus::instance::FIVE_BOMB_DROP_COUNTER);

                let rng_n = 10;
                let rng = app::sv_math::rand(hash40("fighter"), rng_n) as f32;
                let counter = VarModule::get_int(samus, vars::samus::instance::FIVE_BOMB_DROP_COUNTER) as f32;
                let total_bombs_num = vl::param_fivebombdrop::total_bombs_num as f32;
                let speed_x_max = vl::param_fivebombdrop::speed_x0_max;
                let bomb_social_distance = (speed_x_max * 2.0) / total_bombs_num;
                let variation = rng.clamp(1.0, rng_n as f32 - 2.0) / (rng_n - 1) as f32;
                let speed_x = speed_x_max - bomb_social_distance * (variation + counter);

                let speed_y_min = vl::param_fivebombdrop::speed_y0_min;
                let speed_y_max = vl::param_fivebombdrop::speed_y0_max;
                let speed_y_diff = speed_y_max - speed_y_min;
                let speed_y = speed_y_max - speed_y_diff * (speed_x.abs() / speed_x_max);

                KineticModule::change_kinetic(boma, *WEAPON_KINETIC_TYPE_NORMAL);
                sv_kinetic_energy!(
                    set_speed,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    speed_x,
                    speed_y
                );

                sv_kinetic_energy!(
                    set_accel,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    0.0,
                    vl::param_fivebombdrop::accel_y
                );

                sv_kinetic_energy!(
                    set_brake,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    vl::param_fivebombdrop::brake_x,
                    0.0
                );
            
                sv_kinetic_energy!(
                    set_stable_speed,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    5.0,
                    5.0
                );
            
                sv_kinetic_energy!(
                    set_limit_speed,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    5.0,
                    5.0
                );
            }
        }
    }
}