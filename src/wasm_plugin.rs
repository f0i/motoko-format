use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;
use std::path::Path;

use super::configuration::resolve_config;
use super::configuration::Configuration;

struct MotokoPluginHandler;

impl SyncPluginHandler<Configuration> for MotokoPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> ResolveConfigurationResult<Configuration> {
        resolve_config(config, global_config)
    }

    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
      name: env!("CARGO_PKG_NAME").to_string(),
      version: version.clone(),
      config_key: "motoko".to_string(),
      file_extensions: vec!["mo".to_string()],
      file_names: vec![],
      help_url: "https://gitlab.com/f0i/motoko-formatter".to_string(),
      // TODO: setup CI
      config_schema_url: format!("https://gitlab.com/f0i/motoko-formatter/-/jobs/artifacts/{}/raw/public/dprint/schema.json?job=pages", version),
      update_url: Some("https://f0i.gitlab.io/motoko-formater/dprint/latest.json".to_string()),
    }
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../LICENSE"))
            .unwrap()
            .into()
    }

    fn format(
        &mut self,
        file_path: &Path,
        file_text: &str,
        config: &Configuration,
        _format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
    ) -> FormatResult {
        super::format_text(file_path, file_text, config)
    }
}

generate_plugin_code!(MotokoPluginHandler, MotokoPluginHandler);
