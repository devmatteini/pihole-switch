use std::time::Duration;

pub struct PiHoleDisableTime {
    time: Option<Duration>,
}

impl PiHoleDisableTime {
    pub fn new(time: Option<Duration>) -> PiHoleDisableTime {
        PiHoleDisableTime { time }
    }

    pub fn from_secs(time: Option<u64>) -> PiHoleDisableTime {
        PiHoleDisableTime::new(time.map(Duration::from_secs))
    }

    pub fn from_duration(time: Duration) -> PiHoleDisableTime {
        PiHoleDisableTime::new(Some(time))
    }

    pub fn none() -> PiHoleDisableTime {
        PiHoleDisableTime::new(None)
    }

    pub fn as_secs(&self) -> u64 {
        self.time
            .unwrap_or_else(|| Duration::from_secs(0))
            .as_secs()
    }
}
