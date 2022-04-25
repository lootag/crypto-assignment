use std::io::ErrorKind;

use lootag_cryptoassignment_domain::user::User;
use ring::hmac::Tag;
use sha2::{Digest, Sha256};

use crate::{nonce::Nonce, payload::{RequestPayload, encode}, uri::Uri};

pub struct WebRequest {
    payload: RequestPayload,
    uri: Uri,
    nonce: Nonce,
    user: User,
}

pub fn new(payload: RequestPayload, uri: Uri, nonce: Nonce, user: User) -> WebRequest {
    WebRequest {
        payload: payload,
        uri: uri,
        nonce: nonce,
        user: user,
    }
}

pub fn api_sign(web_request: &WebRequest) -> Result<String, std::io::Error> { 
    api_sign_impl(web_request, encode)
}

fn api_sign_impl(
    web_request: &WebRequest,
    encoded_payload: fn(&RequestPayload, &Nonce) -> String,
) -> Result<String, std::io::Error> {
    let nonce_and_payload = format!(
        "{}{}",
        web_request.nonce.value.to_string(),
        encoded_payload(&web_request.payload, &web_request.nonce)
    );
    let nonce_and_payload_bytes = nonce_and_payload.as_bytes();
    let mut hasher = Sha256::new();
    hasher.update(nonce_and_payload_bytes);
    let sha256_encoded_nonce_and_payload = hasher.finalize().to_vec();
    let encoded_uri = web_request.uri.value().as_bytes().to_vec();
    let message = [&encoded_uri[..], &sha256_encoded_nonce_and_payload[..]].concat();
    let decoded_private_key = base64::decode(web_request.user.private_key())
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;
    let hmac_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA512, &decoded_private_key);
    let encoded_signature = base64::encode(ring::hmac::sign(&hmac_key, &message).as_ref());
    Ok(encoded_signature)
}

#[cfg(test)]
mod tests {
    use crate::payload::{self, OpenOrdersRequestPayload};

    use super::super::nonce::Nonce;
    use super::super::{nonce, uri};
    use super::*;
    use lootag_cryptoassignment_domain::user;
    use lootag_cryptoassignment_domain::user::User;

    #[test]
    fn should_api_sign_return_ok_result() {
        //Arrange
        let open_orders = OpenOrdersRequestPayload {};
        let payload = RequestPayload::OpenOrders(open_orders);
        let uri = uri::new(String::from("some_uri"));
        let nonce = Nonce {
            value: 1616492376594,
        };
        let user = user::new(
            String::from(""), 
            String::from("kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==")
        );
        let web_request = new(payload, uri, nonce, user);

        //Act
        let api_sign = api_sign_impl(&web_request, encoded_payload);

        //Assert
        assert!(api_sign.is_ok());
    }

    #[test]
    fn should_api_sign_return_correct_signature() {
        //Arrange
        let open_orders = OpenOrdersRequestPayload {};
        let payload = RequestPayload::OpenOrders(open_orders);
        let uri = uri::new(String::from("/0/private/AddOrder"));
        let nonce = Nonce {
            value: 1616492376594,
        };
        let user = user::new(
            String::from(""), 
            String::from("kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==")
        );
        let web_request = new(payload, uri, nonce, user);

        //Act
        let api_sign = api_sign_impl(&web_request, encoded_payload).unwrap();

        //Assert
        assert_eq!(
            api_sign,
            "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ=="
        );
    }

    fn encoded_payload(payload: &RequestPayload, nonce: &Nonce) -> String {
        let example_payload =
            "nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25";
        String::from(example_payload)
    }

    fn new_nonce() -> Nonce {
        Nonce {
            value: 1616492376594,
        }
    }
}
