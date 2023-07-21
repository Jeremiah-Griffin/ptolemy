use serde::{Serialize, Deserialize};

///These types are used to represent mostly database models in a
///manner which doesn't leak any sensitive data or implementation details.
/// They should be generated from the client side and sent to the server side exclusively.




///This train current exists to mark which types User::new() should be generic over.
pub trait ClientUserModel{}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConsumerClientModel {
    pub address: String,
    pub email: String,
    pub country_code: i16,
    pub phone: i64,
    pub first_name: String,
    pub last_name: String,
}
impl ClientUserModel for ConsumerClientModel{}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MerchantClientModel{
    pub address: String,
    pub email: String,
    pub country_code: i16,
    pub phone: i64,
    pub name: String,
}
impl ClientUserModel for MerchantClientModel{}