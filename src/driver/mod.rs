//! MPU6500 驱动相关API，详见 [`Mpu6500`] 结构体。

pub use crate::Mpu6500;

mod calibrate;
mod config_ops;
mod fifo;
mod interrupt;
mod power;
mod read;
mod reg_rw;

use crate::config::Mpu6500Config;
use crate::config::calculate_sample_rate_divider;
use crate::register::*;
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS> Mpu6500<SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
{
    /// 创建新的MPU6500实例
    pub fn new(spi: SPI, cs: CS, config: Mpu6500Config) -> Self {
        Self {
            spi,
            cs,
            config,
            accel_offset: (0, 0, 0),
            gyro_offset: (0, 0, 0),
            last_update: None,
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
        }
    }

    /// 基本初始化
    pub async fn init(&mut self) -> Result<(), SPI::Error> {
        // 唤醒设备
        self.write_register(PWR_MGMT_1, 0x00).await?;
        Timer::after_millis(100).await;
        Ok(())
    }

    /// 完整初始化和配置
    pub async fn init_with_config(&mut self) -> Result<(), SPI::Error> {
        self.init().await?;

        self.set_clock_source(self.config.clock_source).await?;
        let sample_rate_div = calculate_sample_rate_divider(self.config.sample_rate);
        self.write_register(SMPLRT_DIV, sample_rate_div).await?;
        self.write_register(CONFIG, self.config.dlpf_config as u8)
            .await?;
        self.set_gyro_scale(self.config.gyro_scale).await?;
        self.set_accel_scale(self.config.accel_scale).await?;
        if self.config.enable_interrupts {
            self.enable_interrupts().await?;
        }

        Ok(())
    }

    /// 校准初始化（包含传感器校准）
    pub async fn calibrate_init(&mut self) -> Result<(), SPI::Error> {
        self.init_with_config().await?;
        self.calibrate_sensors().await?;
        Ok(())
    }
}
