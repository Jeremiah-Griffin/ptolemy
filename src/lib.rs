use serde::{Serialize, Deserialize};
pub mod fallible_primitives;
pub mod client;
mod fallible;


pub trait PtolemyFallible: Serialize{
    fn error() -> Self;
}

pub trait PtolemyFallibleBlanket{
    fn error_json() -> String;
}

impl  <T: PtolemyFallible> PtolemyFallibleBlanket for T{
    fn error_json() -> String {
        serde_json::to_string(&T::error()).unwrap()
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
///Actix requires a response no matter what. The value
/// of "val" is somewhat up to the caller of this function. Usually,
/// it should be that true: is a successful acceptance of the previous request, and false
/// is some sort of "soft" failure.
pub struct ThothAck{
    pub val: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
impl From<Vec<(String, String)>> for PlaceNameId{
    fn from(value: Vec<(String, String)>) -> Self {PlaceNameId{is_err: false,  names_ids: value }}
}

/*
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction{
    pub is_err: bool,
    pub placed: String,
    pub fulfilled: String,
    pub sixty_kg_bags_coffee: u32,
    pub sixty_kg_bags_scraps: u32,
}
impl PtolemyFallible for Transaction{
    fn error() -> Self {
        Transaction{
            is_err: true,
            placed: String::default(),
            fulfilled: String::default(),
            sixty_kg_bags_coffee: 0,
            sixty_kg_bags_scraps: 0,
        }
    }
}
*/
