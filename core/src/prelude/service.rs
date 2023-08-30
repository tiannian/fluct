use std::error::Error;

pub trait Service: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    fn run(&mut self) -> Result<(), Self::Error>;

    fn start(&mut self) -> Result<(), Self::Error>;

    fn stop(&mut self) -> Result<(), Self::Error>;

    fn kill(&mut self) -> Result<(), Self::Error>;
}
