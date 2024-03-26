use bytes::{Buf, BytesMut};

pub fn buf_to_vec(buf: &mut BytesMut) -> Vec<String> {
    let mut vec = Vec::new();
    let mut word = String::new();

    while buf.has_remaining() {
        let byte = buf.get_u8();

        if byte == b' ' {
            if !word.is_empty() {
                vec.push(word);
                word = String::new();
            }
        } else {
            word.push(byte as char);
        }
    }

    if !word.is_empty() {
        vec.push(word);
    }

    vec
}
