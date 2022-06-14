use crate::configuration::Configuration;

#[derive(Debug)]
pub struct Context<'a> {
    pub config: &'a Configuration,
    // keep track of whitespace
    pub expect_space: bool,
}

impl<'a> Context<'a> {
    pub fn new(_text: &'a str, config: &'a Configuration) -> Self {
        Self {
            config,
            expect_space: false,
        }
    }
}
