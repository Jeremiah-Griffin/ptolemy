use serde::{Serialize, Deserialize};

///These types are used to represent mostly database models in a
///manner which doesn't leak any sensitive data or implementation details.
/// They should be generated from the client side and sent to the server side exclusively.

#[derive(Serialize, Deserialize)]
pub struct ClientConsumerModel {
    pub address: String,
    pub email: String,
    pub country_code: i16,
    pub phone: i64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClientMerchantModel{
    pub address: String,
    pub email: String,
    pub country_code: i16,
    pub phone: i64,
    pub name: String,
}

