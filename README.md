# GOL Check for Zed

Anti-hallucination firewall for Zed. Validates commands before execution.

## Install

Install from Zed's extension registry, or build from source:

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
/check npm install some-package
/check curl https://api.example.com/v1/data
```

## Get a Client ID

Free at [golproductions.com/check](https://www.golproductions.com/check.html)
