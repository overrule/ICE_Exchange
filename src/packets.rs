use md5::{Md5, Digest};
use std::mem;
enum OrderType {
    BUY,
    SELL,
}
struct PacketHeader {
    code: u32,
    length: u32,
    hash: Md5,
}
impl PacketHeader {
    pub fn generate_header<T>() -> Self {
        PacketHeader {
            code: 0,
            length: mem::size_of::<T>() as u32,
            hash: Md5::new(),
        }
    }
}
struct OrderRequestPayload {
    order_number: i32,
    user_id: u32,
    order_type: OrderType,
    volume: u32,
    price: u32,
}
struct OrderRequest {
    header: PacketHeader,
    payload: OrderRequestPayload,
}
impl OrderRequest {
    pub fn new(order_number: i32, user_id: u32, order_type: OrderType, volume: u32, price: u32) -> Self {
        let payload = OrderRequestPayload {
            order_number,
            user_id,
            order_type,
            volume,
            price,
        };
        let header = PacketHeader::generate_header::<OrderRequest>();
        return OrderRequest {
            header,
            payload,
        };
    }
}
struct OrderConfirmPayload {
    order_number: i32,
    user_id: u32,
    filled_volume: u32,
}
struct OrderConfirm {
    header: PacketHeader,
    payload: OrderConfirmPayload
}
impl OrderConfirm {
    pub fn new(order_number: i32, user_id: u32, filled_volume: u32) -> Self {
        let payload = OrderConfirmPayload {
            order_number,
            user_id,
            filled_volume,
        };
        let header = PacketHeader::generate_header::<OrderConfirm>();
        return OrderConfirm {
            header,
            payload,
        }
    }
}
struct OrderCancelPayload {
    order_number: i32,
    user_id: u32,
}
struct OrderCancel {
    header: PacketHeader,
    payload: OrderCancelPayload,
}
impl OrderCancel {
    pub fn new(order_number: i32, user_id: u32) -> Self {
        let payload = OrderCancelPayload {
            order_number,
            user_id,
        };
        let header = PacketHeader::generate_header::<OrderConfirm>();
        return OrderCancel {
            header,
            payload,
        };
    }
}
struct LoginRequestPayload {
    user_id: u32,
    password: u32,
}
struct LoginRequest {
    header: PacketHeader,
    payload: LoginRequestPayload
}
impl LoginRequest {
    pub fn new(user_id: u32, password: u32) -> Self {
        let payload = LoginRequestPayload {
            user_id,
            password,
        };
        let header = PacketHeader::generate_header::<LoginRequest>();
        return LoginRequest {
            header,
            payload,
        };
    }
}