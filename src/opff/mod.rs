use super::*;

mod misc;
mod morphball;
mod weapons;
pub mod speedboost;

pub fn install() {
    smashline::install_agent_frame_callbacks!(samus_frame);
    weapons::install();
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