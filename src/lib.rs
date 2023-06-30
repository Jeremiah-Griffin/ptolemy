use std::error::Error;
use std::fmt::{Debug, Display};
use serde::{Serialize, Deserialize};
use serde::de::Unexpected::Bool;


pub trait PtolemyFallible {
    fn error() -> Self;
}

#[derive(Serialize, Deserialize)]
///if is_err map value must be empty.
/// if not, the map is keyed by place names, and the values are that place's ID.
///
/// However, recognizing that a programmer may reasonably err, the constructor of
/// PLaceNameId *wilL* take a value even if it is supposed to return an error.
/// I'd rather wasted bandwidth than a failed assertion.
pub struct PlaceNameId{
    pub is_err: bool,
    pub names_ids: Vec<(String, String)>,
}
impl PtolemyFallible for PlaceNameId{
    fn error() -> Self{
        PlaceNameId{is_err: true, names_ids: vec![]}
    }
}



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

