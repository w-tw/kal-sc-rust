//! Minimal RFC 4648 standard base64 encode/decode.
//!
//! Matches the behavior of `base64::STANDARD`: `A-Z a-z 0-9 + /` alphabet,
//! `=` padding, strict decode (rejects invalid characters, incorrect length,
//! and non-canonical trailing bits).

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const PAD: u8 = b'=';

/// Encode `input` as standard base64 with `=` padding.
pub fn encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    let mut chunks = input.chunks_exact(3);
    for chunk in &mut chunks {
        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];
        out.push(ALPHABET[(b0 >> 2) as usize] as char);
        out.push(ALPHABET[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);
        out.push(ALPHABET[(((b1 & 0b1111) << 2) | (b2 >> 6)) as usize] as char);
        out.push(ALPHABET[(b2 & 0b11_1111) as usize] as char);
    }
    let rem = chunks.remainder();
    match rem.len() {
        0 => {}
        1 => {
            let b0 = rem[0];
            out.push(ALPHABET[(b0 >> 2) as usize] as char);
            out.push(ALPHABET[((b0 & 0b11) << 4) as usize] as char);
            out.push(PAD as char);
            out.push(PAD as char);
        }
        2 => {
            let b0 = rem[0];
            let b1 = rem[1];
            out.push(ALPHABET[(b0 >> 2) as usize] as char);
            out.push(ALPHABET[(((b0 & 0b11) << 4) | (b1 >> 4)) as usize] as char);
            out.push(ALPHABET[((b1 & 0b1111) << 2) as usize] as char);
            out.push(PAD as char);
        }
        _ => unreachable!(),
    }
    out
}

fn decode_char(c: u8) -> Result<u8, ()> {
    match c {
        b'A'..=b'Z' => Ok(c - b'A'),
        b'a'..=b'z' => Ok(c - b'a' + 26),
        b'0'..=b'9' => Ok(c - b'0' + 52),
        b'+' => Ok(62),
        b'/' => Ok(63),
        _ => Err(()),
    }
}

/// Decode a standard base64 string. Strict: the length must be a multiple of
/// 4, characters outside the alphabet (including whitespace) are rejected,
/// and non-canonical padding (trailing bits set) is rejected.
pub fn decode(input: &str) -> Result<Vec<u8>, ()> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    if bytes.len() % 4 != 0 {
        return Err(());
    }

    let full_chunks = bytes.len() / 4 - 1;
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);

    for i in 0..full_chunks {
        let c0 = decode_char(bytes[i * 4])?;
        let c1 = decode_char(bytes[i * 4 + 1])?;
        let c2 = decode_char(bytes[i * 4 + 2])?;
        let c3 = decode_char(bytes[i * 4 + 3])?;
        out.push((c0 << 2) | (c1 >> 4));
        out.push((c1 << 4) | (c2 >> 2));
        out.push((c2 << 6) | c3);
    }

    let last = &bytes[full_chunks * 4..];
    let c0 = decode_char(last[0])?;
    let c1 = decode_char(last[1])?;
    match (last[2], last[3]) {
        (PAD, PAD) => {
            if c1 & 0b1111 != 0 {
                return Err(());
            }
            out.push((c0 << 2) | (c1 >> 4));
        }
        (b2, PAD) => {
            let c2 = decode_char(b2)?;
            if c2 & 0b11 != 0 {
                return Err(());
            }
            out.push((c0 << 2) | (c1 >> 4));
            out.push((c1 << 4) | (c2 >> 2));
        }
        (b2, b3) => {
            let c2 = decode_char(b2)?;
            let c3 = decode_char(b3)?;
            out.push((c0 << 2) | (c1 >> 4));
            out.push((c1 << 4) | (c2 >> 2));
            out.push((c2 << 6) | c3);
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_rfc4648_vectors() {
        assert_eq!(encode(b""), "");
        assert_eq!(encode(b"f"), "Zg==");
        assert_eq!(encode(b"fo"), "Zm8=");
        assert_eq!(encode(b"foo"), "Zm9v");
        assert_eq!(encode(b"foob"), "Zm9vYg==");
        assert_eq!(encode(b"fooba"), "Zm9vYmE=");
        assert_eq!(encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn decode_rfc4648_vectors() {
        assert_eq!(decode("").unwrap(), b"");
        assert_eq!(decode("Zg==").unwrap(), b"f");
        assert_eq!(decode("Zm8=").unwrap(), b"fo");
        assert_eq!(decode("Zm9v").unwrap(), b"foo");
        assert_eq!(decode("Zm9vYg==").unwrap(), b"foob");
        assert_eq!(decode("Zm9vYmE=").unwrap(), b"fooba");
        assert_eq!(decode("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    fn decode_rejects_bad_length() {
        assert!(decode("Z").is_err());
        assert!(decode("Zg").is_err());
        assert!(decode("Zg=").is_err());
        assert!(decode("Zg===").is_err());
        assert!(decode("Zm9vYmFy=").is_err());
    }

    #[test]
    fn decode_rejects_invalid_chars() {
        assert!(decode("Z!==").is_err());
        assert!(decode("Zg-=").is_err());
        assert!(decode("Z_==").is_err());
        assert!(decode("Zm\n8=").is_err());
    }

    #[test]
    fn decode_rejects_non_canonical_padding() {
        assert!(decode("Zh==").is_err());
        assert!(decode("Zm9=").is_err());
    }

    #[test]
    fn roundtrip_all_lengths() {
        for n in 0..=300usize {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 31 + 7) as u8).collect();
            let encoded = encode(&bytes);
            let decoded = decode(&encoded).expect("roundtrip");
            assert_eq!(bytes, decoded, "failed at length {n}");
        }
    }
}
