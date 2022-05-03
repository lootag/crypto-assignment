mod dto;
mod nonce;
mod payload;
mod uri;
mod web_request;

use std::collections::HashMap;

use std::{io::ErrorKind, time::Duration};

use backoff::future::retry;

use backoff::ExponentialBackoff;

use dto::{ServerTimeDto, XbtUsdPairDto};
use lootag_cryptoassignment_domain::{
    credentials::Credentials, open_order::OpenOrder, server_time::ServerTime, xbt_usd::XbtUsd,
};
use reqwest::header::CONTENT_TYPE;
use web_request::encoded_payload;

use crate::dto::OpenOrdersDto;
use crate::{
    payload::{OpenOrdersRequestPayload, RequestPayload},
    web_request::{api_sign, WebRequest},
};

#[derive(Debug)]
pub struct KrakenService {
    configuration: Configuration,
}

impl KrakenService {
    pub fn new(configuration: Configuration) -> Self {
        KrakenService {
            configuration: configuration,
        }
    }
}

impl KrakenService {
    pub async fn retrieve_server_time(&self) -> Result<ServerTime, std::io::Error> {
        retrieve_server_time_impl(&self.configuration).await
    }

    pub async fn retrieve_xbtusd_pair(&self) -> Result<XbtUsd, std::io::Error> {
        retrieve_xbtusd_pair_impl(&self.configuration).await
    }

    pub async fn retrieve_open_orders(
        &self,
        credentials: &Credentials,
    ) -> Result<Vec<OpenOrder>, std::io::Error> {
        retrieve_open_orders_impl(&self.configuration, credentials).await
    }
}

async fn retrieve_server_time_impl(
    configuration: &Configuration,
) -> Result<ServerTime, std::io::Error> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", configuration.base_url, String::from("/public/Time"));
    let response_json = client
        .get(&url)
        .send()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::NotFound, e.to_string()))?
        .text()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    let dto: ServerTimeDto = serde_json::from_str(&response_json)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    dto.to_business()
}

async fn retrieve_xbtusd_pair_impl(
    configuration: &Configuration,
) -> Result<XbtUsd, std::io::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}{}",
        configuration.base_url,
        String::from("/public/AssetPairs?pair=XXBTZUSD")
    );
    let response_json = client
        .get(&url)
        .send()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::NotFound, e.to_string()))?
        .text()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    let dto: XbtUsdPairDto = serde_json::from_str(&response_json)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    dto.to_business()
}

async fn retrieve_open_orders_impl(
    configuration: &Configuration,
    credentials: &Credentials,
) -> Result<Vec<OpenOrder>, std::io::Error> {
    let payload = RequestPayload::OpenOrders(OpenOrdersRequestPayload {});
    let uri = uri::new(String::from("/0/private/OpenOrders"));
    let nonce = nonce::new()?;
    let web_request = web_request::new(payload, uri, nonce, credentials.clone());
    let url = format!(
        "{}{}",
        configuration.base_url,
        String::from("/private/OpenOrders")
    );
    let response_json = request_open_orders(&url, credentials, &web_request, configuration)
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    let dto: OpenOrdersDto = serde_json::from_str(&response_json)?;

    dto.result
        .open
        .unwrap_or_else(|| HashMap::new())
        .into_iter()
        .map(|(identifier, order)| order.to_business(&identifier))
        .collect::<Result<Vec<OpenOrder>, std::io::Error>>()
}

async fn request_open_orders(
    url: &String,
    credentials: &Credentials,
    web_request: &WebRequest,
    configuration: &Configuration,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut backoff = ExponentialBackoff::default();
    backoff.max_elapsed_time = Some(configuration.retry_max_interval);
    backoff.multiplier = configuration.retry_multiplier;
    backoff.initial_interval = configuration.retry_initial_interval;
    retry(backoff, || async {
        match send_open_orders_request(&client, url, web_request, credentials).await {
            Ok(json) => Ok(json),
            Err(e) => Err(backoff::Error::transient(e.to_string())),
        }
    })
    .await
}

async fn send_open_orders_request(
    client: &reqwest::Client,
    url: &String,
    web_request: &WebRequest,
    credentials: &Credentials,
) -> Result<String, std::io::Error> {
    let result_field_name = "result";
    let text = client
        .post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .header("API-Sign", api_sign(&web_request)?)
        .header("API-Key", credentials.api_key().clone())
        .body(encoded_payload(&web_request)?)
        .send()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::NotFound, e.to_string()))?
        .text()
        .await
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

    if text.contains(result_field_name) {
        Ok(text)
    } else {
        Err(std::io::Error::new(
            ErrorKind::InvalidData,
            "error retrieving open orders",
        ))
    }
}

#[derive(Debug)]
pub struct Configuration {
    base_url: String,
    retry_initial_interval: Duration,
    retry_multiplier: f64,
    retry_max_interval: Duration,
}

impl Configuration {
    pub fn new(
        base_url: String,
        retry_initial_interval: Duration,
        retry_multiplier: f64,
        retry_max_interval: Duration,
    ) -> Self {
        Self {
            base_url: base_url,
            retry_initial_interval: retry_initial_interval,
            retry_multiplier: retry_multiplier,
            retry_max_interval: retry_max_interval,
        }
    }
}
