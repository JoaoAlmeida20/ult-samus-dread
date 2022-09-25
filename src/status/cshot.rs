use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_status_scripts!(
        samus_cshot_shoot_init
    );
}

#[status_script(agent = "samus_cshot", status = WEAPON_SAMUS_CSHOT_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn samus_cshot_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = utils::get_battle_object_from_id(otarget_id);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED)
    && !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            1.0,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET);
    }
    let lr = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_SHOOT_LR);
    let charge = WorkModule::get_float(weapon.module_accessor, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_FLOAT_CHARGE);
    // let angle = WorkModule::get_param_int(weapon.module_accessor, hash40("param_cshot"), hash40("angle")) as f32; An angle, in radians, as an integer. very funny
    let angle = if StatusModule::situation_kind(oboma) == *SITUATION_KIND_AIR {
        VarModule::get_float(oobject, vars::samus::instance::AIM_ANGLE)
    }
    else {
        VarModule::get_float(oobject, vars::samus::instance::AIM_ANGLE).clamp(-45.0, 45.0)
    };
    let min_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_speed"));
    let max_speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_speed"));
    let speed = (max_speed - min_speed) * charge + min_speed;
    let speed_x = angle.to_radians().cos() * speed * lr;
    let speed_y = angle.to_radians().sin() * speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -1.0,
        -1.0
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0,
        0.0
    );
    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        let min_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("min_scale"));
        let max_scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_cshot"), hash40("max_scale"));
        let scale = (max_scale - min_scale) * charge + min_scale;
        if (0.3..1.0).contains(&scale) {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samus_cshot_bullet_sub_a"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
        }
        else {
            effect!(
                weapon,
                MA_MSC_EFFECT_REQUEST_FOLLOW,
                Hash40::new("samus_cshot_bullet_sub_b"),
                Hash40::new("top"),
                7.98004,
                -0.50584,
                -0.25092,
                -91.2728,
                -1.7974,
                176.373,
                scale,
                false,
                0,
                0,
                0
            );
        }
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW);
        effect!(
            weapon,
            MA_MSC_EFFECT_REQUEST_FOLLOW,
            Hash40::new("samus_cshot_bullet_sub"),
            Hash40::new("top"),
            7.98004,
            -0.50584,
            -0.25092,
            -91.2728,
            -1.7974,
            176.373,
            scale,
            false,
            0,
            0,
            0
        );
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_EFFECT_GET_LAST_HANDLE);
        sv_module_access::effect(weapon.lua_state_agent);
        let handle = weapon.pop_lua_stack(1).get_i32();
        WorkModule::set_int(weapon.module_accessor, handle, *WEAPON_SAMUS_CSHOT_INSTANCE_WORK_ID_INT_EFH_BULLET_FOLLOW_SUB);
    }
    0.into()
}