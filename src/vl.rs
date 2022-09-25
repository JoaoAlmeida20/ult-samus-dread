#![allow(non_upper_case_globals)]

pub mod param_speedboost {
    pub const charge_frame : f32 = 35.0;
    pub const speed_max : f32 = 2.65;
    pub const run_accel_add : f32 = 0.5;
}

pub mod param_shinespark {
    pub const storage_duration : f32 = 300.0;
    pub const ground_speed: f32 = 1.6;
    pub const air_speed: f32 = 2.4;
    pub const ball_speed: f32 = 8.0;
}

pub mod param_flashshift {
    pub const cooldown : f32 = 180.0;
    pub const chain_frame : f32 = 25.0;
    pub const chain_max_num : i32 = 3;
}

pub mod param_supermissile {
    pub const spike_min_angle : f32 = -70.0;
}

pub mod param_speciallw {
    pub const bomb_max_num_airtime: i32 = 8;
}