use chrono::{NaiveDate, NaiveDateTime, TimeDelta};

#[derive(Debug)]
pub struct GameClock {
    now: NaiveDateTime,
    // small accumulator to carry sub seconds
    acc: f32,
    scale: f32,
    paused: bool,
}

const DEFAULT_TIME_SCALING: f32 = 60.;

impl Default for GameClock {
    fn default() -> Self {
        Self::new(
            NaiveDate::from_ymd_opt(2026, 7, 8)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap(),
            DEFAULT_TIME_SCALING,
        )
    }
}

impl GameClock {
    pub fn new(start: NaiveDateTime, scale: f32) -> Self {
        Self {
            now: start,
            acc: 0.,
            scale,
            paused: false,
        }
    }

    pub(super) fn tick(&mut self, delta: f32) {
        if self.paused {
            return;
        }

        self.acc += delta * self.scale;
        let floor = self.acc.floor();
        self.acc -= floor;
        self.advance(TimeDelta::seconds(floor as i64));
    }

    pub fn advance(&mut self, delta: TimeDelta) {
        self.now += delta;
    }

    pub fn pause(&mut self) {
        self.paused = false;
    }

    pub fn resume(&mut self) {
        self.paused = true;
    }

    pub fn now(&self) -> NaiveDateTime {
        self.now
    }
}
