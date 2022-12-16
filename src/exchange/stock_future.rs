mod orderbook;
use crate::orderbook::Orderbook;
use crate::orderbook::TradeResult;
use crate::stock::Stock;
use crate::user::{
    UserID,
    UserList
};
use crate::exchange::{
    StockID,
    StockFutureID,
    Exchange
};
use crate::contest_dates::ContestDate;
pub struct StockFuture{
    underlying_instrument_id: StockID,
    instrument_id: StockFutureID,
    orderbook: Orderbook,
    expiry_date: ContestDate,
    total_stock_obligation: [i32; crate::MAX_USERS],
    total_cash_obligation: [i64; crate::MAX_USERS],
}
impl StockFuture{
    pub fn trade_confirmation(&mut self, tr: TradeResult){
        self.total_cash_obligation[tr.user1.user_id] += tr.price;
        self.total_cash_obligation[tr.user2.user_id] -= tr.price;
        self.total_stock_obligation[tr.user1.user_id] -= tr.size;
        self.total_stock_obligation[tr.user2.user_id] += tr.size;
    }
    fn close_positions(&mut self, exchange: &mut Exchange, userlist: &mut UserList){
        // TODO: Set all obligations to 0, and execute the trades. 
        // Assume everyone has enough stocks (The brokers should have enough stocks to cover positions)
        let underlying_stock: &mut Stock = exchange.stock(self.underlying_instrument_id);
        for user in 0..crate::MAX_USERS{
            underlying_stock.change_holdings_by(UserID::new(user), -self.total_stock_obligation[user]);
            userlist.change_cash_by(UserID::new(user), -self.total_cash_obligation[user]);
        }
    }
}
