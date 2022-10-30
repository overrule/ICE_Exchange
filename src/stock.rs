mod orderbook;
use orderbook::Orderbook;
struct StockValue{
    underlying_handle: String,
}
impl StockValue{
    pub fn get_dividends() -> u32{
        // TODO: impl CF API call and decide dividend function
        0
    }
}
struct Stock{
    // TODO: Considering switching to inheritance?
    orderbook: Orderbook,
    stock_value: StockValue,
    ownership: [u32; crate::MAX_USERS]
}

