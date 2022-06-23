# Motoko Formatter

A code formatter for the Motoko programming language.

## :warning: Risks and Disclaimer

This formatter is in early development!

> None of this is coding advice.
> Programs are risky; so are formatters.
> **You could definitely lose what you put in.**
> But we are headed west! This is the frontier.
> It's not for everyone, but we're glad you're with us.
> [¹](http://podcast.banklesshq.com/)

## Installation

The development version can be used as a plugin for [dprint](https://dprint.dev/).

Go to <https://dprint.dev/install/> to get install instructions for your platform.

Then run `dprint init` inside your project directory.
This will create a `.dprint.json` file.

Edit this `dprint.json` file to include include the Motoko plugin:

```json
{
  "includes": ["**/*.{mo}"],
  "excludes": [],
  "plugins": [
    "https://gitlab.com/f0i/motoko-format/-/jobs/artifacts/main/raw/release/dprint_plugin_motoko.wasm?job=release&file=plugin.wasm"
  ]
}
```

## VS Code Extension

A Visual Studio Code extension is available for dprint:
[Dprint Code Formatter](https://marketplace.visualstudio.com/items?itemName=dprint.dprint).

Currently this still requires installation of the CLI as described above.

## Update

Dprint keeps a local copy of the Motoko plugin.
Run `dprint clear-cache` to delete the local copy.
This will force re-downloading of the plugin file.

## Development References

[Syntax description in motoko repo](https://github.com/dfinity/motoko/blob/master/doc/modules/language-guide/pages/language-manual.adoc)

[Motoko grammar.txt](https://raw.githubusercontent.com/dfinity/motoko/master/doc/modules/language-guide/examples/grammar.txt)

[Motoko style guide](https://internetcomputer.org/docs/current/developer-docs/build/languages/motoko/style/)

[pest.rs](https://pest.rs/)

[Calculator parser example](https://createlang.rs/01_calculator/ast.html)

[dprint (rust alternative for prettier)](https://dprint.dev/plugin-dev/)
