use std::path::Path;
use std::path::PathBuf;

use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::configuration::ResolveConfigurationResult;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::SyncPluginHandler;

use super::configuration::resolve_config;
use super::configuration::Configuration;

struct MotokoPluginHandler {}

impl MotokoPluginHandler {
  pub const fn new() -> Self {
    MotokoPluginHandler {}
  }
}

impl SyncPluginHandler<Configuration> for MotokoPluginHandler {
  fn resolve_config(&mut self, config: ConfigKeyMap, global_config: &GlobalConfiguration) -> ResolveConfigurationResult<Configuration> {
    resolve_config(config, global_config)
  }

  // Motoko extensions: markdown, mdown, mkdn, mdwn, mkd, md
  // ref: https://superuser.com/questions/249436/file-extension-for-markdown-files/285878#285878
  // ref: https://github.com/denoland/deno_registry2/issues/206
  fn plugin_info(&mut self) -> PluginInfo {
    let version = env!("CARGO_PKG_VERSION").to_string();
    PluginInfo {
      name: env!("CARGO_PKG_NAME").to_string(),
      version: version.clone(),
      config_key: "markdown".to_string(),
      file_extensions: vec![
        "md".to_string(),
        "mkd".to_string(),
        "mdwn".to_string(),
        "mkdn".to_string(),
        "mdown".to_string(),
        "markdown".to_string(),
      ],
      file_names: vec![],
      help_url: "https://dprint.dev/plugins/markdown".to_string(),
      config_schema_url: format!("https://plugins.dprint.dev/dprint/dprint-plugin-markdown/{}/schema.json", version),
      update_url: Some("https://plugins.dprint.dev/dprint/dprint-plugin-markdown/latest.json".to_string()),
    }
  }

  fn license_text(&mut self) -> String {
    std::str::from_utf8(include_bytes!("../LICENSE")).unwrap().into()
  }

  fn format(
    &mut self,
    _file_path: &Path,
    file_text: &str,
    config: &Configuration,
    mut format_with_host: impl FnMut(&Path, String, &ConfigKeyMap) -> FormatResult,
  ) -> FormatResult {
    return super::format_text(file_text, config, |tag, file_text, line_width| Ok(None));
  }
}

generate_plugin_code!(MotokoPluginHandler, MotokoPluginHandler::new());
