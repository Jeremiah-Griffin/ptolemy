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



#[derive(Serialize, Deserialize, Clone, Debug, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub struct OrderModeller{
    pub is_err: bool,
    pub kind: OrderModellerKind,
    ///id of the "owner" of this modeller.
    ///if it is a ConsumerRequest, it is the id of the consumer.
    ///if it is a MerchantBid it is the id of th merchant.
    pub user_id: String,
    pub sixty_kg_bags_coffee: i16,
    pub sixty_kg_bags_scraps: i16,
}

impl OrderModeller{
    ///Returns err is the order is not a ConsumerRequest
    pub fn try_consumer_request(&self) -> anyhow::Result<()>{

        match self.kind == OrderModellerKind::ConsumerRequest{
            true => Ok(()),
            false => Err(anyhow::Error::msg("OrderModeller was not a consumer request."))
        }
    }

    pub fn try_merchant_bid(&self) -> anyhow::Result<()>{
        match self.kind == OrderModellerKind::MerchantBid{
            true => Ok(()),
            false => Err(anyhow::Error::msg("OrderModeller was not a consumer request."))
        }
    }

    ///returns true if these orders contain the same counts of material.
    pub fn is_order_equal(&self, other: &OrderModeller) -> bool{
        (self.sixty_kg_bags_coffee, self.sixty_kg_bags_scraps) == (other.sixty_kg_bags_coffee, other.sixty_kg_bags_scraps)
    }
}


///For rewrite: switch to typestate pattern
#[derive(Serialize, Deserialize, Clone, Debug, Ord, Eq, PartialEq, PartialOrd, Hash)]
pub enum OrderModellerKind {
    ///Consumer to server posting a new order to be bid on by merchants
    ConsumerRequest,
    ///An order being bid on by merchants. This is the only kind
    MerchantBid,
}