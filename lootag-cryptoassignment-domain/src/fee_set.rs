use std::{collections::BTreeMap, io::ErrorKind};

#[derive(Debug)]
pub struct FeeSet {
    pub(crate) value: BTreeMap<u32, f32>,
}

pub fn new(value: BTreeMap<u32, f32>) -> Result<FeeSet, std::io::Error> {
    fn is_fee_pair_valid(fee1: f32, fee2: f32) -> bool {
        fee1 > fee2 && fee2 >= 0.00
    }
    let fees_vec = value
        .iter()
        .map(|(_, fee)| fee.clone())
        .collect::<Vec<f32>>();
    let are_fees_valid = (0..fees_vec.len() - 1)
        .into_iter()
        .all(|idx| is_fee_pair_valid(fees_vec[idx], fees_vec[idx + 1]));
    if are_fees_valid {
        Ok(FeeSet { value: value })
    } else {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "fees need to be positive and decreasing in quantity",
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::new;

    #[test]
    fn should_new_return_okay_if_all_validation_criteria_are_satisfied() {
        //Arrange
        let fee_set_value = [(0, 0.26), (50000, 0.24), (100000, 0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();

        //Act
        let fee_set_result = new(fee_set_value);

        //Assert
        assert!(fee_set_result.is_ok());
    }

    #[test]
    fn should_new_return_error_if_fees_are_not_decreasing_in_quantity() {
        //Arrange
        let fee_set_value = [(0, 0.23), (50000, 0.24), (100000, 0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();

        //Act
        let fee_set_result = new(fee_set_value);

        //Assert
        assert!(fee_set_result.is_err());
    }

    #[test]
    fn should_new_return_error_if_a_fee_is_negative() {
        //Arrange
        let fee_set_value = [(0, 0.26), (50000, 0.24), (100000, -0.22)]
            .into_iter()
            .clone()
            .collect::<BTreeMap<u32, f32>>();

        //Act
        let fee_set_result = new(fee_set_value);

        //Assert
        assert!(fee_set_result.is_err());
    }
}
