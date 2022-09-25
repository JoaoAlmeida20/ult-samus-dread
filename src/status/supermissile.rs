use super::*;
use table_consts::*;

pub fn install() {
    smashline::install_status_scripts!(
        samus_supermissile_ready_main,
        samus_supermissile_straight_pre,
        samus_supermissile_straight_main
    );
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_READY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samus_supermissile_ready_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let otarget_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let oboma = sv_battle_object::module_accessor(otarget_id);
    let oobject = utils::get_battle_object_from_id(otarget_id);
    let lr = PostureModule::lr(boma);
    let accel_frame = WorkModule::get_param_int(boma, hash40("param_supermissile"), hash40("s_acc_f"));
    let x_speed_start = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_spd_x0"));
    let y_speed_start = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_spd_y0"));
    let rot = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_rot"));

    let angle = if StatusModule::situation_kind(oboma) == *SITUATION_KIND_AIR {
        VarModule::get_float(oobject, vars::samus::instance::AIM_ANGLE)
    }
    else {
        VarModule::get_float(oobject, vars::samus::instance::AIM_ANGLE).clamp(-45.0, 45.0)
    };
    VarModule::set_float(weapon.battle_object, vars::supermissile::instance::ANGLE, angle);

    MotionModule::change_motion(boma, Hash40::new("ready"), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::set_int(boma, accel_frame, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        x_speed_start * lr,
        -y_speed_start
    );

    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -(x_speed_start * lr) / (accel_frame as f32),
        y_speed_start / (accel_frame as f32)
    );

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        -(rot / (accel_frame as f32)),
        0.0,
        0.0
    );

    ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    KineticModule::enable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(samus_supermissile_ready_main_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(samus_supermissile_ready_main_loop as *const () as _))
    
}

unsafe extern "C" fn samus_supermissile_ready_main_substatus(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    let boma = weapon.module_accessor;

    if param_2.get_bool() {
        WorkModule::dec_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME)
    }
    0.into()
}

unsafe extern "C" fn samus_supermissile_ready_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;

    let angle = VarModule::get_float(weapon.battle_object, vars::supermissile::instance::ANGLE);
    ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }
    else if WorkModule::get_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_READY_WORK_ID_INT_FRAME) <= 0 {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn samus_supermissile_straight_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL, // Originally _NONE
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

#[status_script(agent = "samus_supermissile", status = WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_STRAIGHT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samus_supermissile_straight_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;

    let angle = VarModule::get_float(weapon.battle_object, vars::supermissile::instance::ANGLE);
    let accel = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_acc_x"));
    let max_speed = WorkModule::get_param_float(boma, hash40("param_supermissile"), hash40("s_spd_x_max"));
    let lr = PostureModule::lr(boma);
    let accel_x = angle.to_radians().cos() * accel * lr;
    let accel_y = angle.to_radians().sin() * accel;
    let max_speed_x = angle.to_radians().cos() * max_speed;
    let max_speed_y = angle.to_radians().sin() * max_speed;

    ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        accel_x,
        accel_y
    );

    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        max_speed_x,
        max_speed_y
    );

    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        max_speed_x,
        max_speed_y
    );

    MotionModule::change_motion(boma, Hash40::new("straight"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::unable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(samus_supermissile_straight_main_substatus as *const () as _));
    weapon.fastshift(L2CValue::Ptr(samus_supermissile_straight_main_loop as *const () as _))
}

unsafe extern "C" fn samus_supermissile_straight_main_substatus(weapon: &mut L2CWeaponCommon, param_2: L2CValue) -> L2CValue {
    let boma = weapon.module_accessor;

    if param_2.get_bool() {
        WorkModule::dec_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME)
    }
    0.into()
}

unsafe extern "C" fn samus_supermissile_straight_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;

    let angle = VarModule::get_float(weapon.battle_object, vars::supermissile::instance::ANGLE);
    ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{ x: -angle, y: 0.0, z: 0.0 }, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});

    if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_ALL as u32)
    || WorkModule::get_int(boma, *WEAPON_SAMUS_SUPERMISSILE_STATUS_STRAIGHT_WORK_ID_INT_FRAME) <= 0 {
        weapon.change_status(WEAPON_SAMUS_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }
    0.into()
}