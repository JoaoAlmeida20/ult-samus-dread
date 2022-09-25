use smash::{
    lib::{
        L2CValue,
        LuaConst,
    },
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lib::lua_const::*,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smashline::*;
use custom_var::*;
use utils::*;

pub mod utils;
pub mod table_consts;
pub mod vl;
pub mod vars;
pub mod function_hooks;
pub mod acmd;
pub mod opff;
pub mod status;

#[skyline::main(name = "samus_fighters_pass")]
pub fn main() {
    vars::install();
    function_hooks::install();
    acmd::install();
    opff::install();
    status::install();
}
