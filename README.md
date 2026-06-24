# Keymint Tauri Starter

A Tauri v2 app with Rust-based license activation via Keymint.

## Quick Start

```bash
npm install
npm run tauri dev
```

Set your Keymint credentials as environment variables before building:

```bash
export KEYMINT_CLIENT_API_KEY=km_client_your_key_here
export KEYMINT_PRODUCT_ID=prod_your_product_id_here
npm run tauri dev
```

## Configuration

| Variable | Description |
|---|---|
| `KEYMINT_CLIENT_API_KEY` | Client API key from [Keymint Dashboard](https://app.keymint.dev) |
| `KEYMINT_PRODUCT_ID` | Your product ID |

These are read at compile time via `option_env!()`. Set them before `cargo build` / `npm run tauri dev`.

## What's Included

- **Rust license module** — activate, deactivate, status check via Keymint REST API
- **Machine binding** — uses `machine-uid` crate for hardware-based host IDs
- **Local persistence** — stores license state in `~/.local/share/tauri-license-starter/license.json`
- **Activation UI** — simple HTML/JS frontend that calls Rust via Tauri `invoke()`
- **Key masking** — stored keys are masked when returned to the frontend

## Project Structure

```
├── src/
│   └── index.html              # Frontend activation dialog
├── src-tauri/
│   ├── Cargo.toml              # Rust dependencies
│   ├── tauri.conf.json         # Tauri config
│   └── src/
│       ├── main.rs             # Tauri builder + command registration
│       └── license.rs          # License activation/deactivation logic
└── package.json
```

## Tauri Commands

| Command | Description |
|---|---|
| `activate_license` | Activate a license key against the Keymint API |
| `deactivate_license` | Deactivate the current license |
| `is_activated` | Check if a valid license is stored |
| `get_license_status` | Get masked key + activation state |
