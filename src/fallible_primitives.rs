///These types are intended for server to client communication where fallibility
/// is expected. The is_err fields for these types represents not an error in the connection
/// client to server, but of some "internal" state of the back-end (internal in this case meaning
/// any operation handled by the back end, this may well include remote process calls to other APIs
/// including google's or the shipping providers.)
use serde::{Serialize, Deserialize};
use crate::PtolemyFallible;

#[derive(Serialize, Deserialize)]
pub struct BoolResult{
    pub is_err: bool,
    pub val:  bool,
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
    pub is_err: bool,
    pub val:  f64,
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
    pub is_err: bool,
    pub val:  i64,
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
    pub is_err: bool,
    pub val:  String,
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

