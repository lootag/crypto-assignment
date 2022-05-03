use std::{io::ErrorKind};

#[derive(Debug)]
pub struct CurrencyAmount {
    value: f32,
}

pub fn new(value: f32) -> Result<CurrencyAmount, std::io::Error> {
    if value < 0.0000 {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "currency amount cannot be negative",
        ))
    } else {
        Ok(CurrencyAmount { value: value })
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn should_new_return_ok_if_all_validation_criteria_are_satisfied() {
        //Arrange
        let value = 30.0;

        //Act
        let amount = new(value);

        //Assert
        assert!(amount.is_ok());
    }

    #[test]
    fn should_new_return_error_if_value_is_negative() {
        //Arrange
        let value = -30.0;

        //Act
        let amount = new(value);

        //Assert
        assert!(amount.is_err());
    }
}
