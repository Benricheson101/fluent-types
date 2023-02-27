<h1 align="center">💬 Fluent Types</h1>

Generate TypeScript type declarations for [Fluent](https://projectfluent.org) language files

## Example Usage
`fluent-types ./lang/en.ftl -o src/output.d.ts`

```ts
import {MessageNames, Placeholders} from './output';

const get<Name extends MessageNames>(msg: Name, args: Placeholders<Name>): string {
  const pattern = bundle.getMessage(msg);
  return bundle.formatPattern(pattern.value, args);
};
```


## CLI Usage
```sh
Fluent Types
Generate TypeScript type declarations for Fluent language files

USAGE:
    fluent-types [OPTIONS] <files>...

ARGS:
    <files>...    input fluent files

OPTIONS:
    -h, --help         Print help information
    -o <output>        the output file [default: -]
```
