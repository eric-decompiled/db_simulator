use rand::Rng;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

pub fn random_interval<R: Rng>(rng: &mut R) -> [SystemTime; 2] {
    let mut range = [random_time(rng), random_time(rng)];
    range.sort();
    range
}

pub fn random_time<R: Rng>(rng: &mut R) -> SystemTime {
    let week_of_seconds = 604800;
    SystemTime::now() - Duration::from_secs(rng.gen_range(0..week_of_seconds))
}

/// will sleep for cadence minus exec_time amount of time. Does nothing if exec_time exceeds cadence
pub fn sleep_to_cadence(cadence: Duration, exec_time: Duration) {
    let sleep_for = cadence - exec_time;
    if sleep_for > Duration::from_millis(0) {
        thread::sleep(sleep_for);
    }
}
