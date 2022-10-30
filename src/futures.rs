use orderbook::Orderbook;
use orderbook::TradeResult;
use stock::Stock;
use stock::MAX_USERS;
struct Future{
    underlying_stock: Stock,
    orderbook: Orderbook,
    total_stock_obligation: [i32; crate::MAX_USERS],
    total_cash_obligation: [i64; crate::MAX_USERS]
}
impl Future{
    fn trade_confirmation(tr: TradeResult){
        total_cash_obligation[tr.user1] += tr.price;
        total_cash_obligation[tr.user2] -= tr.price;
        total_stock_obligation[tr.user1] -= tr.size;
        total_stock_obligation[tr.user2] += tr.size;
    }
    fn close_positions(){
        // TODO: Set all obligations to 0, and execute the trades. 
        // Assume everyone has enough stocks (The brokers should have enough stocks to cover positions)
    }
}