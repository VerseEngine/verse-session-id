//! # verse-session-id
//!
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/verse-session-id.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/verse-session-id)
//!
//! Session ID for [@VerseEngine/verse-core](https://github.com/VerseEngine/verse-core)
//!
//! ## Usage
//! ### Signature Verification
//! ```rust
//! use verse_session_id::*;
//!
//! ...
//! pub fn verify_string(session_id: &str, signature: &str, data: &str) -> bool {
//!   let Ok(sid) = session_id.parse::<SessionId>() else {
//!      return false;
//!   };
//!   let Ok(ss) = signature.parse::<SignatureSet>() else {
//!      return false;
//!   };
//!
//!   sid.verify(vec![data.as_bytes()], &ss).is_ok()
//!}
mod session_id;
pub use session_id::*;

mod session_id_pair;
pub use session_id_pair::*;

mod errors;
