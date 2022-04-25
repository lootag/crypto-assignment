pub struct User { 
    api_key: String,
    private_key: String
}

pub fn new(
    api_key: String,
    private_key: String 
) -> User { 
    User { 
        api_key: api_key,
        private_key: private_key
    }
}

impl User { 
    pub fn api_key(&self) -> &String { 
        &self.api_key
    }

    pub fn private_key(&self) -> &String { 
        &self.private_key
    }
}
