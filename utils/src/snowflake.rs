use crate::error::SnowflakeError;
use std::{
    sync::Mutex,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// 常量定义
const START_TIMESTAMP: i64 = 1735660800;
const SEQUENCE_BITS: i32 = 12;
const WORKER_ID_BITS: i32 = 10;
const MAX_WORKER_ID: i32 = -1 ^ (-1 << WORKER_ID_BITS);
const MAX_SEQUENCE: u16 = (-1 ^ (-1 << SEQUENCE_BITS)) as u16;

/// 雪花算法ID生成器
///
/// - 41位时间戳
/// - 10位工作机器ID
/// - 12位序列号
#[derive(Debug)]
pub struct Generator {
    worker_id: i32,
    inner: Mutex<GeneratorInner>,
}

#[derive(Debug)]
struct GeneratorInner {
    sequence: u16,
    last_timestamp: i64,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            worker_id: 0,
            inner: Mutex::new(GeneratorInner {
                sequence: 0,
                last_timestamp: 0,
            }),
        }
    }
}

impl Generator {
    pub fn new(worker_id: i32) -> Result<Self, SnowflakeError> {
        if !(0..MAX_WORKER_ID).contains(&worker_id) {
            return Err(SnowflakeError::InvalidWorkerId(worker_id));
        }

        Ok(Self {
            worker_id,
            inner: Mutex::new(GeneratorInner {
                sequence: 0,
                last_timestamp: 0,
            }),
        })
    }

    fn get_current_timestamp() -> Result<i64, SnowflakeError> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .map_err(|_| SnowflakeError::SystemTimeError)
    }

    pub fn next_id(&self) -> Result<u64, SnowflakeError> {
        let mut inner = self.inner.lock().unwrap();
        let current = Self::get_current_timestamp()?;
        let timestamp = current - START_TIMESTAMP;

        if timestamp < inner.last_timestamp {
            return Err(SnowflakeError::ClockMovedBackwards(
                inner.last_timestamp - timestamp,
            ));
        }

        if timestamp == inner.last_timestamp {
            inner.sequence = (inner.sequence + 1) & MAX_SEQUENCE;
            if inner.sequence == 0 {
                // 序列号用尽，等待下一毫秒
                thread::sleep(Duration::from_millis(1));
                let new_timestamp = Self::get_current_timestamp()? - START_TIMESTAMP;
                if new_timestamp <= timestamp {
                    return Err(SnowflakeError::SystemTimeError);
                }
                inner.last_timestamp = new_timestamp;
            }
        } else {
            inner.sequence = 0;
            inner.last_timestamp = timestamp;
        }

        Ok((((timestamp & 0x1FFFFFFFFFF) << 22)
            | ((self.worker_id & 0x3FF) << 12) as i64
            | (inner.sequence as i64)) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn test_invalid_worker_id() {
        assert!(Generator::new(-1).is_err());
        assert!(Generator::new(1024).is_err());
        assert!(Generator::new(0).is_ok());
    }

    #[test]
    fn test_unique_ids() -> Result<(), SnowflakeError> {
        let generator = Generator::new(1)?;
        let mut ids = HashSet::new();
        for _ in 0..1000 {
            ids.insert(generator.next_id()?);
        }
        assert_eq!(ids.len(), 1000);
        Ok(())
    }

    #[test]
    fn test_concurrent_generation() -> Result<(), SnowflakeError> {
        let generator = Arc::new(Generator::new(1)?);
        let mut handles = vec![];

        for _ in 0..10 {
            let gen = Arc::clone(&generator);
            handles.push(thread::spawn(move || {
                let mut ids = Vec::with_capacity(1000);
                for _ in 0..1000 {
                    ids.push(gen.next_id().unwrap());
                }
                ids
            }));
        }

        let mut all_ids = HashSet::new();
        for handle in handles {
            all_ids.extend(handle.join().unwrap());
        }

        assert_eq!(all_ids.len(), 10000);
        Ok(())
    }
}
