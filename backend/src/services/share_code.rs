use anyhow::Result;
use base64::prelude::*;

use crate::entities::share_code::ShareCode;

pub fn decode_share_code_string(code: String) -> Result<Vec<String>> {
    let deck_code = String::from_utf8(BASE64_STANDARD.decode(code)?)?;

    Ok(deck_code
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>())
}

pub fn encode_share_codes(codes: &[ShareCode]) -> String {
    let card_codes: Vec<String> = codes.iter().map(|code| code.processed.clone()).collect();

    encode(&card_codes)
}

pub fn encode_share_code_strings(codes: &[String]) -> String {
    encode(codes)
}

fn encode(codes: &[String]) -> String {
    let deck_code = codes.join(",");

    BASE64_STANDARD.encode(deck_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_share_codes() {
        let codes = vec![
            ShareCode::new(String::from("SilverSable")),
            ShareCode::new(String::from("Bast")),
            ShareCode::new(String::from("Abomination")),
        ];

        assert_eq!(
            encode_share_codes(&codes),
            String::from("U2x2clNibEIsQnN0NCxBYm1udG5C")
        );
    }

    #[test]
    fn test_encode_share_code_strings() {
        let codes = vec![
            String::from("SlvrSblB"),
            String::from("Bst4"),
            String::from("AbmntnB"),
        ];

        assert_eq!(
            encode_share_code_strings(&codes),
            String::from("U2x2clNibEIsQnN0NCxBYm1udG5C")
        );
    }

    #[test]
    fn test_decode_share_code_string() {
        assert_eq!(
            decode_share_code_string(String::from("U2x2clNibEIsQnN0NCxBYm1udG5C")).unwrap(),
            vec![
                String::from("SlvrSblB"),
                String::from("Bst4"),
                String::from("AbmntnB"),
            ]
        );
    }
}
