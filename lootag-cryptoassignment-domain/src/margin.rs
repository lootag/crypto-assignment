use std::io::ErrorKind;

#[derive(Debug)]
pub struct Margin {
    pub(crate) value: u32,
}

impl Margin {
    pub fn value(&self) -> u32 {
        self.value
    }
}

pub fn new(value: u32) -> Result<Margin, std::io::Error> {
    let maximimum_margin_value = 100;
    if value > maximimum_margin_value {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "margin can't be larger than 100%",
        ))
    } else {
        Ok(Margin { value: value })
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn should_new_return_ok_if_margin_value_is_less_than_100() {
        //Arrange
        let margin_value = 80;

        //Act
        let margin = new(margin_value);

        //Assert
        assert!(margin.is_ok())
    }

    #[test]
    fn should_new_return_ok_if_margin_value_is_100() {
        //Arrange
        let margin_value = 100;

        //Act
        let margin = new(margin_value);

        //Assert
        assert!(margin.is_ok())
    }

    #[test]
    fn should_new_return_error_if_margin_value_is_larger_than_100() {
        //Arrange
        let margin_value = 150;

        //Act
        let margin = new(margin_value);

        //Assert
        assert!(margin.is_err())
    }
}
