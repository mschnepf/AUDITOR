use crate::helpers::spawn_app;
use auditor::domain::{Record, RecordTest};
use fake::{Fake, Faker};

#[tokio::test]
async fn get_started_since_returns_a_200_and_list_of_records() {
    // Arange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // First send a couple of records
    let test_cases = (1..10)
        .into_iter()
        .map(|i| {
            Faker
                .fake::<RecordTest>()
                // Giving a name which is sorted the same as the time is useful for asserting later
                .with_record_id(format!("r{}", i))
                .with_start_time(format!("2022-03-0{}T12:00:00-00:00", i))
        })
        .collect::<Vec<_>>();

    for case in test_cases.iter() {
        let response = client
            .post(&format!("{}/add", &app.address))
            .header("Content-Type", "application/json")
            .json(&case)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(200, response.status().as_u16());
    }

    // Try different start dates and receive records
    for i in 1..10 {
        let response = client
            .get(&format!(
                "{}/get/started/since/2022-03-0{}T00:00:00-00:00",
                &app.address, i
            ))
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(200, response.status().as_u16());

        let mut received_records = response.json::<Vec<Record>>().await.unwrap();

        // make sure they are both sorted
        received_records.sort_by(|a, b| a.record_id.cmp(&b.record_id));

        for (j, (record, received)) in test_cases
            .iter()
            .skip(i - 1)
            .zip(received_records.iter())
            .enumerate()
        {
            assert_eq!(
                record,
                received,
                "Check {}|{}: Record {} and {} did not match.",
                i,
                j,
                record.record_id.as_ref().unwrap(),
                received.record_id
            );
        }
    }
}

#[tokio::test]
async fn get_started_since_returns_a_200_and_no_records() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!(
            "{}/get/started/since/2022-03-01T13:00:00-00:00",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let received_records = response.json::<Vec<Record>>().await.unwrap();

    assert!(received_records.is_empty());
}

#[tokio::test]
async fn get_stopped_since_returns_a_200_and_list_of_records() {
    // Arange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // First send a couple of records
    let test_cases = (1..10)
        .into_iter()
        .map(|i| {
            Faker
                .fake::<RecordTest>()
                // Giving a name which is sorted the same as the time is useful for asserting later
                .with_record_id(format!("r{}", i))
                .with_stop_time(format!("2022-03-0{}T12:00:00-00:00", i))
        })
        .collect::<Vec<_>>();

    for case in test_cases.iter() {
        let response = client
            .post(&format!("{}/add", &app.address))
            .header("Content-Type", "application/json")
            .json(&case)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(200, response.status().as_u16());
    }

    // Try different start dates and receive records
    for i in 1..10 {
        let response = client
            .get(&format!(
                "{}/get/stopped/since/2022-03-0{}T00:00:00-00:00",
                &app.address, i
            ))
            .send()
            .await
            .expect("Failed to execute request.");
        assert_eq!(200, response.status().as_u16());

        let mut received_records = response.json::<Vec<Record>>().await.unwrap();

        // make sure they are both sorted
        received_records.sort_by(|a, b| a.record_id.cmp(&b.record_id));

        for (j, (record, received)) in test_cases
            .iter()
            .skip(i - 1)
            .zip(received_records.iter())
            .enumerate()
        {
            assert_eq!(
                record,
                received,
                "Check {}|{}: Record {} and {} did not match.",
                i,
                j,
                record.record_id.as_ref().unwrap(),
                received.record_id
            );
        }
    }
}

#[tokio::test]
async fn get_stopped_since_returns_a_200_and_no_records() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!(
            "{}/get/stopped/since/2022-03-01T13:00:00-00:00",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());

    let received_records = response.json::<Vec<Record>>().await.unwrap();

    assert!(received_records.is_empty());
}

#[tokio::test]
async fn get_wrong_since_returns_a_404() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!(
            "{}/get/wrong/since/2022-03-01T13:00:00-00:00",
            &app.address
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(404, response.status().as_u16());
}
