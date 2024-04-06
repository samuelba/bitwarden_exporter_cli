use crate::types::{JsonCipher, JsonFolder};

use bitwarden_crypto::Kdf;
use bitwarden_exporters::{export, Cipher, Folder};

use std::num;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("Export error: {0}")]
    Export(#[from] bitwarden_exporters::ExportError),
}

pub fn generate_encrypted_json(
    json_folders_str: &str,
    json_ciphers_str: &str,
    password: &str,
) -> Result<String, UtilsError> {
    // Parse the JSON strings into the respective types.
    let json_folders: Vec<JsonFolder> = match serde_json::from_str(json_folders_str) {
        Ok(folders) => folders,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(UtilsError::Serde(e));
        }
    };
    let json_ciphers: Vec<JsonCipher> = match serde_json::from_str(json_ciphers_str) {
        Ok(ciphers) => ciphers,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(UtilsError::Serde(e));
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
            return Err(UtilsError::Export(e));
        }
    };

    return Ok(json_str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_encrypted_json() {
        let json_folders_str =
            r#"[{"id":"00000000-0000-0000-0000-000000000001","name":"My Folder"}]"#;
        let json_ciphers_str = r#"[{"folderId":"00000000-0000-0000-0000-000000000001","name":"My Test","notes":"My Notes","username":"my_username","password":"my_password","loginUris":["https://example.com"]}]"#;
        let password = "password";

        let json_str =
            generate_encrypted_json(json_folders_str, json_ciphers_str, password).unwrap();
        let json = serde_json::from_str::<serde_json::Value>(&json_str).unwrap();
        assert_eq!(json["encrypted"], true);
        assert_eq!(json["passwordProtected"], true);
        assert_ne!(json["salt"], "");
        assert_eq!(json["kdfType"], 0);
        assert_eq!(json["kdfIterations"], 600000);
        assert_eq!(json["kdfMemory"], serde_json::Value::Null);
        assert_eq!(json["kdfParallelism"], serde_json::Value::Null);
        assert_ne!(json["encKeyValidation_DO_NOT_EDIT"], "");
        assert_eq!(json["encKeyValidation_DO_NOT_EDIT"].as_str().unwrap().starts_with("2."), true);
        assert_ne!(json["data"], "");
        assert_eq!(json["data"].as_str().unwrap().starts_with("2."), true);
    }

    #[test]
    fn test_generate_encrypted_json_invalid_folders_json() {
        let json_folders_str =
            r#"[{"id":"00000000-0000-0000-0000-000000000001","name":"My Folder"}"#;
        let json_ciphers_str = r#"[{"folderId":"00000000-0000-0000-0000-000000000001","name":"My Test","notes":"My Notes","username":"my_username","password":"my_password","loginUris":["https://example.com"]}]"#;
        let password = "password";

        let result = generate_encrypted_json(json_folders_str, json_ciphers_str, password);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_encrypted_json_invalid_ciphers_json() {
        let json_folders_str =
            r#"[{"id":"00000000-0000-0000-0000-000000000001","name":"My Folder"}]"#;
        let json_ciphers_str = r#"[{"folderId":"00000000-0000-0000-0000-000000000001","name":"My Test","notes":"My Notes","username":"my_username","password":"my_password","loginUris":["https://example.com"]}"#;
        let password = "password";

        let result = generate_encrypted_json(json_folders_str, json_ciphers_str, password);
        assert!(result.is_err());
    }
}
