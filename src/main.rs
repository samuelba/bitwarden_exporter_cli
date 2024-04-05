mod types;
use types::{JsonCipher, JsonFolder};

use bitwarden_crypto::Kdf;
use bitwarden_exporters::{export, Cipher, Folder};

use clap::Parser;
use std::num;

#[derive(Debug, Parser)]
struct Args {
    /// The password to use for encryption.
    #[clap(short, long)]
    password: String,
    /// The JSON string representing the folders. Example:
    /// '[{"id":"00000000-0000-0000-0000-000000000001","name":"My Folder"}]'
    #[clap(short, long)]
    folders: String,
    /// The JSON string representing the ciphers. Example:
    /// '[{"folderId":"00000000-0000-0000-0000-000000000001","name":"My Test","notes":"My Notes","username":"my_username","password":"my_password","loginUris":["https://example.com"]}]'
    #[clap(short, long)]
    ciphers: String,
}

fn main() {
    // Parse the command line arguments.
    let args: Args = Args::parse();
    let json_folders_str = args.folders;
    let json_ciphers_str = args.ciphers;
    let password = args.password;

    // Parse the JSON strings into the respective types.
    let json_folders: Vec<JsonFolder> = match serde_json::from_str(json_folders_str.as_str()) {
        Ok(folders) => folders,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    let json_ciphers: Vec<JsonCipher> = match serde_json::from_str(json_ciphers_str.as_str()) {
        Ok(ciphers) => ciphers,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Convert the JSON types into the types used by the exporter.
    let folders: Vec<Folder> = json_folders
        .iter()
        .map(|folder| Folder::from(folder.clone()))
        .collect();
    let ciphers: Vec<Cipher> = json_ciphers
        .iter()
        .map(|cipher| Cipher::from(cipher.clone()))
        .collect();

    // Create the export format.
    let kdf = Kdf::PBKDF2 {
        iterations: num::NonZeroU32::new(600000).unwrap(),
    };
    let format = bitwarden_exporters::Format::EncryptedJson {
        password: password.to_string(),
        kdf,
    };

    // Export the data.
    let json_str = match export(folders, ciphers, format) {
        Ok(json_str) => json_str,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("{}", json_str);
}
