use std::io::ErrorKind;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ServerTime {
    unixtime: u64,
    rfc1123: String,
}

pub fn new(unixtime: u64, rfc1123: String) -> Result<ServerTime, std::io::Error> {
    new_impl(unixtime, rfc1123, now)
}

fn new_impl(
    unixtime: u64,
    rfc1123: String,
    now: fn() -> u64,
) -> Result<ServerTime, std::io::Error> {
    let _ = validate_unix_time_rfc_equivalence(unixtime, &rfc1123)?;
    let _ = validate_unix_time_is_not_in_the_future(unixtime, now)?;
    let _ = validate_unix_time_is_not_older_than_ten_seconds(unixtime, now)?;
    Ok(ServerTime {
        unixtime: unixtime,
        rfc1123: rfc1123,
    })
}

fn now() -> u64 {
    Utc::now().timestamp() as u64
}

fn validate_unix_time_rfc_equivalence(
    unixtime: u64,
    rfc1123: &String,
) -> Result<(), std::io::Error> {
    let dt = DateTime::parse_from_rfc2822(rfc1123)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;
    if dt.timestamp() as u64 == unixtime {
        Ok(())
    } else {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "unix time and rfc1123 do not match!",
        ))
    }
}

fn validate_unix_time_is_not_in_the_future(
    unixtime: u64,
    now: fn() -> u64,
) -> Result<(), std::io::Error> {
    if unixtime > now() {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "the api returned a timestamp from the future",
        ))
    } else {
        Ok(())
    }
}

fn validate_unix_time_is_not_older_than_ten_seconds(
    unixtime: u64,
    now: fn() -> u64,
) -> Result<(), std::io::Error> {
    let maximum_tolerance_in_seconds = 10;
    if unixtime < now() - maximum_tolerance_in_seconds {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "the api returned a timestamp which is too old",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::new_impl;

    #[test]
    pub fn should_new_return_ok_when_all_validation_criteria_are_satisfied() {
        //Arrange
        let unixtime = 1616336594;
        let rfc1123 = String::from("Sun, 21 Mar 21 14:23:14 +0000");

        //Act
        let result = new_impl(unixtime, rfc1123, now_mock);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    pub fn should_new_return_error_when_rfc1123_and_unixtime_are_not_equivalent() {
        //Arrange
        let unixtime = 1616336594;
        let rfc1123 = String::from("Mon, 22 Mar 21 14:23:14 +0000");

        //Act
        let result = new_impl(unixtime, rfc1123, now_mock);

        //Assert
        assert!(result.is_err());
    }

    #[test]
    pub fn should_new_return_error_when_timestamp_is_in_the_future() {
        //Arrange
        let unixtime = 1616336595;
        let rfc1123 = String::from("Sun, 21 Mar 21 14:23:15 +0000");

        //Act
        let result = new_impl(unixtime, rfc1123, now_mock);

        //Assert
        assert!(result.is_err());
    }

    #[test]
    pub fn should_new_return_error_when_timestamp_is_older_than_10_seconds() {
        //Arrange
        let unixtime = 1616336583;
        let rfc1123 = String::from("Sun, 21 Mar 21 14:23:03 +0000");

        //Act
        let result = new_impl(unixtime, rfc1123, now_mock);

        //Assert
        assert!(result.is_err());
    }

    fn now_mock() -> u64 {
        1616336594
    }
}
