use std::{thread::sleep, time::{Duration, Instant}};
use rand_distr::{Normal, Distribution};
use once_cell::sync::Lazy;

static NORMAL_DIST: Lazy<Normal<f64>> = Lazy::new(||Normal::new(0.0, 1.0).unwrap());
pub fn randn() -> f64 {
    NORMAL_DIST.sample(&mut rand::thread_rng())
}



pub fn stabilize_framerate(frame_start: Instant, target_frame_duration: Duration) {
    let frame_duration = frame_start.elapsed();
    if frame_duration < target_frame_duration {
        sleep(target_frame_duration - frame_duration);
    }
}