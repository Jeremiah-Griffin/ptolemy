use serde::{Serialize, Deserialize};
use crate::PtolemyFallible;

#[derive(Serialize, Deserialize)]
pub struct BoolResult{
    is_err: bool,
    val: bool,
}
impl From<bool> for BoolResult{
    fn from(value: bool) -> Self {
        BoolResult{is_err: false, val: value}
    }
}
impl PtolemyFallible for BoolResult{
    fn error() -> Self {
        BoolResult{is_err: true, val: false}
    }
}


#[derive(Serialize, Deserialize)]
pub struct DoubleResult{
    is_err: bool,
    val: f64,
}
impl From<f64> for DoubleResult{
    fn from(value: f64) -> Self {
        DoubleResult{is_err: false, val: value}
    }
}
impl PtolemyFallible for DoubleResult{
    fn error() -> Self {
        DoubleResult{is_err: true, val: 0.0}
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntResult{
    is_err: bool,
    val: i64,
}
impl From<i64> for IntResult{
    fn from(value: i64) -> Self {
        IntResult{is_err: false, val: value}
    }
}
impl PtolemyFallible for IntResult{
    fn error() -> Self {
        IntResult{is_err: true, val: 0}
    }
}

#[derive(Serialize, Deserialize)]
pub struct StringResult{
    is_err: bool,
    val: String,
}
impl From<String> for StringResult{
    fn from(value: String) -> Self {
        StringResult{is_err: false, val: value}
    }
}
impl PtolemyFallible for StringResult{
    fn error() -> Self {
        StringResult{is_err: true, val: String::new()}
    }
}

