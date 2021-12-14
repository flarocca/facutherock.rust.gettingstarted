// pub fn client_demo() {
//     let mut client = BusClientBuilder::new()
//         .with_connection_string("connection_string")
//         .for_topic("sarasa")
//         .create();

//     let command = SendEmailCommand::new("", "", "");

//     client.send(command).await;
// }

// pub fn server_demo() {
//     let mut server = BusClientBuilder::new()
//         .with_connection_string("connection_string")
//         .for_topic("sarasa")
//         .subscribe<SendEmailCommand>(handler)
//         .create();

//     server.start().await;
// }

// struct SendEmailCommand {
//     to: String,
//     subject: String,
//     body: String
// }

// impl SendEmailCommand {
//     fn new(to: String, subject: String, body: String) -> SendEmailCommand {
//         SendEmailCommand {
//             to,
//             subject,
//             body
//         }
//     }
// }

// impl Command for SendEmailCommand {}

// pub trait Command { }

// pub trait Event { }

// pub trait BusClient {
//     fn publish<T: Event>(&self, event: T) -> Future;

//     fn send<T: Command>(&self, command: T) -> Future;
// }

// pub struct BusClientBuilder;

// impl BusClientBuilder {
//     fn new() -> Self;

//     fn with_connection_string(&self, connection_string: String) -> Self;

//     fn for_topic(&self, topic_name: String) -> Self;

//     fn create(&self) -> BusServer;
// }

// pub trait BusServer {
//     fn start(&self) -> Future;

//     fn stop(&self) -> Future;
// }

// pub struct BusServerBuilder;

// impl BusServerBuilder {
//     fn new() -> Self;

//     fn with_connection_string(&self, connection_string: String) -> Self;

//     fn for_topic(&self, topic_name: String) -> Self;

//     // No se como expresar el OR
//     fn subscribe<T: Command + Event>(&self, message_handler: MessageHandler<T>) -> Self;

//     fn create(&self) -> BusServer;
// }

// pub trait MessageHandler<T: Command + Event> {
//     fn handle(message: T) -> Future;
// } 