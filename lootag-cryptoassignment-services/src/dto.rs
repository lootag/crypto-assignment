use std::{
    collections::{BTreeMap, HashMap},
    io::ErrorKind,
};

use lootag_cryptoassignment_domain::{
    currency_amount,
    fee_set::{self, FeeSet},
    leverage, margin,
    open_order::{self, OpenOrder, OrderStatus},
    open_order_description::{self, OpenOrderDescription, OrderType, Position},
    server_time::{self, ServerTime},
    xbt_usd::{self, XbtUsd},
};
use serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct ServerTimeDto {
    error: Vec<String>,
    result: ServerTimeResult,
}

impl ServerTimeDto {
    pub(crate) fn to_business(&self) -> Result<ServerTime, std::io::Error> {
        server_time::new(self.result.unixtime, self.result.rfc1123.clone())
    }
}

#[derive(Deserialize)]
pub(crate) struct ServerTimeResult {
    unixtime: u64,
    rfc1123: String,
}

#[derive(Deserialize)]
pub(crate) struct XbtUsdPairDto {
    error: Vec<String>,
    result: XbtUsdPairResult,
}

impl XbtUsdPairDto {
    pub(crate) fn to_business(&self) -> Result<XbtUsd, std::io::Error> {
        fn fees_to_business(fees: &Vec<Vec<f32>>) -> Result<FeeSet, std::io::Error> {
            let value = fees
                .into_iter()
                .map(|f| (f[0] as u32, f[1]))
                .collect::<BTreeMap<u32, f32>>();
            fee_set::new(value)
        }
        xbt_usd::new(
            self.result.XXBTZUSD.altname.clone(),
            self.result.XXBTZUSD.wsname.clone(),
            self.result.XXBTZUSD.aclass_base.clone(),
            self.result.XXBTZUSD.base.clone(),
            self.result.XXBTZUSD.aclass_quote.clone(),
            self.result.XXBTZUSD.quote.clone(),
            self.result.XXBTZUSD.lot.clone(),
            self.result.XXBTZUSD.pair_decimals,
            self.result.XXBTZUSD.lot_decimals,
            self.result.XXBTZUSD.lot_multiplier,
            leverage::new(self.result.XXBTZUSD.leverage_buy.clone())?,
            leverage::new(self.result.XXBTZUSD.leverage_sell.clone())?,
            fees_to_business(&self.result.XXBTZUSD.fees)?,
            fees_to_business(&self.result.XXBTZUSD.fees_maker)?,
            self.result.XXBTZUSD.fee_volume_currency.clone(),
            margin::new(self.result.XXBTZUSD.margin_call)?,
            margin::new(self.result.XXBTZUSD.margin_stop)?,
            self.result.XXBTZUSD.ordermin.clone(),
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct XbtUsdPairResult {
    XXBTZUSD: XbtUsdPair,
}

#[derive(Deserialize)]
pub(crate) struct XbtUsdPair {
    altname: String,
    wsname: String,
    aclass_base: String,
    base: String,
    aclass_quote: String,
    quote: String,
    lot: String,
    pair_decimals: i32,
    lot_decimals: i32,
    lot_multiplier: i32,
    leverage_buy: Vec<u32>,
    leverage_sell: Vec<u32>,
    fees: Vec<Vec<f32>>,
    fees_maker: Vec<Vec<f32>>,
    fee_volume_currency: String,
    margin_call: u32,
    margin_stop: u32,
    ordermin: String,
}

#[derive(Deserialize)]
pub(crate) struct OpenOrdersDto {
    error: Vec<String>,
    pub(crate) result: OpenOrdersResult,
}

#[derive(Deserialize)]
pub(crate) struct OpenOrdersResult {
    #[serde(flatten)]
    pub(crate) open: Option<HashMap<String, OpenOrderDto>>,
}

#[derive(Deserialize)]
pub(crate) struct OpenOrderDto {
    refid: Option<String>,
    userref: u32,
    status: String,
    opentm: f32,
    starttm: f32,
    expiretm: f32,
    descr: OpenOrderDescrDto,
    vol: String,
    vol_exec: String,
    cost: String,
    fee: String,
    price: String,
    stopprice: String,
    limitprice: String,
    misc: String,
    oflags: String,
    trades: Vec<String>,
}

impl OpenOrderDto {
    pub(crate) fn to_business(&self, identifier: &String) -> Result<OpenOrder, std::io::Error> {
        open_order::new(
            identifier.clone(),
            self.refid.clone(),
            self.userref,
            self.string_to_order_status(&self.status)?,
            self.opentm,
            self.starttm,
            self.expiretm,
            self.descr.to_business()?,
            currency_amount::new(string_to_f32(&self.vol)?)?,
            currency_amount::new(string_to_f32(&self.vol_exec)?)?,
            currency_amount::new(string_to_f32(&self.cost)?)?,
            currency_amount::new(string_to_f32(&self.fee)?)?,
            currency_amount::new(string_to_f32(&self.price)?)?,
            currency_amount::new(string_to_f32(&self.stopprice)?)?,
            currency_amount::new(string_to_f32(&self.limitprice)?)?,
            self.misc.clone(),
            self.oflags.clone(),
            self.trades.clone(),
        )
    }

    fn string_to_order_status(&self, status: &String) -> Result<OrderStatus, std::io::Error> {
        let open_string = String::from("open");
        let closed_string = String::from("closed");
        if status == &open_string {
            Ok(OrderStatus::Open)
        } else if status == &closed_string {
            Ok(OrderStatus::Closed)
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "unknown order status",
            ))
        }
    }
}

fn string_to_f32(string: &String) -> Result<f32, std::io::Error> {
    string
        .parse::<f32>()
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))
}

#[derive(Deserialize)]
pub(crate) struct OpenOrderDescrDto {
    pair: String,
    #[serde(rename(deserialize = "type"))]
    position: String,
    order_type: String,
    price: String,
    price2: String,
    leverage: String,
    order: String,
    close: String,
}

impl OpenOrderDescrDto {
    pub(crate) fn to_business(&self) -> Result<OpenOrderDescription, std::io::Error> {
        open_order_description::new(
            self.pair.clone(),
            self.string_to_position(&self.position)?,
            self.string_to_order_type(&self.order_type)?,
            string_to_f32(&self.price)?,
            string_to_f32(&self.price2)?,
            self.leverage.clone(),
            self.order.clone(),
            self.close.clone(),
        )
    }

    fn string_to_position(&self, position: &String) -> Result<Position, std::io::Error> {
        let buy_string = String::from("buy");
        let sell_string = String::from("sell");
        if position == &buy_string {
            Ok(Position::Buy)
        } else if position == &sell_string {
            Ok(Position::Sell)
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "unknown position",
            ))
        }
    }

    fn string_to_order_type(&self, order_type: &String) -> Result<OrderType, std::io::Error> {
        let limit_string = String::from("buy");
        let market_string = String::from("sell");
        if order_type == &limit_string {
            Ok(OrderType::Limit)
        } else if order_type == &market_string {
            Ok(OrderType::Market)
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "unknown position",
            ))
        }
    }
}

#[derive(Serialize)]
pub(crate) struct WebRequestDto {}
