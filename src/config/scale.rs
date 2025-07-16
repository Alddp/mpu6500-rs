use crate::register::{
    ACCEL_SCALE_2G, ACCEL_SCALE_4G, ACCEL_SCALE_8G, ACCEL_SCALE_16G, GYRO_SCALE_250,
    GYRO_SCALE_500, GYRO_SCALE_1000, GYRO_SCALE_2000,
};
// 加速度计量程配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelScale {
    Scale2G = 0x00,  // ±2g
    Scale4G = 0x08,  // ±4g
    Scale8G = 0x10,  // ±8g
    Scale16G = 0x18, // ±16g
}

impl AccelScale {
    pub fn get_scale_factor(&self) -> f32 {
        match self {
            AccelScale::Scale2G => ACCEL_SCALE_2G,
            AccelScale::Scale4G => ACCEL_SCALE_4G,
            AccelScale::Scale8G => ACCEL_SCALE_8G,
            AccelScale::Scale16G => ACCEL_SCALE_16G,
        }
    }
}

// 陀螺仪量程配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroScale {
    Scale250 = 0x00,  // ±250°/s
    Scale500 = 0x08,  // ±500°/s
    Scale1000 = 0x10, // ±1000°/s
    Scale2000 = 0x18, // ±2000°/s
}

impl GyroScale {
    pub fn get_scale_factor(&self) -> f32 {
        match self {
            GyroScale::Scale250 => GYRO_SCALE_250,
            GyroScale::Scale500 => GYRO_SCALE_500,
            GyroScale::Scale1000 => GYRO_SCALE_1000,
            GyroScale::Scale2000 => GYRO_SCALE_2000,
        }
    }
}
