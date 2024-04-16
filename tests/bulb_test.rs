use tokio::try_join;

use yeetlight::*;

mod mock;

#[tokio::test]
async fn test_power() {
    let mock_listener = mock::BulbListener::serve().await.unwrap();

    let bulb = Bulb::new(mock_listener.addr.ip());
    let mock_connection = mock_listener.accept();
    let bulb_connection = bulb.connect();
    let (mut mock_connection, mut bulb_connection) =
        try_join!(mock_connection, bulb_connection).unwrap();

    let response = bulb_connection.set_power(true, Effect::Smooth(400));
    let expected = r#"{"id":1,"method":"set_power","params":["on","smooth",400]}"#;
    let message = mock_connection.receive();
    let (message, _response) = try_join!(message, response).unwrap();
    assert_eq!(message, expected);

    let response = bulb_connection.set_power(false, Effect::Sudden);
    let expected = r#"{"id":1,"method":"set_power","params":["off","sudden",0]}"#;
    let message = mock_connection.receive();
    let (message, _response) = try_join!(message, response).unwrap();
    assert_eq!(message, expected);
}
