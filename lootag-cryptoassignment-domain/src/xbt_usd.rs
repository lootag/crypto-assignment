use std::{io::ErrorKind};

use crate::{
    fee_set::FeeSet,
    leverage::Leverage,
    margin::{Margin},
};

#[derive(Debug)]
pub struct XbtUsd {
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
    leverage_buy: Leverage,
    leverage_sell: Leverage,
    fees: FeeSet,
    fees_maker: FeeSet,
    fee_volume_currency: String,
    margin_call: Margin,
    margin_stop: Margin,
    order_min: String,
}

pub fn new(
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
    leverage_buy: Leverage,
    leverage_sell: Leverage,
    fees: FeeSet,
    fees_maker: FeeSet,
    fee_volume_currency: String,
    margin_call: Margin,
    margin_stop: Margin,
    order_min: String,
) -> Result<XbtUsd, std::io::Error> {
    let _ = validate_margins(&margin_call, &margin_stop)?;
    Ok(XbtUsd {
        altname: altname,
        wsname: wsname,
        aclass_base: aclass_base,
        base: base,
        aclass_quote: aclass_quote,
        quote: quote,
        lot: lot,
        pair_decimals: pair_decimals,
        lot_decimals: lot_decimals,
        lot_multiplier: lot_multiplier,
        leverage_buy: leverage_buy,
        leverage_sell: leverage_sell,
        fees: fees,
        fees_maker: fees_maker,
        fee_volume_currency: fee_volume_currency,
        margin_call: margin_call,
        margin_stop: margin_stop,
        order_min: order_min,
    })
}

fn validate_margins(margin_call: &Margin, margin_stop: &Margin) -> Result<(), std::io::Error> {
    if margin_call.value() < margin_stop.value() {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "margin stop cannot be larger than margin call",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{fee_set::FeeSet, leverage::Leverage, margin::Margin};

    use super::new;

    #[test]
    fn should_new_return_ok_if_all_validation_criteria_are_satisfied() {
        //Arrange
        let altname = String::from("");
        let wsname = String::from("");
        let aclass_base = String::from("");
        let base = String::from("");
        let aclass_quote = String::from("");
        let quote = String::from("");
        let lot = String::from("");
        let pair_decimals = 5;
        let lot_decimals = 8;
        let lot_multiplier = 1;
        let leverage_buy = vec![2, 3, 4, 5];
        let leverage_sell = vec![2, 3, 4, 5];
        let fees = [(0, 0.26), (50000, 0.24), (100000, 0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();
        let fees_maker = [(0, 0.26), (50000, 0.24), (100000, 0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();
        let fee_volume_currency = String::from("");
        let margin_call = 80;
        let margin_stop = 40;
        let order_min = String::from("");

        //Act
        let result = new(
            altname,
            wsname,
            aclass_base,
            base,
            aclass_quote,
            quote,
            lot,
            pair_decimals,
            lot_decimals,
            lot_multiplier,
            Leverage {
                value: leverage_buy,
            },
            Leverage {
                value: leverage_sell,
            },
            FeeSet { value: fees },
            FeeSet { value: fees_maker },
            fee_volume_currency,
            Margin { value: margin_call },
            Margin { value: margin_stop },
            order_min,
        );

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn should_new_return_error_if_margin_stop_is_above_margin_call() {
        //Arrange
        let altname = String::from("");
        let wsname = String::from("");
        let aclass_base = String::from("");
        let base = String::from("");
        let aclass_quote = String::from("");
        let quote = String::from("");
        let lot = String::from("");
        let pair_decimals = 5;
        let lot_decimals = 8;
        let lot_multiplier = 1;
        let leverage_buy = vec![2, 3, 4, 5];
        let leverage_sell = vec![2, 3, 4, 5];
        let fees = [(0, 0.26), (50000, 0.24), (100000, 0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();
        let fees_maker = [(0, 0.26), (50000, 0.24), (100000, 0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();
        let fee_volume_currency = String::from("");
        let margin_call = 80;
        let margin_stop = 90;
        let order_min = String::from("");

        //Act
        let result = new(
            altname,
            wsname,
            aclass_base,
            base,
            aclass_quote,
            quote,
            lot,
            pair_decimals,
            lot_decimals,
            lot_multiplier,
            Leverage {
                value: leverage_buy,
            },
            Leverage {
                value: leverage_sell,
            },
            FeeSet { value: fees },
            FeeSet { value: fees_maker },
            fee_volume_currency,
            Margin { value: margin_call },
            Margin { value: margin_stop },
            order_min,
        );

        //Assert
        assert!(result.is_err());
    }
}
