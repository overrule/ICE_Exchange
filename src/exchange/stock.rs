use crate::orderbook::Orderbook;
use crate::user::UserID;
struct StockValue{
    underlying_handle: String,
}
impl StockValue{
    pub fn get_dividends() -> u32{
        // TODO: impl CF API call and decide dividend function
        0
    }
}
pub struct Stock{
    orderbook: Orderbook,
    stock_value: StockValue,
    instrument_id: i32,
    holdings: [i32; crate::MAX_USERS]
}
impl Stock{
    pub fn change_holdings_by(&mut self, user: UserID, delta_stock: i32){
        self.holdings[user.user_id] += delta_stock;
    }
}