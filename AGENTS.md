# AGENTS

## Project Overview

This repository contains `tba-rs`, an async Rust client for The Blue Alliance API. The crate is organized around:

- `src/api_client.rs`: shared HTTP client, authentication, base URL handling, and GET request execution.
- `src/api_result.rs`: response wrapper for successful results, ETag caching, `304 Not Modified`, unauthorized responses, and API errors.
- `src/api/`: endpoint modules. Keep endpoint paths relative to the configured API base URL. Generated full-API accessors are grouped by OpenAPI tag.
- `src/models/`: serde response models exported through `src/models/mod.rs`. `src/models/generated.rs` is generated from the TBA OpenAPI spec.
- `src/main.rs`: local smoke-test binary, not the primary public API.

## Development Commands

- `cargo +nightly fmt`
- `cargo check`
- `cargo test`
- `cargo clippy --all-targets -- -D warnings`

Run all relevant commands before handing off changes. If a command cannot be run, report that explicitly.

## Coding Guidelines

- Follow the existing module-per-endpoint and model-per-resource structure.
- Prefer typed serde models over unstructured JSON values.
- For generated OpenAPI coverage, keep accessors grouped by tag and avoid hand-editing generated model shapes unless the spec cannot represent the wire format correctly.
- Keep endpoint constants relative to `https://www.thebluealliance.com/api/v3`; do not include `/api/v3` or `/v3` in endpoint modules.
- Return `TbaApiResult<T>` from endpoint helpers.
- Preserve optional ETag support when adding endpoints.
- Do not commit `.env` files or API keys.
- Avoid broad refactors unless they directly support the requested change.

## Release Metadata

Keep `Cargo.toml`, `README.md`, and `LICENSE` aligned before publishing. This crate is licensed as `LGPL-3.0-or-later`.
