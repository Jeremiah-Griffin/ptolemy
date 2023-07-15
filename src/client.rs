///These types are used to represent mostly database models in a
///manner which doesn't leak any sensitive data or implementation details.
/// They should be generated from the client side and sent to the server side exclusively.

pub struct ConsumerModel{
    pub address: String,
    pub email: String,
    pub phone: u64,
    pub first_name: String,
    pub last_name: String,
}

pub struct MerchantModel{
    pub address: String,
    pub email: String,
    pub phone: u64,
    pub name: String,
}

