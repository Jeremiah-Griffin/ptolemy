use geoutils::Location;
use serde::{Serialize, Deserialize};

///These types are used to represent mostly database models in a
///manner which doesn't leak any sensitive data or implementation details.
/// They should be generated from the client side and sent to the server side exclusively.


///This trait current exists to mark which types User::new() should be generic over.
pub trait ClientUserModel{}

///represents all the data in a ConsumerModel (database model) that is safe
///and useful for the client to consume.
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


///represents all the data in a MerchantModel (database model) that is safe
///and useful for the client to consume.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MerchantClientModel{
    pub address: String,
    pub email: String,
    pub country_code: i16,
    pub phone: i64,
    pub name: String,
}
impl ClientUserModel for MerchantClientModel{}

///Submitted to the server
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderRequest {
    pub sixty_kg_bags_coffee: u32,
    pub sixty_kg_bags_scraps: u32,
    pub consumer_location: Location,
}