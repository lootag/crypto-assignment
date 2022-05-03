#[derive(Clone, Debug)]
pub struct Credentials {
    api_key: String,
    private_key: String,
    otp_secret: String,
}

pub fn new(api_key: String, private_key: String, otp_secret: String) -> Credentials {
    Credentials {
        api_key: api_key,
        private_key: private_key,
        otp_secret: otp_secret,
    }
}

impl Credentials {
    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    pub fn private_key(&self) -> &String {
        &self.private_key
    }

    pub fn otp_secret(&self) -> &String {
        &self.otp_secret
    }
}
