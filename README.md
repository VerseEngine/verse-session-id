# verse-session-id

[<img alt="crates.io" src="https://img.shields.io/crates/v/verse-session-id.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/verse-session-id)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/verse-session-id?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/verse-session-id)
[<img alt="MIT" src="https://img.shields.io/github/license/VerseEngine/verse-session-id?style=for-the-badge" height="20">](https://github.com/VerseEngine/verse-session-id/blob/main/LICENSE)
[<img alt="MIT" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge" height="20">](https://github.com/VerseEngine/verse-three/pulls)


ID with signature/verification functions.  
Used as session ID in [@VerseEngine/verse-core](https://github.com/VerseEngine/verse-session-id).


## Usage
### Signature Verification
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

### Generate ID
```rust
let id_pair = new_session_id_pair()?;
let session_id = id_pair.get_id();
// to string
let s = format!("{}", session_id);
```


### Create a signature
```rust
pub fn sign_string(&self, data: &str) -> Result<String> {
    let id_pair = ...;
    Ok(id_pair
        .sign(vec![data.as_bytes()])?
        .to_string())
}
```