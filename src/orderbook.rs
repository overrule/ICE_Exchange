use crate::user::UserID;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::hash::{Hash};
use std::cmp::min;
#[derive(Copy, Clone)]
pub enum LimitOrderType{
    Bid,
    Ask,
}
#[derive(Copy, Clone)]
pub struct LimitOrder{
    limit_order_type: LimitOrderType,
    price: u32,
    size: u32,
    filled: u32,
    cancelled: bool
}
pub enum OrderResult{
    Successful,
    TradeOutOfBounds,
    OrderAlreadyExists
}
pub enum CancelResult{
    Successful,
    TradeDoesNotExist,
    TradeIsAlreadyFilled
}
//user1.stock += size, user1.cash -= price
pub struct TradeResult{
    pub user1: UserID,
    pub user2: UserID,
    pub size: u32,
    pub price: i64
}
#[derive(Copy, Clone)]
pub struct Trade{
    tradeID: TradeID,
    user_order: LimitOrder
}
impl Hash for Trade {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tradeID.hash(state)
    }
}
impl Eq for Trade {}
impl PartialEq for Trade {
    fn eq(&self, other: &Self) -> bool {
        self.tradeID == other.tradeID
    }
}
#[derive(Copy, Clone, PartialEq, Eq)]
struct TradeID{
    user_id: UserID,
    order_number: u32
}
impl Hash for TradeID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.user_id.user_id, self.order_number).hash(state)
    }
}
#[derive(Default)]
pub struct Orderbook{
    bid_pq: PriorityQueue<Trade, u32>,
    ask_pq: PriorityQueue<Trade, Reverse<u32>>,
    trade_ptr: HashMap<TradeID, Trade>,
}
impl Orderbook{
    pub fn new() -> Orderbook {
        Self::default()
    }
    fn execute_trades<F : FnMut(TradeResult)> (&mut self, execute_trade: &mut F){
        while self.bid_pq.len() > 0 {
            let (best_bid, price) = self.bid_pq.peek_mut().unwrap();
            *best_bid = self.trade_ptr.get(&best_bid.tradeID).unwrap().clone();
            if best_bid.user_order.cancelled == false {
                break;
            }
            self.bid_pq.pop();
        }
        while self.ask_pq.len() > 0 {
            let (best_ask, price) = self.ask_pq.peek_mut().unwrap();
            *best_ask = self.trade_ptr.get(&best_ask.tradeID).unwrap().clone();
            if best_ask.user_order.cancelled == false {
                break;
            }
            self.ask_pq.pop();
        }
        if self.bid_pq.len() == 0 || self.ask_pq.len() == 0 {
            return;
        }
        let (best_bid, &bid_price) = self.bid_pq.peek_mut().unwrap();
        let (best_ask, &Reverse(ask_price)) = self.ask_pq.peek_mut().unwrap();
        if bid_price < ask_price {
            return;
        }
        let mid_price = ((bid_price + ask_price) / 2) as i64;
        let trade_qty = min(best_bid.user_order.size - best_bid.user_order.filled, 
                                best_ask.user_order.size - best_ask.user_order.filled);
        let trade_result = TradeResult{user1: best_bid.tradeID.user_id, user2: best_ask.tradeID.user_id, price: mid_price, size: trade_qty};
        execute_trade(trade_result);
        let update_bid = self.trade_ptr.get_mut(&best_bid.tradeID).unwrap();
        let update_ask = self.trade_ptr.get_mut(&best_ask.tradeID).unwrap();
        update_ask.user_order.filled += trade_qty;
        update_bid.user_order.filled += trade_qty;
        if update_ask.user_order.filled < update_ask.user_order.size{
            *best_ask = *update_ask;
        }
        if update_bid.user_order.filled < update_ask.user_order.size{
            *best_bid = *update_bid;
        }

    }
    pub fn add_trade(&mut self, user_id : UserID, user_order : LimitOrder, order_number : u32) -> OrderResult{
        // TODO: Add checks that may return OrderResult::TradeOutOfBounds and OrderResult::OrderAlreadyExists
        let c_trade = Trade {
            tradeID: TradeID{ user_id: user_id, order_number: order_number },
            user_order: user_order
        };
        if self.trade_ptr.contains_key(&c_trade.tradeID) {
            return OrderResult::OrderAlreadyExists;
        }
        self.trade_ptr.insert(c_trade.tradeID, c_trade);
        match c_trade.user_order.limit_order_type {
            LimitOrderType::Bid => { self.bid_pq.push(c_trade, c_trade.user_order.price); },
            LimitOrderType::Ask => { self.ask_pq.push(c_trade, Reverse(c_trade.user_order.price)); },
        }
        OrderResult::Successful
    }
    pub fn cancel_trade(&mut self, user_id : UserID,  order_number : u32) -> CancelResult{
        // Search for order hashmap to determine order type, search in corresponding priority queue, remove order? -Colin
        let tradeID = TradeID {user_id: user_id, order_number: order_number};
        if !self.trade_ptr.contains_key(&tradeID){
            return CancelResult::TradeDoesNotExist;
        }
        let trade = &mut self.trade_ptr.get_mut(&tradeID).unwrap().clone();
        trade.user_order.cancelled = true;
        // TODO: Change trade priority to highest in corresponding priority queue and pop trade
        CancelResult::Successful
    }
}


