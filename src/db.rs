///This module contains types that are used in the database but also shared between client and server
///All other schema members should be defined in Thoth.
use serde::{Serialize, Deserialize};
use crate::PtolemyFallible;


#[derive(Serialize, Deserialize)]
///This struct is used for collecting user data when they express
/// interest in Ceres, but are not currently able to sign up due to being geofenced out.
pub struct UserInterestForm{
    is_err: bool,
    email: String,
    ///This should be a raw place name as place IDs aren't guarunteed to be stable indefinitely,
    ///nor are they portable.
    location: String,
}

impl PtolemyFallible for UserInterestForm{
    fn error() -> Self {
        UserInterestForm{
            is_err: true,
            email: String::new(),
            location: String::new(),
        }
    }
}