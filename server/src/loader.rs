use std::{fs, path::Path};

use anyhow::Result;
use tracing::{info, warn};

use crate::types::TokenMetadata;

pub fn load_tokens_from_folder<P: AsRef<Path>>(folder: P) -> Result<Vec<TokenMetadata>> {
    let mut tokens = vec![];

    for entry in fs::read_dir(folder)? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                warn!("Skipping unreadable file: {e}");
                continue;
            }
        };
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let data = match fs::read_to_string(&path) {
            Ok(d) => d,
            Err(e) => {
                warn!("Failed to read file {:?}: {e}", path);
                continue;
            }
        };

        let metadata: TokenMetadata = match serde_json::from_str(&data) {
            Ok(t) => t,
            Err(e) => {
                warn!("Skipping file that failed to parse JSON {:?}: {e}", path);
                continue;
            }
        };

        tokens.push(metadata);
    }

    info!("Loaded {} tokens", tokens.len());
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use std::fs::write;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn load_tokens_from_valid_folder() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("token.json");

        let json = r#"
        {
            "symbol": "TEST",
            "address": "0x0000000000000000000000000000000000000000",
            "decimals": 18,
            "name": "Test Token"
        }
        "#;

        write(&path, json).unwrap();

        let tokens = load_tokens_from_folder(dir.path()).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].symbol, "TEST");
        assert_eq!(tokens[0].name, "Test Token");
        assert_eq!(
            tokens[0].address.to_string(),
            "0x0000000000000000000000000000000000000000"
        );
    }

    #[test]
    fn skips_invalid_json_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("invalid.json");

        // Malformed JSON
        write(&path, r#"{ "symbol": "BAD", "#).unwrap();

        let tokens = load_tokens_from_folder(dir.path()).unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn ignores_non_json_files() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("not_a_token.txt");

        write(&path, "This is not JSON").unwrap();

        let tokens = load_tokens_from_folder(dir.path()).unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn fails_on_missing_folder() {
        let result = load_tokens_from_folder("nonexistent_folder");
        assert!(result.is_err());
    }
}
