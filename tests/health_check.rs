use std::net::TcpListener;

use sqlx::Connection;
use zero2prod::configuration;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16()); // Check if return code is 200
    assert_eq!(Some(0), response.content_length()); // Check if body is empty
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let address = spawn_app();
    let configuration = configuration::get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = sqlx::PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(
        200,
        response.status().as_u16(),
        "body: {}",
        response
            .text()
            .await
            .expect("couldn't decode response text")
    ); // Check if return code is 200

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_guin%40gmail.com", "missing name"),
        ("", "missing both email and name"),
    ];

    for (body, case) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 BAD REQUEST when the payload was {case}"
        ); // Check if return code is 400
    }
}

// Spawn the app and return the address it is listening on
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener).expect("failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}
