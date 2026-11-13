# Keymint Tauri Starter

A Tauri v2 app with Rust-based license activation via Keymint.

## Quick Start

```bash
npm install
npm run tauri dev
```

The frontend is plain HTML in `src/`, served by Tauri straight from `frontendDist`. There is no separate dev server to start.

Set your Keymint credentials as environment variables before building:

```bash
export KEYMINT_CLIENT_API_KEY=km_live_your_client_api_key_here
export KEYMINT_PRODUCT_ID=prod_your_product_id_here
npm run tauri dev
```

## Configuration

| Variable | Description |
|---|---|
| `KEYMINT_CLIENT_API_KEY` | Client API key from the [Keymint dashboard](https://app.keymint.dev) |
| `KEYMINT_PRODUCT_ID` | Your product ID |

These are read at compile time via `option_env!()`. Set them before `cargo build` / `npm run tauri dev`.

> **The client key ships inside your app.** Anything compiled into the binary can be extracted by anyone who has a copy of it. Use a key with the `client` scope, bound to this product, and never an `admin` key. Treat the client key as public and rotate it from the dashboard if it is abused.

## What's Included

- **Rust license module** — activate, deactivate, status check via Keymint REST API
- **Machine binding** — host ID derived from the OS machine ID (`machine-uid` crate), hashed before it is sent (see [Host ID](#host-id))
- **Local persistence** — stores license state in `license.json` under the OS local data directory (see [License state file](#license-state-file))
- **Activation UI** — plain HTML/JS frontend that calls Rust via Tauri `invoke()` from the `window.__TAURI__` global (`app.withGlobalTauri`), so no remote scripts are loaded and it works offline
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

## License state file

The activation state is saved as `license.json` in a `tauri-license-starter` folder inside the OS local data directory (`dirs::data_local_dir()`):

| OS | Path |
|---|---|
| Linux | `~/.local/share/tauri-license-starter/license.json` (or `$XDG_DATA_HOME/tauri-license-starter/`) |
| macOS | `~/Library/Application Support/tauri-license-starter/license.json` |
| Windows | `%LOCALAPPDATA%\tauri-license-starter\license.json` |

Change the folder name in `license_file_path()` in `src-tauri/src/license.rs` to match your app.

## Host ID

Each activation is tied to a host ID. The starter computes it in `src-tauri/src/license.rs` as the lowercase hex SHA-256 of a fixed, app-specific prefix (`HOST_ID_PREFIX`) followed by the OS machine ID. The result is always 64 characters, the maximum the API accepts, and the raw machine ID never leaves the device.

- Change `HOST_ID_PREFIX` to something unique to your app before your first release, then keep it. Changing it later changes every install's host ID.
- If the machine ID can't be read, `activate_license` and `deactivate_license` return the error "Couldn't read a device ID for this computer…", which the activation screen displays. There is no shared fallback ID, so machines without a readable ID never share one activation slot.
- If you need to keep sending the raw machine ID (for continuity with installs that activated before hashing), `host_id_from_machine_id` has a one-line commented alternative. Hashed is the default and is recommended.

### Upgrade note

Earlier versions of this starter sent the raw OS machine ID as the host ID, and fell back to the shared value `unknown-host` when it couldn't be read. Host IDs are now hashed, so an install that activated with an earlier build reports a different host ID after upgrading:

- The first time it activates with the new build, it uses one more activation slot. Its old activation stays on the license until you remove it, for example with `POST /key/deactivate` and an admin API key.
- Deactivating from the upgraded install sends the new host ID, so it can't release the old activation. Remove that one yourself in the same way.

If your licenses are close to their activation limit, raise the limit or clear old activations before shipping the upgrade. To avoid the change entirely, use the raw-ID line described above.
