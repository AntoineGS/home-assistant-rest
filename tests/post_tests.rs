use std::collections::HashMap;

use chrono::{FixedOffset, NaiveDate, NaiveDateTime, NaiveTime};
use home_assistant_rest::{get::StateEnum, post, Client};
use mockito::{Mock, ServerGuard};

fn create_mock_server(server: &mut ServerGuard, endpoint: &str) -> Mock {
    server
        .mock("POST", endpoint)
        .match_header("content-type", "application/json")
        .match_header("Authorization", "Bearer test_token")
}

#[tokio::test]
async fn test_update_post_states1_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/states/sensor.sun")
        .match_body(r#"{"state":"above_horizon","attributes":{}}"#)
        .with_body(r#"{"entity_id":"sensor.sun","state":"above_horizon","attributes":{},"last_changed":"2023-04-25T23:49:34.728773+00:00","last_updated":"2023-04-25T23:49:34.728773+00:00","context":{"id":"01GYXD54C8D0YFJ6ASFDGJBJR9","parent_id":null,"user_id":"ae03ad0cefa6247baf4178ffce416910"}}"#)
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;

    let request = post::StateParams {
        entity_id: "sensor.sun".to_owned(),
        state: "above_horizon".to_owned(),
        attributes: HashMap::new(),
    };

    let response = client.post_states(request).await?;

    let timezone = FixedOffset::east_opt(0).unwrap();
    assert_eq!(response.entity_id, "sensor.sun");
    assert_eq!(
        response.state,
        Some(StateEnum::String("above_horizon".to_owned()))
    );
    assert_eq!(response.attributes.len(), 0);
    assert_eq!(
        response.last_changed,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 25).unwrap(),
            NaiveTime::from_hms_nano_opt(23, 49, 34, 728_773_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap()
    );
    assert_eq!(
        response.last_updated,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 25).unwrap(),
            NaiveTime::from_hms_nano_opt(23, 49, 34, 728_773_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap()
    );
    assert_eq!(response.context.id, "01GYXD54C8D0YFJ6ASFDGJBJR9");
    assert_eq!(response.context.parent_id, None);
    assert_eq!(response.context.user_id, "ae03ad0cefa6247baf4178ffce416910");

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_update_post_states2_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/states/climate.thermostat")
        .match_body(r#"{"state":"cool","attributes":{}}"#)
        .with_body(r#"{"entity_id":"climate.thermostat","state":"cool","attributes":{},"last_changed":"2023-04-26T01:17:56.033828+00:00","last_updated":"2023-04-26T01:17:56.033828+00:00","context":{"id":"01GYXJ6XE1008RBVG58E2NKJ3N","parent_id":null,"user_id":"ae03ad0cefa6247baf4178ffce416910"}}"#)
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;

    let request = post::StateParams {
        entity_id: "climate.thermostat".to_owned(),
        state: "cool".to_owned(),
        attributes: HashMap::new(),
    };

    let response = client.post_states(request).await?;

    let timezone = FixedOffset::east_opt(0).unwrap();
    assert_eq!(response.entity_id, "climate.thermostat");
    assert_eq!(response.state, Some(StateEnum::String("cool".to_owned())));
    assert_eq!(response.attributes.len(), 0);
    assert_eq!(
        response.last_changed,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            NaiveTime::from_hms_nano_opt(1, 17, 56, 33_828_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap()
    );
    assert_eq!(
        response.last_updated,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            NaiveTime::from_hms_nano_opt(1, 17, 56, 33_828_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap()
    );
    assert_eq!(response.context.id, "01GYXJ6XE1008RBVG58E2NKJ3N");
    assert_eq!(response.context.parent_id, None);
    assert_eq!(response.context.user_id, "ae03ad0cefa6247baf4178ffce416910");

    mock_server.assert_async().await;

    Ok(())
}

#[tokio::test]
async fn test_create_post_states_async() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = mockito::Server::new();

    let mock_server = create_mock_server(&mut server, "/api/states/sensor.test")
        .match_body(r#"{"state":"create_new","attributes":{}}"#)
        .with_body(r#"{"entity_id":"sensor.test","state":"create_new","attributes":{},"last_changed":"2023-04-26T01:23:35.616516+00:00","last_updated":"2023-04-26T01:23:35.616516+00:00","context":{"id":"01GYXJH920PEZGN2ZB0QRNY763","parent_id":null,"user_id":"ae03ad0cefa6247baf4178ffce416910"}}"#)
        .create_async()
        .await;

    let client = Client::new(server.url().as_str(), "test_token")?;

    let request = post::StateParams {
        entity_id: "sensor.test".to_owned(),
        state: "create_new".to_owned(),
        attributes: HashMap::new(),
    };

    let response = client.post_states(request).await?;

    let timezone = FixedOffset::east_opt(0).unwrap();
    assert_eq!(response.entity_id, "sensor.test");
    assert_eq!(
        response.state,
        Some(StateEnum::String("create_new".to_owned()))
    );
    assert_eq!(response.attributes.len(), 0);
    assert_eq!(
        response.last_changed,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            NaiveTime::from_hms_nano_opt(1, 23, 35, 616_516_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap()
    );
    assert_eq!(
        response.last_updated,
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2023, 4, 26).unwrap(),
            NaiveTime::from_hms_nano_opt(1, 23, 35, 616_516_000).unwrap()
        )
        .and_local_timezone(timezone)
        .unwrap()
    );
    assert_eq!(response.context.id, "01GYXJH920PEZGN2ZB0QRNY763");
    assert_eq!(response.context.parent_id, None);
    assert_eq!(response.context.user_id, "ae03ad0cefa6247baf4178ffce416910");

    mock_server.assert_async().await;

    Ok(())
}