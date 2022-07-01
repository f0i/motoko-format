use super::helper::*;
use crate::configuration::Configuration;
use dprint_core::formatting::*;

#[derive(Debug)]
pub struct Context<'a> {
    config: &'a Configuration,
    expect_space: bool,
    expect_space_or_newline: bool,
    mode_no_space: bool,
}

impl<'a> Context<'a> {
    pub fn new(_text: &'a str, config: &'a Configuration) -> Self {
        Self {
            config,
            expect_space: false,
            expect_space_or_newline: false,
            mode_no_space: false,
        }
    }

    pub fn expect_space(&mut self) {
        self.expect_space = true;
    }

    pub fn expect_space_or_newline(&mut self) {
        self.expect_space_or_newline = true;
    }

    pub fn gen_expected_space(&mut self) -> PrintItems {
        let mut items = PrintItems::new();
        if !self.mode_no_space {
            if self.expect_space_or_newline {
                items.push_signal(Signal::SpaceOrNewLine);
            } else if self.expect_space {
                items.push_signal(Signal::SpaceIfNotTrailing);
            }
        }
        self.reset_expect();

        if_not_start_of_line(items)
    }

    pub fn start_no_space(&mut self) {
        self.mode_no_space = true;
    }

    pub fn finish_no_space(&mut self) {
        self.mode_no_space = false;
    }

    pub fn reset_expect(&mut self) {
        self.expect_space = false;
        self.expect_space_or_newline = false;
    }
}
