# @f0i/dprint-motoko

npm distribution of [motoko-format](https://github.com/f0i/motoko-format).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-motoko WASM file](https://github.com/f0i/motoko-format/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@f0i/dprint-motoko";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText("test.mo", "//Motoko Code ..."));
```
