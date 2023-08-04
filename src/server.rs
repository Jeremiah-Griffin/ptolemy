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

pub struct OrderDescriptor{
    pub is_err: bool,
    pub sixty_kg_bags_coffee: i16,
    pub sixty_kg_bags_scraps: i16,
}