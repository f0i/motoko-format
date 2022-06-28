# Style rules

The [Motoko style guidelines](https://internetcomputer.org/docs/current/developer-docs/build/languages/motoko/style)
are the basis for this formatter.
However, the style guide allows for some variation.
The rules applied in the formatter and rules derivating from the style guidelines are listed in this document.

## Spacing

| Rule             | Status   | Example               |
| ---------------- | -------- | --------------------- |
| Arithmetic group | Planned  | `1 + 2*3 - 4/2`       |
| Operator spacing | Required | `x := 1`, `var x = 2` |
| List spacing     | Required | `(1, 2, 3); 4;`       |
| Brackets spacing | Required | `{ a = 1; b = 2 }`    |
| Brackets compact | Conflict | `{a = 1; b = 2}`      |

## Newlines

Line width is configurable in dprint.

| Rule                         | Status      | Example                                  |
| ---------------------------- | ----------- | ---------------------------------------- |
| Linebreak after operator     | Required    | `1 + 2 +\n  3;`, `let x =\n  4;`         |
| Indent width 2               | Required    | `let x =\n  4;`                          |
| Break indent width 4         | Idea        | `let x =\n    4;`                        |
| One-liner without blank line | Should have | `func a() { 1 };\nfunc b() { 2 };`       |
| Multi-liner with blank line  | Should have | `func a() {\n  1\n};\n\nfunc b() { 2 };` |

Break indent width could be increased for things like function parameter over multiple lines.

A maximum of two blank lines are kept between declarations.

## Comments

| Rule                    | Status       | Example                          |
| ----------------------- | ------------ | -------------------------------- |
| Prefer line comments    | Nice to have | `1; /* one */\n` => `1;  // one` |
| 2 spaces before comment | Required     | `1;// one` => `1;  // one`       |
| Preserve more spaces    | Should have  | `1;    // one` => `1;    // one` |

Comments shold have at least two spaces if they are after a statement.
Several examples are using alligned comments.
This conflicts with the reasoning from [#indentation](https://internetcomputer.org/docs/current/developer-docs/build/languages/motoko/style#indentation):

> Indentation should not depend on the lexical contents of previous lines.
>
> [...]
>
> It can produce realignment churn when changing a line, which (even when automated by editors) inflates and obfuscates diffs.

## Semicolon

| Rule                           | Status      | Example                    |
| ------------------------------ | ----------- | -------------------------- |
| No tailing semicolon inline    | Should have | `{ a = 1; b = 2 }`         |
| Tailing semicolon              | Should have | `{\n  a = 1;\n  b = 2;\n}` |
| Semicolon after last statement | Should have |                            |

Semicolon after last import and last declaration are optional.
The formatter should add them if not present.

## Numbers

| Rule            | Status       | Example                      |
| --------------- | ------------ | ---------------------------- |
| Add underscores | Nice to have | `1000000` => `1_000_000`     |
| Format hex      | Nice to have | `0x1000000` => `0x0100_0000` |
