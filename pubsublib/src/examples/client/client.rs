pub fn client_demo() {
    let mut client = BusClientBuilder::new()
        .with_connection_string("connection_string")
        .for_topic("sarasa")
        .create();

    let command = SendEmailCommand::new("", "", "");

    client.send(command).await;
}