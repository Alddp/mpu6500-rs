// 中断配置
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptType {
    DataReady = 0x01,
    FifoOverflow = 0x10,
    Motion = 0x40,
}
