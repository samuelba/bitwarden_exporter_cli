![GitHub](https://img.shields.io/github/license/samuelba/bitwarden_exporter_cli)
[![ci](https://github.com/samuelba/bitwarden_exporter_cli/actions/workflows/ci.yml/badge.svg)](https://github.com/samuelba/bitwarden_exporter_cli/actions/workflows/ci.yml)
![GitHub tag (latest SemVer)](https://img.shields.io/github/v/tag/samuelba/bitwarden_exporter_cli)

# Bitwarden Exporter CLI

This is a simple CLI tool to generate an encrypted Bitwarden JSON export file that can be imported into a Bitwarden vault.

The repo contains a submodule to the [Bitwarden SDK](https://github.com/bitwarden/sdk). Initialize it with `git submodule update --init --recursive`.

## Usage

```bash
$ bitwarden-exporter-cli --help

Usage: bitwarden-exporter-cli --password <PASSWORD> --folders <FOLDERS> --ciphers <CIPHERS>

Options:
  -p, --password <PASSWORD>  The password to use for encryption
  -f, --folders <FOLDERS>    The JSON string representing the folders. Example: '[{"id":"00000000-0000-0000-0000-000000000001","name":"My Folder"}]'
  -c, --ciphers <CIPHERS>    The JSON string representing the ciphers. Example: '[{"folderId":"00000000-0000-0000-0000-000000000001","name":"My Test","notes":"My Notes","username":"my_username","password":"my_password","loginUris":["https://example.com"]}]'
  -h, --help                 Print help
```
