use crate::exchange::{
    Exchange,
    StockID,
    StockOptionID
};
use crate::orderbook::Orderbook;
use crate::contest_dates::ContestDate;
pub struct StockOption{
    underlying_instrument_id: StockID,
    instrument_id: StockOptionID,
    orderbook: Orderbook,
    expiry_date: ContestDate,
    position: [i32; crate::MAX_USERS]
}
impl StockOption{
    
}