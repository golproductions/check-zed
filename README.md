# GOL Check for Zed

> **This is a thin wrapper.** The one location for Check (install, hook mode, MCP, CLI, and the HTTP contract) is [check.golproductions.com](https://check.golproductions.com) · [golproductions/check](https://github.com/golproductions/check). Integrate from there.

The anti-hallucination layer for Zed.

## Install

Not in Zed's extension registry. Install as a dev extension from this repo:

```bash
cd check-zed
cargo build --release --target wasm32-wasi
```

## Setup

Set your Client ID as an environment variable:

```bash
export GOL_CLIENT_ID="your_client_id"
```

## Usage

In Zed's chat or editor, use the slash command:

```
/check <command>
```

## Get a Client ID

Free at [golproductions.com/check](https://www.golproductions.com/check.html)
