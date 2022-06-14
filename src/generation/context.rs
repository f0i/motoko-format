use std::collections::HashSet;

use crate::motoko_parser::Node;

use crate::configuration::Configuration;
use dprint_core::formatting::*;

#[derive(Debug)]
pub struct Context<'a> {
    pub config: &'a Configuration,
    // keep track of whitespace
    pub expect_space: bool,
}

impl<'a> Context<'a> {
    pub fn new(text: &'a str, config: &'a Configuration) -> Self {
        Self {
            config,
            expect_space: false,
        }
    }
}
