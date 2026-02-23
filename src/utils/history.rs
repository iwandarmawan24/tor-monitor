// src/utils/history.rs
// Ring buffer for storing history data

/// Ring buffer for storing history data
/// Automatically overwrites old data when full
pub struct History {
    data: Vec<f32>,
    capacity: usize,
    write_pos: usize,
    len: usize,
}

impl History {
    pub fn new(capacity: usize) -> Self {
        History {
            data: vec![0.0; capacity],
            capacity,
            write_pos: 0,
            len: 0,
        }
    }

    /// Push a new value
    pub fn push(&mut self, value: f32) {
        self.data[self.write_pos] = value;
        self.write_pos = (self.write_pos + 1) % self.capacity;
        if self.len < self.capacity {
            self.len += 1;
        }
    }

    /// Get all data in chronological order
    pub fn get_data(&self) -> Vec<f32> {
        if self.len < self.capacity {
            // Not full yet, get from beginning
            self.data[..self.len].to_vec()
        } else {
            // Full, order from write_pos (oldest) to write_pos-1 (newest)
            let mut result = Vec::with_capacity(self.capacity);
            for i in 0..self.capacity {
                let idx = (self.write_pos + i) % self.capacity;
                result.push(self.data[idx]);
            }
            result
        }
    }

    /// Get last N items
    #[allow(dead_code)]
    pub fn get_last(&self, n: usize) -> Vec<f32> {
        let data = self.get_data();
        let start = data.len().saturating_sub(n);
        data[start..].to_vec()
    }

    /// Get last value
    #[allow(dead_code)]
    pub fn last(&self) -> Option<f32> {
        if self.len == 0 {
            None
        } else {
            let idx = if self.write_pos == 0 {
                self.capacity - 1
            } else {
                self.write_pos - 1
            };
            Some(self.data[idx])
        }
    }

    /// Average of all values
    #[allow(dead_code)]
    pub fn average(&self) -> f32 {
        if self.len == 0 {
            return 0.0;
        }
        let sum: f32 = self.get_data().iter().sum();
        sum / self.len as f32
    }

    /// Max value
    #[allow(dead_code)]
    pub fn max(&self) -> f32 {
        self.get_data().iter().cloned().fold(0.0, f32::max)
    }

    /// Min value
    #[allow(dead_code)]
    pub fn min(&self) -> f32 {
        self.get_data().iter().cloned().fold(f32::MAX, f32::min)
    }
}

/// Multi-track history (for network/disk with in/out)
#[allow(dead_code)]
pub struct DualHistory {
    pub incoming: History,
    pub outgoing: History,
}

#[allow(dead_code)]
impl DualHistory {
    pub fn new(capacity: usize) -> Self {
        DualHistory {
            incoming: History::new(capacity),
            outgoing: History::new(capacity),
        }
    }

    pub fn push(&mut self, incoming: f32, outgoing: f32) {
        self.incoming.push(incoming);
        self.outgoing.push(outgoing);
    }
}
