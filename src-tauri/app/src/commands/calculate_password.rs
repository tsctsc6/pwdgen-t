use crate::commands::{CommandError, UniversalError};
use crate::keystream_provider::KeystreamProvider;
use chacha20::ChaCha20;
use chacha20::cipher::KeyIvInit;
use chacha20::cipher::generic_array::GenericArray;
use serde::Deserialize;
use sha2::{Digest, Sha256};

static UP_LETTER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
static LOW_LETTER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
static NUMBER: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static SP_CHAR: [char; 15] = [
    '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=',
];

#[derive(Deserialize)]
pub struct Request {
    pub user_name: String,
    pub platform: String,
    pub skip_count: u32,
    pub use_up_letter: bool,
    pub use_low_letter: bool,
    pub use_number: bool,
    pub use_sp_char: bool,
    pub pwd_len: u32,
    pub main_password: String,
}

pub fn validate(request: &Request) -> Result<(), CommandError> {
    if request.user_name.is_empty() {
        Err(UniversalError {
            code: 0,
            message: "user_name is empty".to_string(),
        })?;
    }

    if request.platform.is_empty() {
        Err(UniversalError {
            code: 0,
            message: "platform is empty".to_string(),
        })?;
    }

    if request.pwd_len > 255 {
        Err(UniversalError {
            code: 0,
            message: "pwd_len is too long".to_string(),
        })?;
    }

    if request.main_password.is_empty() {
        Err(UniversalError {
            code: 0,
            message: "main_password is empty".to_string(),
        })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn calculate_password(
    _: tauri::AppHandle,
    request: Request,
) -> Result<String, CommandError> {
    validate(&request)?;
    let hash1 = Sha256::digest(
        [
            request.user_name.as_bytes(),
            request.main_password.as_bytes(),
        ]
        .concat(),
    )
    .to_vec();
    let hash2 = Sha256::digest(
        [
            request.main_password.as_bytes(),
            request.platform.as_bytes(),
        ]
        .concat(),
    )
    .to_vec();
    let hash: Vec<_> = hash1.iter().zip(hash2.iter()).map(|(x, y)| x ^ y).collect();

    // key is hash, nonce is hash first 96 bits.
    let mut keystream_provoder = KeystreamProvider::new(Box::new(ChaCha20::new(
        GenericArray::from_slice(&hash),
        GenericArray::from_slice(&hash[0..12]),
    )));

    let mut string_builder: Vec<char> = vec![];

    for _ in 0..request.skip_count {
        let _ = keystream_provoder.get_next_key()?;
    }

    for _ in 0..request.pwd_len {
        let char_set = loop {
            let key = keystream_provoder.get_next_key()?;
            let char_set: &[char] = match key % 4 {
                0 => {
                    if request.use_up_letter {
                        &UP_LETTER
                    } else {
                        &[]
                    }
                }
                1 => {
                    if request.use_low_letter {
                        &LOW_LETTER
                    } else {
                        &[]
                    }
                }
                2 => {
                    if request.use_number {
                        &NUMBER
                    } else {
                        &[]
                    }
                }
                3 => {
                    if request.use_sp_char {
                        &SP_CHAR
                    } else {
                        &[]
                    }
                }
                _ => &[],
            };
            if char_set.len() != 0 {
                break char_set;
            }
        };
        let char = loop {
            let key = keystream_provoder.get_next_key()?;
            break match uniformly_pick(char_set, key as usize) {
                None => continue,
                Some(char) => char,
            };
        };
        string_builder.push(char);
    }
    Ok(string_builder.iter().collect())
}

fn uniformly_pick(char_set: &[char], key: usize) -> Option<char> {
    if key >= ((256 / char_set.len()) * char_set.len()) {
        return None;
    }
    Some(char_set[key % char_set.len()])
}
