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

            if StatusModule::status_kind(boma) == *WEAPON_SAMUS_BOMB_STATUS_KIND_FALL
            && MotionModule::frame(boma) == 1.0
            && VarModule::get_int(samus, vars::samus::instance::BOMB_BURST_COUNTER) > 0 {
                VarModule::dec_int(samus, vars::samus::instance::BOMB_BURST_COUNTER);

                let rng = app::sv_math::rand(hash40("fighter"), 10) as f32;
                let rng_dir = app::sv_math::rand(hash40("fighter"), 2);
                let dir =
                    if rng_dir == 0 {
                        1.0
                    }
                    else {
                        -1.0
                    };

                KineticModule::change_kinetic(boma, *WEAPON_KINETIC_TYPE_NORMAL);
                sv_kinetic_energy!(
                    set_speed,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    0.0 + dir * rng * 0.1,
                    1.5 - rng * 0.1
                );

                sv_kinetic_energy!(
                    set_accel,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    0.0,
                    -0.05
                );

                sv_kinetic_energy!(
                    set_brake,
                    weapon,
                    WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                    0.05,
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