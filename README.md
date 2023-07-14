# verse-session-id

[<img alt="crates.io" src="https://img.shields.io/crates/v/verse-session-id.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/verse-session-id)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/verse-session-id?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/verse-session-id)

Session ID for [@VerseEngine/verse-core](https://github.com/VerseEngine/verse-core)


## Usage
### Signature Validation
```rust
use verse_session_id::*;

...
pub fn verify_string(session_id: &str, signature: &str, data: &str) -> bool {
    let Ok(sid) = session_id.parse::<SessionId>() else {
        return false;
    };
    let Ok(ss) = signature.parse::<SignatureSet>() else {
        return false;
    };

    sid.verify(vec![data.as_bytes()], &ss).is_ok()
}
```
