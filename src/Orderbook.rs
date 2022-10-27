use std::collections::HashMap;
use std::collections::hash_map::Entry;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::hash::{Hash};

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
}
pub enum TradeResult{
    Successful,
    TradeOutOfBounds,
    OrderAlreadyExists
}
pub enum CancelResult{
    Successful,
    TradeDoesNotExist,
    TradeIsAlreadyFilled
}
#[derive(Copy, Clone)]
pub struct Trade{
    user_id: u32,
    user_order: LimitOrder,
    order_number: u32,
}
impl Hash for Trade {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let x: u64 = (self.user_id as u64) * (1<<32) + (self.order_number as u64);
        x.hash(state)
    }
}
impl Eq for Trade {}
impl PartialEq for Trade {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id && self.order_number == other.order_number
    }
}

pub struct Orderbook{
    bid_pq: PriorityQueue<Trade, u32>,
    ask_pq: PriorityQueue<Trade, Reverse<u32>>,
    trade_ptr: HashMap<u64, Trade>,
}
impl Orderbook{
    pub fn new() -> Orderbook {
        let mut orderbook = Orderbook {
            bid_pq: PriorityQueue::new(),
            ask_pq: PriorityQueue::new(),
            trade_ptr: HashMap::new(),
        };
        orderbook
    }

    pub fn add_trade(&mut self, user_id : u32, user_order : LimitOrder, order_number : u32) -> TradeResult{
        // TODO: Add checks that may return TradeResult::TradeOutOfBounds and TradeResult::OrderAlreadyExists
        let c_trade = Trade {
            user_id,
            user_order,
            order_number
        };
        let trade_hash: u64 = (user_id as u64) * (1<<32) + (c_trade.order_number as u64);
        if self.trade_ptr.contains_key(&trade_hash) {
            return TradeResult::OrderAlreadyExists;
        }
        self.trade_ptr.insert(trade_hash, c_trade);
        match c_trade.user_order.limit_order_type {
            LimitOrderType::Bid => { self.bid_pq.push(c_trade, c_trade.user_order.price); },
            LimitOrderType::Ask => { self.ask_pq.push(c_trade, Reverse(c_trade.user_order.price)); },
        }
        TradeResult::Successful
    }
    pub fn cancel_trade(&mut self, user_id : u32,  order_number : u32) -> CancelResult{
        // Search for order hashmap to determine order type, search in corresponding priority queue, remove order? -Colin
        let trade_hash: u64 = (user_id as u64) * (1<<32) + (order_number as u64);
        if !self.trade_ptr.contains_key(&trade_hash){
            return CancelResult::TradeDoesNotExist;
        }
        let trade: &Trade = self.trade_ptr.get(&trade_hash).unwrap();
        // TODO: Change trade priority to highest in corresponding priority queue and pop trade
        CancelResult::Successful
    }
}


