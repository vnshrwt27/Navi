use std::error::Error;

pub trait Command {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}
