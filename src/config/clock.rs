// 电源管理配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    Normal = 0x00,
    Sleep = 0x40,
    Cycle = 0x20,
    Standby = 0x80,
}

// 时钟源配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClockSource {
    Internal = 0x00,
    PllGyroX = 0x01,
    PllGyroY = 0x02,
    PllGyroZ = 0x03,
    PllExt32kHz = 0x04,
    PllExt19MHz = 0x05,
    Reserved = 0x06,
    StopClock = 0x07,
}
