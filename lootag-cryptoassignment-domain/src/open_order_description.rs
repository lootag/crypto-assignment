#[derive(Debug)]
pub struct OpenOrderDescription {
    pair: String,
    position: Position,
    order_type: OrderType,
    price: f32,
    price2: f32,
    leverage: String,
    order: String,
    close: String,
}

#[derive(Debug)]
pub enum Position {
    Buy,
    Sell,
}

#[derive(Debug)]
pub enum OrderType {
    Market,
    Limit,
}

pub fn new(
    pair: String,
    position: Position,
    order_type: OrderType,
    price: f32,
    price2: f32,
    leverage: String,
    order: String,
    close: String,
) -> Result<OpenOrderDescription, std::io::Error> {
    Ok(OpenOrderDescription {
        pair: pair,
        position: position,
        order_type: order_type,
        price: price,
        price2: price2,
        leverage: leverage,
        order: order,
        close: close,
    })
}
