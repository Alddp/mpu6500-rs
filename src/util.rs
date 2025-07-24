pub use core::f32::consts::PI;

#[inline(always)]
/// 角度转弧度
pub fn deg2rad(deg: f32) -> f32 {
    deg * crate::DEG2RAD
}
#[inline(always)]
/// 弧度转角度
pub fn rad2deg(rad: f32) -> f32 {
    rad * crate::RAD2DEG
}

/// 计算采样周期 dt（秒）
/// - last: 上一次采样的时间戳（单位：微秒或毫秒）
/// - now: 当前时间戳
/// - unit_per_sec: 1秒对应的单位数（微秒=1_000_000，毫秒=1_000）
///   返回值为秒
pub fn calc_dt(last: u64, now: u64, unit_per_sec: u64) -> f32 {
    if now > last {
        (now - last) as f32 / unit_per_sec as f32
    } else {
        0.0
    }
}

/// 计算互补滤波系数 alpha
/// tau: 滤波时间常数（秒），如 0.05~0.5
/// dt: 采样周期（秒）
pub fn calc_alpha(tau: f32, dt: f32) -> f32 {
    tau / (tau + dt)
}
