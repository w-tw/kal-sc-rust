use crate::ffi;

fn ffi_hash(data: &[u8], hash_type: &str) -> String {
    let data_pointer = data.as_ptr();
    let data_length = data.len() as i32;
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len = unsafe {
        ffi::hash(
            data_pointer,
            data_length,
            hash_type.as_ptr(),
            hash_type.len() as i32,
            target_pointer,
        )
    } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap()
}

/// SHA-256 hash of the input data.
pub fn sha256(data: &[u8]) -> String {
    ffi_hash(data, "sha256")
}

/// SHA-512 hash of the input data.
pub fn sha512(data: &[u8]) -> String {
    ffi_hash(data, "sha512")
}

/// Blake3 hash of the input data at the specified bit length.
pub fn blake3(data: &[u8], length: Blake3Length) -> String {
    let hash_type = match length {
        Blake3Length::Bits256 => "blake3_256",
        Blake3Length::Bits512 => "blake3_512",
        Blake3Length::Bits1024 => "blake3_1024",
        Blake3Length::Bits2048 => "blake3_2048",
        Blake3Length::Bits4096 => "blake3_4096",
        Blake3Length::Bits9001 => "blake3_9001",
    };
    ffi_hash(data, hash_type)
}

/// SHAKE hash of the input data at the specified bit length.
pub fn shake(data: &[u8], length: ShakeLength) -> String {
    let hash_type = match length {
        ShakeLength::Bits1024 => "shake_1024",
        ShakeLength::Bits2048 => "shake_2048",
        ShakeLength::Bits4096 => "shake_4096",
    };
    ffi_hash(data, hash_type)
}

/// Verify a signature against a message and public key.
pub fn verify_signature(message: &str, signature: &str, public_key: &str) -> bool {
    let message_pointer = message.as_bytes().as_ptr();
    let message_length = message.len() as i32;
    let signature_pointer = signature.as_bytes().as_ptr();
    let signature_length = signature.len() as i32;
    let public_key_pointer = public_key.as_bytes().as_ptr();
    let public_key_length = public_key.len() as i32;
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len = unsafe {
        ffi::verify_signature(
            message_pointer,
            message_length,
            signature_pointer,
            signature_length,
            public_key_pointer,
            public_key_length,
            target_pointer,
        )
    } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap() == "true"
}

#[derive(Debug, Clone, Copy)]
pub enum Blake3Length {
    Bits256,
    Bits512,
    Bits1024,
    Bits2048,
    Bits4096,
    Bits9001,
}

#[derive(Debug, Clone, Copy)]
pub enum ShakeLength {
    Bits1024,
    Bits2048,
    Bits4096,
}
