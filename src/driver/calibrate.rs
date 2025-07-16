use crate::Mpu6500;

use embassy_time::Timer;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::spi::SpiBus;

impl<SPI, CS> Mpu6500<SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin,
{
    /// 校准传感器
    pub async fn calibrate_sensors(&mut self) -> Result<(), SPI::Error> {
        self.calibrate_accel().await?;
        self.calibrate_gyro().await?;
        Ok(())
    }

    /// 校准加速度计
    pub async fn calibrate_accel(&mut self) -> Result<(), SPI::Error> {
        let mut sum = (0i32, 0i32, 0i32);
        for _ in 0..100 {
            let (x, y, z) = self.read_accel_raw().await?;
            sum.0 += x as i32;
            sum.1 += y as i32;
            sum.2 += z as i32;
            Timer::after_micros(500).await;
        }
        let avg = (
            (sum.0 / 100) as i16,
            (sum.1 / 100) as i16,
            ((sum.2 / 100) - 16384) as i16, // 减去重力加速度
        );
        self.accel_offset = avg;
        Ok(())
    }

    /// 校准陀螺仪
    pub async fn calibrate_gyro(&mut self) -> Result<(), SPI::Error> {
        let mut sum = (0i32, 0i32, 0i32);
        for _ in 0..100 {
            let (x, y, z) = self.read_gyro_raw().await?;
            sum.0 += x as i32;
            sum.1 += y as i32;
            sum.2 += z as i32;
            Timer::after_micros(500).await;
        }
        let avg = (
            (sum.0 / 100) as i16,
            (sum.1 / 100) as i16,
            (sum.2 / 100) as i16,
        );
        self.gyro_offset = avg;
        Ok(())
    }
}
