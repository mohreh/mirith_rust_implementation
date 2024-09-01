use std::cmp;
use std::ops;

pub trait Numeric:
    std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + cmp::PartialOrd
    + Copy
    + Clone
    + Default
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
{
    fn one() -> Self;
    fn zero() -> Self;
    fn from_i32(value: i32) -> Self;
    fn from_i64(value: i64) -> Self;
    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;
    fn from_usize(value: usize) -> Self;
    fn to<S>(&self) -> S
    where
        S: Numeric;
}

impl Numeric for i32 {
    fn one() -> Self {
        1i32
    }

    fn zero() -> Self {
        0i32
    }

    fn from_i32(value: i32) -> Self {
        value
    }

    fn from_i64(value: i64) -> Self {
        value as i32
    }

    fn from_usize(value: usize) -> Self {
        value as i32
    }

    fn from_f32(value: f32) -> Self {
        value.trunc() as i32
    }

    fn from_f64(value: f64) -> Self {
        value.trunc() as i32
    }

    fn to<S>(&self) -> S
    where
        S: Numeric,
    {
        S::from_i32(*self)
    }
}

impl Numeric for i64 {
    fn one() -> Self {
        1i64
    }

    fn zero() -> Self {
        0i64
    }

    fn from_i32(value: i32) -> Self {
        value as i64
    }

    fn from_i64(value: i64) -> Self {
        value
    }

    fn from_usize(value: usize) -> Self {
        value as i64
    }

    fn from_f32(value: f32) -> Self {
        value.trunc() as i64
    }

    fn from_f64(value: f64) -> Self {
        value.trunc() as i64
    }

    fn to<S>(&self) -> S
    where
        S: Numeric,
    {
        S::from_i64(*self)
    }
}

impl Numeric for f32 {
    fn one() -> Self {
        1f32
    }

    fn zero() -> Self {
        0f32
    }

    fn from_i32(value: i32) -> Self {
        value as f32
    }

    fn from_i64(value: i64) -> Self {
        value as f32
    }

    fn from_usize(value: usize) -> Self {
        value as f32
    }

    fn from_f32(value: f32) -> Self {
        value
    }

    fn from_f64(value: f64) -> Self {
        value as f32
    }

    fn to<S>(&self) -> S
    where
        S: Numeric,
    {
        S::from_f32(*self)
    }
}

impl Numeric for f64 {
    fn one() -> Self {
        1f64
    }

    fn zero() -> Self {
        0f64
    }

    fn from_i32(value: i32) -> Self {
        value as f64
    }

    fn from_i64(value: i64) -> Self {
        value as f64
    }

    fn from_usize(value: usize) -> Self {
        value as f64
    }

    fn from_f32(value: f32) -> Self {
        value as f64
    }

    fn from_f64(value: f64) -> Self {
        value
    }

    fn to<S>(&self) -> S
    where
        S: Numeric,
    {
        S::from_f64(*self)
    }
}
