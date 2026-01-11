## Aquila CLI
[![Crates.io](https://img.shields.io/crates/v/aquila_cli.svg)](https://crates.io/crates/aquila_cli)
[![Downloads](https://img.shields.io/crates/d/aquila_cli.svg)](https://crates.io/crates/aquila_cli)

A command-line interface for managing the server.

Allows developers and CI/CD pipelines to upload assets, publish versions and manage tokens.

> [!NOTE]  
> This tool requires a running server.

### Installation

crates.io:
```bash
cargo install aquila_cli
```
From source:
```bash
cargo install --path crates/aquila_cli
```

### Configuration

Can be configured via flags or environment variables:

* **URL**: `--url` or `AQUILA_URL` (default: `http://localhost:3000`)
* **Token**: `--token` or `AQUILA_TOKEN`

### Common Commands

* **Publish a version**:
    ```bash
    aquila publish ./assets --version v1.0.0
    ```
* **Mint a long-lived token** (requires admin/write permissions):
    ```bash
    aquila mint-token  "build_server" --duration 31536000
    ```
* **Generate a JWT Secret** (for server setup):
    ```bash
    aquila generate-secret
    ```

License: MIT OR Apache-2.0
