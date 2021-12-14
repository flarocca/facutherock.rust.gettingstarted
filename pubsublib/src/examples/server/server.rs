pub fn server_demo() {
    let mut server = BusServerBuilder::new()
        .with_connection_string("connection_string")
        .for_topic("sarasa")
        .subscribe<SendEmailCommand>(handler)
        .create();

    server.start().await;
}