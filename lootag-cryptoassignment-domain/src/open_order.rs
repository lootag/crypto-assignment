use std::io::ErrorKind;

use crate::{currency_amount::CurrencyAmount, open_order_description::OpenOrderDescription};

#[derive(Debug)]
pub struct OpenOrder {
    identifier: String,
    refid: Option<String>,
    userref: u32,
    status: OrderStatus,
    opentm: f32,
    starttm: f32,
    expiretm: f32,
    description: OpenOrderDescription,
    volume: CurrencyAmount,
    vol_exec: CurrencyAmount,
    cost: CurrencyAmount,
    fee: CurrencyAmount,
    price: CurrencyAmount,
    stopprice: CurrencyAmount,
    limitprice: CurrencyAmount,
    misc: String,
    oflags: String,
    trades: Vec<String>,
}

#[derive(Debug)]
pub enum OrderStatus {
    Open,
    Closed,
}

pub fn new(
    identifier: String,
    refid: Option<String>,
    userref: u32,
    status: OrderStatus,
    opentm: f32,
    starttm: f32,
    expiretm: f32,
    description: OpenOrderDescription,
    volume: CurrencyAmount,
    vol_exec: CurrencyAmount,
    cost: CurrencyAmount,
    fee: CurrencyAmount,
    price: CurrencyAmount,
    stopprice: CurrencyAmount,
    limitprice: CurrencyAmount,
    misc: String,
    oflags: String,
    trades: Vec<String>,
) -> Result<OpenOrder, std::io::Error> {
    let _ = validate_order_status(&status);
    Ok(OpenOrder {
        identifier: identifier,
        refid: refid,
        userref: userref,
        status: status,
        opentm: opentm,
        starttm: starttm,
        expiretm: expiretm,
        description: description,
        volume: volume,
        vol_exec: vol_exec,
        cost: cost,
        fee: fee,
        price: price,
        stopprice: stopprice,
        limitprice: limitprice,
        misc: misc,
        oflags: oflags,
        trades: trades,
    })
}

fn validate_order_status(order_status: &OrderStatus) -> Result<(), std::io::Error> {
    match order_status {
        &OrderStatus::Open => Ok(()),
        _ => Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "order cannot be closed",
        )),
    }
}
