use std::f32::consts::PI;

use rand::{thread_rng, Rng};
use lazy_static::lazy_static;

pub const DELTA_TIME: f32 = 1.0 / 60.0;
pub const SQRT_OF_2: f32 = 1.41421356237f32 / 2.0;
pub static mut GLOBAL_PIPE_ID: u32 = 0;
pub const HALF_PI: f32 = PI / 2.0;
pub const HALF_SIZE: f32 = 10.0;

lazy_static! {
    pub static ref SEED: u32 = thread_rng().gen::<u32>();
}