use anyhow::Result;
use dprint_core::configuration::resolve_new_line_kind;
use dprint_core::formatting::PrintOptions;
use dprint_core::plugins::FormatResult;
use std::path::Path;

use crate::motoko_parser as motoko;

use crate::configuration::Configuration;
use crate::generation::generate;
use crate::motoko_parser::Node;

pub fn format_text(_file_path: &Path, text: &str, config: &Configuration) -> FormatResult {
    let node = motoko::parse(text)?;

    let ir = generate(&node, text, config);

    let result = dprint_core::formatting::format(|| ir, config_to_print_options(text, config));
    println!("text: {} => {} <> {:?}", text, result, node);
    if result == text {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[cfg(feature = "tracing")]
pub fn trace_file(
    _file_path: &Path,
    text: &str,
    config: &Configuration,
) -> dprint_core::formatting::TracingResult {
    let node = motoko::parse(text).unwrap();
    let mut print_options = config_to_print_options(text, config);

    dprint_core::formatting::trace_printing(|| generate(&node, text, config), print_options)
}

fn config_to_print_options(text: &str, config: &Configuration) -> PrintOptions {
    PrintOptions {
        indent_width: 2,
        max_width: config.line_width,
        use_tabs: false,
        new_line_text: resolve_new_line_kind(text, config.new_line_kind),
    }
}
