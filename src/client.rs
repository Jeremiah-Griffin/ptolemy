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


///This is the state of the transaction as stored in the DB. This is *not* a comprehensive
/// list of states, for example, those which would be viewed on the front end. States such
/// as whether the
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TransactionStateKind{
    ///The order has been received by thoth and is being processed
    /// and batched for the couriers.
    Received,
    ///The order has been paired with all the merchants and will  be shipped
    ///once courier(s) are dispatched.
    Processing,
    Shipped,
    Delivered,
    /*
    ///The order has was shipped at idx 0, and is expected to arrive at idx 1.
    Shipped(NaiveDateTime, NaiveDateTime),

    ///The order was delivered at the enclosed NaiveDateTime,
    /// (DateTime of delivery is handled in the TransactionState struct)
    Delivered(NaiveDateTime),*/
    ///The order was cancelled by the consumer
    Cancelled,
    ///The order was refunded to the consumer
    Refunded,
    ///Thoth was not able to complete the order for some reason.
    ///Maybe not enough merchants signed on,
    ///maybe no driver accepted the order within some period of time.
    ///
    ///This should not be used for cases that can be fixed with a retry, such as
    /// a driver cancelling or somesuch. THey should only be used for truly impossible to
    /// recover from scenarios.
    CouldNotComplete

}


