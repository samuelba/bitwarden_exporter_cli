use bitwarden_exporters::{Cipher, CipherType, Folder, Login, LoginUri};

use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonFolder {
    id: Uuid,
    name: String,
}

impl From<JsonFolder> for Folder {
    fn from(folder: JsonFolder) -> Self {
        Folder {
            id: folder.id,
            name: folder.name,
        }
    }
}

#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonCipher {
    folder_id: Option<Uuid>,

    name: String,
    notes: Option<String>,

    pub username: Option<String>,
    pub password: Option<String>,
    pub login_uris: Vec<String>,
}

impl From<JsonCipher> for Cipher {
    fn from(cipher: JsonCipher) -> Self {
        Cipher {
            id: Uuid::new_v4(),
            folder_id: cipher.folder_id,
            name: cipher.name,
            notes: cipher.notes,
            r#type: CipherType::Login(Box::new(Login {
                username: cipher.username,
                password: cipher.password,
                login_uris: cipher
                    .login_uris
                    .iter()
                    .map(|uri| LoginUri {
                        uri: Option::from(uri.to_string()),
                        r#match: None,
                    })
                    .collect(),
                totp: None,
            })),
            favorite: false,
            reprompt: 0,
            fields: vec![],
            revision_date: Utc::now(),
            creation_date: Utc::now(),
            deleted_date: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json_folder() {
        let json_folder = JsonFolder {
            id: Uuid::new_v4(),
            name: "My Folder".to_string(),
        };
        let folder = Folder::from(json_folder.clone());
        assert_eq!(folder.id, json_folder.id);
        assert_eq!(folder.name, json_folder.name);
    }

    #[test]
    fn test_from_json_cipher() {
        let json_cipher = JsonCipher {
            folder_id: None,
            name: "My Test".to_string(),
            notes: Some("My Notes".to_string()),
            username: Some("my_username".to_string()),
            password: Some("my_password".to_string()),
            login_uris: vec!["https://example.com".to_string()],
        };
        let cipher = Cipher::from(json_cipher.clone());
        assert_eq!(cipher.id.is_nil(), false);
        assert_eq!(cipher.folder_id, json_cipher.folder_id);
        assert_eq!(cipher.name, json_cipher.name);
        assert_eq!(cipher.notes, json_cipher.notes);
        match cipher.r#type {
            CipherType::Login(login) => {
                assert_eq!(login.username, json_cipher.username);
                assert_eq!(login.password, json_cipher.password);
                assert_eq!(login.login_uris.len(), json_cipher.login_uris.len());
                for (login_uri, json_login_uri) in
                    login.login_uris.iter().zip(json_cipher.login_uris.iter())
                {
                    assert_eq!(login_uri.uri, Some(json_login_uri.to_string()));
                    assert_eq!(login_uri.r#match, None);
                }
                assert_eq!(login.totp, None);
            }
            _ => panic!("Expected Login cipher type"),
        }
        assert_eq!(cipher.favorite, false);
        assert_eq!(cipher.reprompt, 0);
        assert_eq!(cipher.fields.len(), 0);
        assert_eq!(cipher.revision_date.timestamp(), Utc::now().timestamp());
        assert_eq!(cipher.creation_date.timestamp(), Utc::now().timestamp());
        assert_eq!(cipher.deleted_date, None);
    }
}
