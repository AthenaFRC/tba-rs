# codegen

`codegen` produces checked-in Rust source from the pinned TBA OpenAPI
document. It is a workspace development tool and is not published.

The generator currently owns every top-level string and integer enum in the
schema. When present, variant names come from the schema's
`x-enum-varnames` extension; other string-enum names are derived from their
wire values. Integer enums include an `Unknown(i64)` variant that preserves the
library's existing ability to accept values introduced by newer API versions.

Generator inputs are configured by the workspace-root `codegen.toml`. It
selects the OpenAPI document and pins both the OpenAPI dialect version and the
TBA API version. Its `overrides` table contains Rust compatibility adjustments;
the generator fails if an override no longer matches the schema. Relative file
paths are resolved from the config file's directory. Select another config
with the global `--config <FILE>` option.

Each supported model family owns its schema parsing and Rust token rendering.
Shared modules only dispatch schemas, enforce cross-model rules, and assemble
the generated file. Rendering uses `quote` and `proc-macro2`; the complete
token stream is validated as a `syn::File`, normalized with Prettyplease, and
then formatted with the workspace's nightly rustfmt configuration.

From the workspace root, regenerate the output with:

```sh
cargo regen-models
```

Verify that the checked-in output is current without modifying it with:

```sh
cargo check-models
```

The generated output is `lib/src/models/generated.rs`. Do not edit that file
directly.
