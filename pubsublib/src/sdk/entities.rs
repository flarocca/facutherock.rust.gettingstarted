use std::future::Future;

pub trait Command { }

pub trait Event { }

pub trait MessageHandler<T: Command> {
    fn handle(message: T) -> dyn Future<Output = String>;
} 