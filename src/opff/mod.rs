use super::*;

mod misc;
mod morphball;
pub mod speedboost;

pub fn install() {
    smashline::install_agent_frame_callbacks!(samus_frame);
    smashline::install_agent_frames!(samus_gbeam_frame);
}

#[smashline::fighter_frame_callback]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let category = utility::get_category(boma);
        let kind = utility::get_kind(boma);
        if category == BATTLE_OBJECT_CATEGORY_FIGHTER && kind == FIGHTER_KIND_SAMUS {
            misc::frame(fighter, boma);
            morphball::frame(fighter, boma);
            speedboost::frame(fighter, boma);
        }
    }
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

                let prev_angle = VarModule::get_float(samus, vars::samus::instance::GBEAM_ANGLE);
                let angle = stick_y.atan2(stick_x * lr).to_degrees().clamp(prev_angle - 15.0, prev_angle + 15.0);
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