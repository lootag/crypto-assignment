use crate::nonce::Nonce;

pub enum RequestPayload {
    OpenOrders(OpenOrdersRequestPayload),
}

pub struct OpenOrdersRequestPayload {}

pub fn encode(payload: &RequestPayload, nonce: &Nonce, otp: &String) -> String {
    encode_impl(payload, nonce, otp)
}

fn encode_impl(payload: &RequestPayload, nonce: &Nonce, otp: &String) -> String {
    match payload {
        RequestPayload::OpenOrders(orders) => encode_open_orders_payload(orders, nonce, otp),
    }
}

fn encode_open_orders_payload(
    _payload: &OpenOrdersRequestPayload,
    nonce: &Nonce,
    otp: &String,
) -> String {
    format!("nonce={}&trades=true&otp={}", nonce.value.to_string(), otp)
}

#[cfg(test)]
mod tests {
    use crate::nonce::Nonce;

    use super::{encode_impl, OpenOrdersRequestPayload, RequestPayload};

    #[test]
    fn should_encode_return_correctly_encoded_payload() {
        //Arrange
        let open_orders = OpenOrdersRequestPayload {};
        let payload = RequestPayload::OpenOrders(open_orders);
        let nonce = Nonce { value: 1234567 };
        let otp = String::from("123456");

        //Act
        let encoded_payload = encode_impl(&payload, &nonce, &otp);

        //Assert
        assert_eq!(
            encoded_payload,
            String::from("nonce=1234567&trades=true&otp=123456")
        );
    }
}
