use crate::sdk::abstractions::BusClient;

pub struct BusClientBuilder;

impl BusClientBuilder {
    fn new() -> Self {
        BusClientBuilder {

        }
    }

    fn with_connection_string(&self, connection_string: String) -> Self {
        self
    }

    fn for_topic(&self, topic_name: String) -> Self {
        self
    }

    fn create(&self) -> dyn BusClient  {
        todo!() 
    }
}