use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::client::TransactionStateKind;

///A subset of the Transaction type, sent over to the
/// client without leaking too much data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientTransaction{
    pub is_err: bool,
    pub id: String,
    pub placed: NaiveDateTime,
    pub fulfilled: Option<NaiveDateTime>,
    pub state: TransactionStateKind,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderDescriptor{
    pub is_err: bool,
    pub sixty_kg_bags_coffee: i16,
    pub sixty_kg_bags_scraps: i16,
}



#[derive(Serialize, Deserialize, Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct OrderModeller{
    pub is_err: bool,
    kind: OrderModellerKind,
    pub sixty_kg_bags_coffe: i16,
    pub sixty_kg_bags_scraps: i16,
}


///For rewrite: swithc to typestate pattern
#[derive(Serialize, Deserialize, Clone, Debug, Ord, Eq, PartialEq, PartialOrd)]
pub enum OrderModellerKind {
    ///Consumer to server posting a new order to be bid on by merchants
    ConsumerRequest,
    ///An order being bid on by merchants. This is the only kind
    MerchantBid,
}