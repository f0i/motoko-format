# Generator

Convert a node tree into a set of PrintItems for dprint.

## Development info

Print items should not contain tailing spaces.
If spaces are required, set `context.expect_space = true`.
This will allow e.g. comments to add the correct amount of spaces if needed.

Unhandled `Node`s should be printed with `gen_id(node, context)`.
For debugging, this can be changed to `gen_debug(node, context)`.
