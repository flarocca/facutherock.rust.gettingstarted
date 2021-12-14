use crate::sdk::{abstractions::BusServer, entities::{Command, MessageHandler}};

pub struct BusServerBuilder;

impl BusServerBuilder {
    fn new() -> Self {
        BusServerBuilder{}
    }

    fn with_connection_string(&self, connection_string: String) -> Self {
        self
    }

    fn for_topic(&self, topic_name: String) -> Self {
        self
    }

    // No se como expresar el OR
    fn subscribe<T: Command>(&self, message_handler: dyn MessageHandler<T>) -> Self {
        self
    }

    fn create(&self) -> dyn BusServer {
        todo!()
    }
}