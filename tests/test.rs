extern crate dprint_development;
extern crate dprint_plugin_motoko;

use std::path::PathBuf;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_motoko::configuration::resolve_config;
use dprint_plugin_motoko::*;

#[test]
fn test_specs() {
    let global_config = resolve_global_config(ConfigKeyMap::new(), &Default::default()).config;

    run_specs(
        &PathBuf::from("./tests/specs/simple"),
        &ParseSpecOptions {
            default_file_name: "file.mo",
        },
        &RunSpecsOptions {
            fix_failures: false,
            format_twice: true,
        },
        {
            let global_config = global_config.clone();
            move |file_path, file_text, spec_config| {
                let config_result =
                    resolve_config(parse_config_key_map(spec_config), &global_config);
                ensure_no_diagnostics(&config_result.diagnostics);

                format_text(file_path, &file_text, &config_result.config)
            }
        },
        move |_file_path, _file_text, _spec_config| {
            #[cfg(feature = "tracing")]
            {
                let config_result =
                    resolve_config(parse_config_key_map(_spec_config), &global_config);
                ensure_no_diagnostics(&config_result.diagnostics);
                return serde_json::to_string(&trace_file(
                    _file_path,
                    _file_text,
                    &config_result.config,
                ))
                .unwrap();
            }

            #[cfg(not(feature = "tracing"))]
      panic!("\n====\nPlease run with `cargo test --features tracing` to get trace output\n====\n")
        },
    )
}
