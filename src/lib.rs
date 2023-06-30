use std::error::Error;
use std::fmt::{Debug, Display};
use serde::{Serialize, Deserialize};

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


#[derive(Serialize, Deserialize)]
pub struct BoolResult{
    is_err: bool,
    val: bool,

}
impl <E: Error> From<E> for BoolResult{
    fn from(value: E) -> Self {
        BoolResult{
            is_err: true,
            val: false,
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct DoubleResult{
    is_err: bool,
    val: f64,
}
impl <E: Error> From<E> for DoubleResult{
    fn from(value: E) -> Self {
        DoubleResult{
            is_err: true,
            val: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntResult{
    is_err: bool,
    val: i64,
}
impl <E: Error> From<E> for IntResult{
    fn from(value: E) -> Self {
        IntResult{
            is_err: true,
            val: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StringResult{
    is_err: bool,
    val: String,
}

impl <E: Error> From<E> for StringResult{
    fn from(value: E) -> Self {
        StringResult{
            is_err: true,
            val: String::from(""),
        }
    }
}
