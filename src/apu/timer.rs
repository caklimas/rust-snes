#[derive(Clone, Copy)]
pub struct Timer {
    pub counter: u8,
    pub divider: u8,
    clock_accumulator: u8,
    internal_counter: u8,
    tick_rate: u8,
}

impl Timer {
    pub fn new(tick_rate: u8) -> Self {
        Self {
            counter: 0,
            divider: 0,
            clock_accumulator: 0,
            internal_counter: 0,
            tick_rate,
        }
    }

    pub fn tick(&mut self, spc_ticks: u8) {
        self.clock_accumulator = self.clock_accumulator.wrapping_add(spc_ticks);

        while self.clock_accumulator >= self.tick_rate {
            self.clock_accumulator = self.clock_accumulator.wrapping_sub(self.tick_rate);
            self.internal_counter = self.internal_counter.wrapping_add(1);

            if self.internal_counter == self.divider {
                self.internal_counter = 0;
                self.counter = self.counter.wrapping_add(1);
            }
        }
    }
}
