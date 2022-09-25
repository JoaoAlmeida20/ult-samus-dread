use super::*;

// wubor patch
#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

// huge raw remix
pub trait BomaExt {
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f);
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn set_joint_rotate(&mut self, bone_name: &str, rotation: Vector3f) {
        ModelModule::set_joint_rotate(self, Hash40::new(&bone_name), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
}

pub trait Vec3Ext {
    fn new(x: f32, y: f32, z: f32) -> Self where Self: Sized;
    fn zero() -> Self where Self: Sized;
    fn mag(&self) -> f32;
    fn normalize(&self) -> Self;
}

impl Vec3Ext for Vector3f {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.mag();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag
        }
    }
}