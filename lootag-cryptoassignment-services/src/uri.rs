pub struct Uri {
    value: String,
}

pub fn new(value: String) -> Uri {
    Uri { value: value }
}

impl Uri {
    pub fn value(&self) -> &String {
        &self.value
    }
}
