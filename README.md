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

The current version can be used as a plugin for [dprint](https://dprint.dev/).

Follow the steps below to install dprint and set it to use motoko-format.

### Windows

- Download the dprint installer: [installer.exe](https://github.com/dprint/dprint/releases/latest/download/dprint-x86_64-pc-windows-msvc-installer.exe).
- Run it and follow the instructions.
- Skip the next section and continue with [Configure your project](#configure-your-project) below.

### Linux, Mac or WSL

- Run the following command in a terminal:

```bash
curl -fsSL https://dprint.dev/install.sh | sh
```

- Continue with [Configure your project](#configure-your-project) below.

### Configure your project

Create a `dprint.json` file in the root of your project directory and insert the following content:

```json
{
  "includes": ["**/*.{mo}"],
  "excludes": [],
  "plugins": [
    "https://gitlab.com/f0i/motoko-format/-/jobs/artifacts/main/raw/release/dprint_plugin_motoko.wasm?job=release&file=plugin.wasm"
  ]
}
```

If everything is working so far, you should be able to format a file with this command in a terminal:

```bash
dprint fmt -- path/to/file.mo
```

### VS Code Extension

The Visual Studio Code plugin currently requires dprint to be installed on your system.

- Follow the steps above to install and configure dprint.
- In VS Code, search in _Extensions_ for "**dprint**" and install the "[Dprint Code Formatter](https://marketplace.visualstudio.com/items?itemName=dprint.dprint)".
- You should also install the [Motoko language support](https://marketplace.visualstudio.com/items?itemName=dfinity-foundation.vscode-motoko) extension.
- Run the VS Code command (_View_ » _Command Palette_) `> Preferences:` **`Open Settings (JSON)`**.
- Add the following section for motoko inside the settings.json:

```json
{
    /* other settings */
    "[motoko]": {
        "editor.defaultFormatter": "dprint.dprint",
        "editor.formatOnSave": true,
    },
    /* other settings */
}
```

Now `.mo` files should get automatically formatted whenever you save them.

## Update

**Run `dprint clear-cache` in a terminal**.

Dprint keeps a local copy of the Motoko plugin.
Running `dprint clear-cache` will delete the local copy.
On the next run, dprint will automatically download the new version of the plugin file.

## Trouble shooting

Please don't hesitate to create an Issue if you run into any problem.
I'm happy and thankful to know any problem with the formatter, plugin, documentation or anything else related to this formatter.

[New GitHub Issue](https://github.com/f0i/motoko-format/issues/new/choose)

or

[New GitLab Issue](https://gitlab.com/f0i/motoko-format/-/issues/new)

- When formatting in VS Code, nothing happens
  - Check if dprint can be executed in a terminal
    ```bash
    dprint fmt -- path/to/file.mo
    ```
  - If that is working, check if dprint is enabled in VS code
  - Please create an issue to document all problems
- Formatter is generating unexpected output
  - Please crate an issue containing the input code and the expected output

## Advanced installation

There are several other options available to install dprint <https://dprint.dev/install/> to get install instructions for your platform.

## Development References

[Syntax description in motoko repo](https://github.com/dfinity/motoko/blob/master/doc/modules/language-guide/pages/language-manual.adoc)

[Motoko grammar.txt](https://raw.githubusercontent.com/dfinity/motoko/master/doc/modules/language-guide/examples/grammar.txt)

[Motoko style guide](https://internetcomputer.org/docs/current/developer-docs/build/languages/motoko/style/)

[pest.rs](https://pest.rs/)

[Calculator parser example](https://createlang.rs/01_calculator/ast.html)

[dprint (rust alternative for prettier)](https://dprint.dev/plugin-dev/)
