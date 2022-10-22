use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
enum LimitOrderType{
    BID,
    ASK,
}
struct LimitOrder{
    limit_order_type: LimitOrderType,
    price: u32,
    size: u32,
}
enum TradeResult{
    SUCCESSFUL,
    TRADE_OUT_OF_BOUNDS,
    ORDER_ALREADY_EXISTS
}
enum CancelResult{
    SUCCESSFUL,
    TRADE_DOES_NOT_EXIST,
    TRADE_IS_ALREADY_FILLED
}
struct Trade{
    user_id: int,
    user_order: LimitOrder,
    order_number: i32,
}
struct Orderbook{
    bid_pq: PriorityQueue<Trade, i32>,
    ask_pq: PriorityQueue<Trade, Reverse<i32>>,
    trade_ptr: HashMap<u64, &Trade>,
}
impl Orderbook{
    fn add_trade(&self, user_id : u32, user_order : LimitOrder, order_number : u32) -> TradeResult{
        let c_trade = Trade {
            user_id,
            user_order,
            order_number
        };
        trade_ptr.insert((user_order as u64) * (1<<32) + order_number, &c_trade);
        if c_trade.user_order.limit_order_type = LimitOrderType::BID {
            bid_pq.push(c_trade, c_trade.user_order.price);
        }
        else{
            ask_pq.push(c_trade, c_trade.user_order.price);
        }
    }
    fn cancel_trade(&self, user_id : i32,  order_number : i32) -> CancelResult{

    }
}
