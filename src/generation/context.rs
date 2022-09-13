use super::helper::*;
use crate::configuration::Configuration;
use dprint_core::formatting::*;

#[derive(Debug)]
pub struct Context<'a> {
    config: &'a Configuration,
    expect_space: bool,
    force_space: bool,
    possible_newline: bool,
    mode_no_space: bool,
}

impl<'a> Context<'a> {
    pub fn new(_text: &'a str, config: &'a Configuration) -> Self {
        Self {
            config,
            expect_space: false,
            force_space: false,
            possible_newline: false,
            mode_no_space: false,
        }
    }

    pub fn expect_space(&mut self) {
        self.expect_space = true;
    }

    pub fn force_space(&mut self) {
        self.force_space = true;
        self.expect_space = true;
    }

    pub fn possible_newline(&mut self) {
        self.possible_newline = true;
    }

    pub fn expect_space_or_newline(&mut self) {
        self.expect_space();
        self.possible_newline();
    }

    pub fn force_space_or_newline(&mut self) {
        self.force_space();
        self.possible_newline();
    }

    pub fn gen_expected_space(&mut self) -> PrintItems {
        let mut items = PrintItems::new();
        if !self.mode_no_space {
            if self.expect_space && self.possible_newline {
                items.push_signal(Signal::SpaceOrNewLine);
            } else if self.expect_space || self.force_space {
                items.push_signal(Signal::SpaceIfNotTrailing);
            } else if self.possible_newline {
                items.push_signal(Signal::PossibleNewLine)
            }
        }
        self.force_space = false;
        self.reset_expect();

        if_not_start_of_line(items)
    }

    pub fn _start_no_space(&mut self) {
        self.mode_no_space = true;
    }

    pub fn _finish_no_space(&mut self) {
        self.mode_no_space = false;
    }

    pub fn reset_possible_newline(&mut self) {
        self.possible_newline = false;
    }

    pub fn reset_expect(&mut self) {
        self.expect_space = false;
        self.possible_newline = false;
    }
}
