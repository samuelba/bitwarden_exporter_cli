# Bitwarden Exporter CLI

This is a simple CLI tool to generate an encrypted Bitwarden JSON export file that can be imported into a Bitwarden vault.

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
