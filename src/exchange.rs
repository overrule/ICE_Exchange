use crate::stock::Stock;
use crate::stock_future::StockFuture;
use crate::stock_option::StockOption;
enum Instrument{
    StockType(Stock),
    FutureType(StockFuture),
    OptionType(StockOption)
}
enum InstrumentID{
    StockType(StockID),
    FutureType(StockFutureID),
    OptionType(StockOptionID)
}
pub struct Exchange{
    intsrument_list: [Instrument; crate::MAX_INSTRUMENTS],
    stock_list: [Stock; crate::MAX_STOCKS],
    futures_list: [StockFuture; crate::MAX_FUTURES],
    options_list: [StockOption; crate::MAX_OPTIONS]
}
impl Exchange{
    pub fn stock(&mut self, stock_id: StockID) -> &mut Stock{
        &mut self.stock_list[stock_id.instrument_id]
    }
    pub fn future(&mut self, future_id: StockFutureID) -> &mut StockFuture{
        &mut self.futures_list[future_id.instrument_id]
    }
    pub fn option(&mut self, option_id: StockOptionID) -> &mut StockOption{
        &mut self.options_list[option_id.instrument_id]
    }
}
#[derive(Copy, Clone)]
pub struct StockID{
    instrument_id: usize
}
#[derive(Copy, Clone)]
pub struct StockFutureID{
    instrument_id: usize
}
#[derive(Copy, Clone)]
pub struct StockOptionID{
    instrument_id: usize
}