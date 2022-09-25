use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_status_scripts!(
        exit_special_n
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