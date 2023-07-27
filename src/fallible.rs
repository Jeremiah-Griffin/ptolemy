use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::client::TransactionStateKind;

///A subset of the Transaction type, sent over to the
/// client without leaking too much data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientTransaction{
    is_err: bool,
    id: String,
    placed: NaiveDateTime,
    fulfilled: Option<NaiveDateTime>,
    state: TransactionStateKind,
}