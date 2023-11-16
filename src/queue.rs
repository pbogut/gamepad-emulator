use std::time::Instant;

const LENGTH: usize = 1000;
const TICK: u128 = 5;

pub struct Queue {
    items: [i32; LENGTH],
    last: Instant,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            items: [0; LENGTH],
            last: Instant::now(),
        }
    }
    pub fn sum(&self) -> i32 {
        self.items.iter().sum()
    }

    pub fn push(&mut self, val: i32) {
        if self.last.elapsed().as_millis() < TICK {
            self.items[0] = self.items[0] + val;
        } else {
            self.last = Instant::now();
            for i in 1..LENGTH {
                self.items[i] = self.items[i - 1];
            }
            self.items[0] = val;
        }
    }
}
