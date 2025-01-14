use std::sync::Mutex;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// 北京时间 2024-01-01 00:00:00
static START: i64 = 1735660800;

pub struct Generator {
    worker_id: i32,
    sequence: Mutex<u16>,
    last_timestamp: Mutex<i64>,
}

impl Default for Generator {
    fn default() -> Self {
        Self {
            worker_id: 1,
            sequence: Mutex::new(0),
            last_timestamp: Mutex::new(0),
        }
    }
}

impl Generator {
    pub fn new(worker_id: i32) -> Self {
        Generator {
            sequence: Mutex::new(0),
            last_timestamp: Mutex::new(0),
            worker_id,
        }
    }

    pub fn next_id(&self) -> u64 {
        let mut sequence = self.sequence.lock().unwrap();
        let mut last_ts = self.last_timestamp.lock().unwrap();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        if timestamp == *last_ts {
            *sequence = (*sequence + 1) & 0xfff;
            if *sequence == 0 {
                // sequence exhausted, wait for next millisecond
                thread::sleep(Duration::from_millis(1));
            }
        } else {
            *sequence = 0;
        }

        *last_ts = timestamp;

        ((((timestamp - START) & 0x1FFFFFFFFFF) << 22)
            | ((self.worker_id as i64 & 0x3FF) << 12)
            | (*sequence & 0xFFF) as i64) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn test_concurrent_unique_ids() {
        let generator = Arc::new(Generator::new(1));
        let mut handles = vec![];
        let total_ids = 1000;

        for _ in 0..10 {
            let gen = Arc::clone(&generator);
            handles.push(thread::spawn(move || {
                (0..total_ids / 10)
                    .map(|_| gen.next_id())
                    .collect::<Vec<_>>()
            }));
        }

        let mut all_ids = HashSet::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            all_ids.extend(ids);
        }

        assert_eq!(all_ids.len(), total_ids);
    }
}
