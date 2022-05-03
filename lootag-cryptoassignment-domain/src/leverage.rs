use std::io::ErrorKind;

#[derive(Debug)]
pub struct Leverage {
    pub(crate) value: Vec<u32>,
}

pub fn new(value: Vec<u32>) -> Result<Leverage, std::io::Error> {
    let max_leverage_value = 5;
    let min_leverage_value = 1;
    let is_leverage_valid = value
        .iter()
        .all(|l| l <= &max_leverage_value && l >= &min_leverage_value);
    if is_leverage_valid {
        Ok(Leverage { value: value })
    } else {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "leverage needs to be between 1 and 5",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn should_new_return_ok_if_all_validation_criteria_are_satisfied() {
        //Arrange
        let leverage_value = vec![1, 2, 3, 4, 5];

        //Act
        let leverage = new(leverage_value);

        //Assert
        assert!(leverage.is_ok());
    }

    #[test]
    fn should_new_return_error_if_leverage_is_less_than_1() {
        //Arrange
        let leverage_value = vec![0, 2, 3, 4, 5];

        //Act
        let leverage = new(leverage_value);

        //Assert
        assert!(leverage.is_err());
    }

    #[test]
    fn should_new_return_error_if_leverage_is_larger_than_5() {
        //Arrange
        let leverage_value = vec![1, 2, 3, 4, 6];

        //Act
        let leverage = new(leverage_value);

        //Assert
        assert!(leverage.is_err());
    }
}
