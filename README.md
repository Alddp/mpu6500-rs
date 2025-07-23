# mpu6500

高效、易用的 MPU6500 六轴传感器 Rust 驱动库，支持 async/await，适用于嵌入式平台。

## 特性

- 支持 async/await，适配 Embassy/RTIC 等异步框架
- 支持自定义加速度计/陀螺仪量程、DLPF、采样率等
- 支持加速度计/陀螺仪校准
- 支持 FIFO、中断等高级功能
- 兼容 [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits
- 无需std，适合no_std环境

## 快速开始

```rust
use mpu6500::{Mpu6500, config::*};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // 初始化 SPI/CS（请替换为你的外设）
    let spi = ...;
    let cs = ...;

    // 推荐配置
    let config = ConfigBuilder::default()
        .accel_scale(AccelScale::Scale2G)
        .gyro_scale(GyroScale::Scale250)
        .dlpf_config(DlpfConfig::Bandwidth42Hz)
        .sample_rate(1000)
        .build();

    let mut mpu = Mpu6500::new(spi, cs, config);
    mpu.init_with_config().await.unwrap();
    mpu.calibrate_sensors().await.unwrap();

    loop {
        let data = mpu.read_all().await.unwrap();
        // 处理 data.accel, data.gyro, data.temp
    }
}
```

```rust
#[embassy_executor::task]
async fn mpu_task(
    spi: Spi<'static, embassy_stm32::mode::Async>,
    cs: Output<'static>,
    mut usart: Uart<'static, embassy_stm32::mode::Blocking>,
) {
    use heapless::String;
    let mut formatted_message = String::<128>::new();
    let config = ConfigBuilder::default().build();
    let mut mpu = Mpu6500::new(spi, cs, config);
    match mpu.init_with_config().await {
        Ok(_) => info!("MPU6500 initialized"),
        Err(_) => {
            warn!("MPU6500 init failed");
            return;
        }
    }
    let i = mpu.who_am_i().await.unwrap();
    info!("I am {}", i);
    mpu.calibrate_sensors().await.unwrap(); //校准
    let mut last = Instant::now();
    loop {
        let now = Instant::now();
        let dt = (now - last).as_micros() as f32 / 1_000_000.0;
        last = now;
        mpu.update(dt, 0.95238).await.unwrap();
        formatted_message.clear();
        core::write!(
            &mut formatted_message,
            "angle: {:.2}, {:.2}, {:.2}\r\n",
            util::rad2deg(mpu.pitch),
            util::rad2deg(mpu.roll),
            util::rad2deg(mpu.yaw),
        )
        .unwrap();
        let message = formatted_message.as_bytes();
        usart.blocking_write(message).unwrap();
        Timer::after_millis(10).await;
    }
}

```

## API 说明

- `ConfigBuilder`：链式配置量程、DLPF、采样率等
- `Mpu6500::init_with_config()`：初始化并写入配置
- `Mpu6500::calibrate_sensors()`：校准加速度计和陀螺仪
- `Mpu6500::read_all()`：读取所有传感器数据
- `Mpu6500::enable_fifo()` / `read_fifo_data()`：FIFO操作
- `Mpu6500::enable_interrupts()` / `read_interrupt_status()`：中断操作
- 更多见 [API 文档](https://docs.rs/mpu6500)

## 依赖与兼容性

- 依赖：`embedded-hal`、`embedded-hal-async`
- 兼容：STM32/ESP32/nRF52 等支持 async/await 的平台

## 许可证

MIT OR Apache-2.0

---
