use serde::{Serialize, Deserialize};
pub mod db;
pub mod fallible_primitives;


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

