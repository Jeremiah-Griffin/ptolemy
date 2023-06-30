

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

impl PlaceNameId{
    ///Check struct documentation explaining why buf isn't an Option<T>
    pub fn new(is_err: bool, buf: impl Into<Vec<(String, String)>>) -> PlaceNameId {
        PlaceNameId{
            is_err,
            names_ids: buf.into(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoolResult{
    is_err: bool,
    val: bool,

}

#[derive(Serialize, Deserialize)]
pub struct DoubleResult{
    is_err: bool,
    val: f64,
}

#[derive(Serialize, Deserialize)]
pub struct IntResult{
    is_err: bool,
    val: i64,
}

#[derive(Serialize, Deserialize)]
pub struct StringResult{
    is_err: bool,
    val: String,
}
