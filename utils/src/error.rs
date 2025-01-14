use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnowflakeError {
    #[error("时钟回拨检测到")]
    ClockMovedBackwards(i64),
    #[error("非法的worker_id: {0}")]
    InvalidWorkerId(i32),
    #[error("系统时间错误")]
    SystemTimeError,
}
