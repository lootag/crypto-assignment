use std::{convert::Infallible, time::Duration};

use async_trait::async_trait;
use cucumber::{given, then, when, World, WorldInit};
use lootag_cryptoassignment_domain::{
    credentials::{self, Credentials},
    open_order::OpenOrder,
    server_time::{ServerTime},
    xbt_usd::XbtUsd,
};
use lootag_cryptoassignment_services::{Configuration, KrakenService};

const BASE_URL: &str = "https://api.kraken.com/0";
const RETRY_POLICY_INITIAL_DURATION: u64 = 2;
const RETRY_POLICY_MULTIPLIER: f64 = 2.0;
const RETRY_POLICY_MAXIMUM_DURATION: u64 = 15;
const API_KEY: &str = "you-api-key";
const PRIVATE_KEY: &str = "your-private-key";
const OTP_SECRET: &str = "your-otp-secret";

#[derive(Debug, WorldInit)]
pub struct WorldImpl {
    kraken_service: KrakenService,
    credentials: Option<Credentials>,
    server_time: Option<Result<ServerTime, std::io::Error>>,
    xbt_usd: Option<Result<XbtUsd, std::io::Error>>,
    open_orders_result: Option<Result<Vec<OpenOrder>, std::io::Error>>,
}

#[async_trait(?Send)]
impl World for WorldImpl {
    type Error = Infallible;
    async fn new() -> Result<Self, Infallible> {
        let configuration = Configuration::new(
            String::from(BASE_URL),
            Duration::from_secs(RETRY_POLICY_INITIAL_DURATION),
            RETRY_POLICY_MULTIPLIER,
            Duration::from_secs(RETRY_POLICY_MAXIMUM_DURATION),
        );
        Ok(Self {
            kraken_service: KrakenService::new(configuration),
            credentials: None,
            server_time: None,
            xbt_usd: None,
            open_orders_result: None,
        })
    }
}

#[given("that I provide a valid set of credentials")]
async fn set_right_credentials(world: &mut WorldImpl) {
    let credentials = credentials::new(
        String::from(API_KEY),
        String::from(PRIVATE_KEY),
        String::from(OTP_SECRET),
    );
    world.credentials = Some(credentials);
}

#[given("that I provide an invalid set of credentials")]
async fn set_wrong_credentials(world: &mut WorldImpl) {
    let credentials = credentials::new(String::from(""), String::from(""), String::from(""));
    world.credentials = Some(credentials);
}

#[when("I request the server time")]
async fn request_server_time(world: &mut WorldImpl) {
    let server_time_result = world.kraken_service.retrieve_server_time().await;
    world.server_time = Some(server_time_result);
}

#[when("I request the XBTUSD trading pair")]
async fn request_xbt_usd_pair(world: &mut WorldImpl) {
    let xbt_usd_result = world.kraken_service.retrieve_xbtusd_pair().await;
    world.xbt_usd = Some(xbt_usd_result);
}

#[when("I request my open orders")]
async fn request_open_orders(world: &mut WorldImpl) {
    let credentials = world.credentials.as_ref().unwrap();
    let open_orders_result = world.kraken_service.retrieve_open_orders(credentials).await;
    world.open_orders_result = Some(open_orders_result);
}

#[then("the api successfully returns a valid server time")]
async fn assert_server_time_is_ok(world: &mut WorldImpl) {
    assert!(world.server_time.as_ref().unwrap().is_ok())
}

#[then("the api successfully returns a valid XBTUSD trading pair")]
async fn assert_xbt_usd_pair_is_ok(world: &mut WorldImpl) {
    assert!(world.xbt_usd.as_ref().unwrap().is_ok())
}

#[then("the api successfully returns a valid set of open orders")]
async fn assert_open_orders_are_ok(world: &mut WorldImpl) {
    assert!(world.open_orders_result.as_ref().unwrap().is_ok())
}

#[then("the api does not return my open orders")]
async fn assert_open_orders_are_error(world: &mut WorldImpl) {
    assert!(world.open_orders_result.as_ref().unwrap().is_err())
}

#[tokio::main]
async fn main() {
    WorldImpl::run("tests/features").await
}
