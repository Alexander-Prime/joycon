use std::ops::{Add, Div, Mul, Sub};

pub fn map_range<T>(input: T, (in_start, in_end): (T, T), (out_start, out_end): (T, T)) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    let in_size = in_end - in_start;
    let out_size = out_end - out_start;
    out_start + out_size * (input - in_start) / in_size
}
