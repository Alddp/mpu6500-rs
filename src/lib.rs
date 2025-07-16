#![no_std]
#![no_main]

pub mod config;
pub mod driver;
pub mod register;
pub mod util;

pub use crate::config::Mpu6500Config;

pub const DEG2RAD: f32 = core::f32::consts::PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / core::f32::consts::PI;

/// MPU6500 数据快照结构体
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SensorData {
    pub accel: (f32, f32, f32),
    pub gyro: (f32, f32, f32),
    pub temp: f32,
}

/// MPU6500 链式构建器
pub struct Mpu6500Builder<SPI, CS> {
    spi: Option<SPI>,
    cs: Option<CS>,
    config: Mpu6500Config,
    accel_offset: (i16, i16, i16),
    gyro_offset: (i16, i16, i16),
    initial_attitude: (f32, f32, f32),
}

impl<SPI, CS> Mpu6500Builder<SPI, CS> {
    pub fn new() -> Self {
        Self {
            spi: None,
            cs: None,
            config: Mpu6500Config::default(),
            accel_offset: (0, 0, 0),
            gyro_offset: (0, 0, 0),
            initial_attitude: (0.0, 0.0, 0.0),
        }
    }
    pub fn spi(mut self, spi: SPI) -> Self {
        self.spi = Some(spi);
        self
    }
    pub fn cs(mut self, cs: CS) -> Self {
        self.cs = Some(cs);
        self
    }
    pub fn config(mut self, config: Mpu6500Config) -> Self {
        self.config = config;
        self
    }
    pub fn accel_offset(mut self, offset: (i16, i16, i16)) -> Self {
        self.accel_offset = offset;
        self
    }
    pub fn gyro_offset(mut self, offset: (i16, i16, i16)) -> Self {
        self.gyro_offset = offset;
        self
    }
    pub fn initial_attitude(mut self, pitch: f32, roll: f32, yaw: f32) -> Self {
        self.initial_attitude = (pitch, roll, yaw);
        self
    }
    pub fn build(self) -> Mpu6500<SPI, CS> {
        Mpu6500 {
            spi: self.spi.expect("SPI未设置"),
            cs: self.cs.expect("CS未设置"),
            config: self.config,
            accel_offset: self.accel_offset,
            gyro_offset: self.gyro_offset,
            last_update: None,
            pitch: self.initial_attitude.0,
            roll: self.initial_attitude.1,
            yaw: self.initial_attitude.2,
        }
    }
}

/// MPU6500 主结构体
pub struct Mpu6500<SPI, CS> {
    pub(crate) spi: SPI,
    pub(crate) cs: CS,
    pub(crate) config: Mpu6500Config,
    pub(crate) accel_offset: (i16, i16, i16),
    pub(crate) gyro_offset: (i16, i16, i16),
    pub(crate) last_update: Option<u64>,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
}
