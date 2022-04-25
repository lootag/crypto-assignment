use std::{
    io::ErrorKind,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Nonce {
    pub(crate) value: u64,
}

pub fn new() -> Result<Nonce, std::io::Error> {
    Ok(Nonce {
        value: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?
            .as_millis() as u64,
    })
}
