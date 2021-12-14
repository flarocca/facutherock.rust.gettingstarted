use std::future::Future;

use super::entities::{Event, Command};

pub trait BusServer {
    fn start(&self) -> dyn Future<Output = String>;

    fn stop(&self) -> dyn Future<Output = String>;
}

pub trait BusClient {
    fn publish<T: Event>(&self, event: T) -> dyn Future<Output = String>;

    fn send<T: Command>(&self, command: T) -> dyn Future<Output = String>;
}