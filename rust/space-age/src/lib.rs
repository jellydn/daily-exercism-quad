#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 7_600_543.819_92
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 19_413_907.2
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 31_557_600.0
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 59_354_032.69
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 374_355_659.12
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 929_292_362.88
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 2_651_370_019.32
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / 5_200_418_560.03
    }
}
