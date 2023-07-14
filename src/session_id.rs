use crate::errors;
use anyhow::Result;
use std::cmp::Ordering;
use std::fmt;

/// Bytes of Session ID
pub const SESSION_ID_SIZE: usize = 32;
/// Session ID data
pub type RawSessionId = [u8; SESSION_ID_SIZE];

/// Session ID
/// The session ID is the public key for ED25519.
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SessionId(RawSessionId);

fn compare_session_ids(a: &[u8], b: &[u8]) -> Ordering {
    let n = a.len();
    if n != b.len() {
        // 想定外
        return n.cmp(&b.len());
        // return (n - b.len()) as i32;
    }
    for i in 0..n {
        let diff = (a[i] as i32).cmp(&(b[i] as i32));
        if diff.is_ne() {
            return diff;
        }
    }
    Ordering::Equal
}

impl SessionId {
    pub fn eq_slice(&self, other: &impl AsRef<[u8]>) -> bool {
        self.cmp_slice(other).is_eq()
    }
    pub fn cmp_slice(&self, other: impl AsRef<[u8]>) -> Ordering {
        compare_session_ids(self.as_ref(), other.as_ref())
    }
    pub fn to_debug_string(&self) -> String {
        let mut s = base64::encode(self);
        s.truncate(7);
        s
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}
impl Default for SessionId {
    fn default() -> Self {
        SessionId([0; SESSION_ID_SIZE])
    }
}
impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", base64::encode(self.0))
    }
}
impl std::str::FromStr for SessionId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        base64::decode(s)?.try_into()
    }
}

impl TryFrom<&[u8]> for SessionId {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let v: RawSessionId = value.try_into()?;
        Ok(SessionId(v))
    }
}

impl TryFrom<&Vec<u8>> for SessionId {
    type Error = anyhow::Error;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref() as &[u8])
    }
}
impl TryFrom<Vec<u8>> for SessionId {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref() as &[u8])
    }
}

impl From<RawSessionId> for SessionId {
    fn from(v: RawSessionId) -> Self {
        SessionId(v)
    }
}
impl From<SessionId> for Vec<u8> {
    fn from(v: SessionId) -> Self {
        v.to_vec()
    }
}

impl AsRef<[u8]> for SessionId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl fmt::Debug for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_debug_string())
    }
}

pub trait SessionIdCompatible {
    fn to_bytes(&self) -> Option<&[u8]>;
    fn to_session_id(&self) -> Result<SessionId> {
        self.to_bytes().ok_or_else(errors::required!())?.try_into()
    }
    fn eq_slice(&self, other: &impl SessionIdCompatible) -> bool {
        let a = self.to_bytes();
        let b = other.to_bytes();
        if a.is_none() && b.is_none() {
            return true;
        }
        if a.is_none() != b.is_none() {
            return false;
        }
        compare_session_ids(a.unwrap(), b.unwrap()).is_eq()
    }
    fn to_debug_string(&self) -> String {
        match self.to_bytes() {
            Some(v) => {
                let mut s = base64::encode(v);
                s.truncate(7);
                s
            }
            None => "<NOID>".to_string(),
        }
    }
}
impl SessionIdCompatible for Option<Vec<u8>> {
    fn to_bytes(&self) -> Option<&[u8]> {
        self.as_ref().map(|v| v as &[u8])
    }
}
impl<'a> SessionIdCompatible for Option<&'a SessionId> {
    fn to_bytes(&self) -> Option<&[u8]> {
        self.map(|v| v.as_ref())
    }
}
impl<'a> SessionIdCompatible for &'a [u8] {
    fn to_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}
/* impl<'a> SessionIdCompatible for &'a Vec<u8> {
    fn to_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
} */
impl SessionIdCompatible for Vec<u8> {
    fn to_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
}
impl SessionIdCompatible for SessionId {
    fn to_bytes(&self) -> Option<&[u8]> {
        Some(self.as_ref())
    }
}
/* impl<'a, T> SessionIdCompatible for T
where
    T: AsRef<[u8]>,
{
    fn to_bytes(&self) -> Option<&[u8]> {
        Some(self)
    }
} */
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_session_id() {
        let sid0 = SessionId::from([1; SESSION_ID_SIZE]);
        let sid1 = SessionId::from([2; SESSION_ID_SIZE]);
        assert_eq!(sid0, SessionId::from([1; SESSION_ID_SIZE]));
        assert_ne!(sid0, sid1);
        assert_eq!(sid0, sid0.clone());
        assert!(sid0 < sid1);
        assert!(sid0 != sid1);
        assert!(sid0.cmp(&sid1).is_ne());
        let sid00 = sid0;
        assert!(sid00 == sid0);

        let mut exists = std::collections::HashSet::<SessionId>::new();
        assert!(!exists.contains(&sid0));
        exists.insert(sid0.clone());
        assert!(exists.contains(&sid0));
        assert!(!exists.contains(&sid1));

        assert!(sid0.cmp_slice(&sid1).is_ne());
        assert!(!sid0.eq_slice(&sid1.to_vec()));

        assert!(sid0.cmp_slice(&sid0).is_eq());
        assert!(sid0.eq_slice(&sid0.to_vec()));

        assert!(sid0.cmp_slice(&[]).is_ne());

        assert_ne!(sid0.to_debug_string(), sid1.to_debug_string());
        assert_eq!(sid0.to_debug_string(), format!("{:?}", sid0));

        let str = format!("{}", sid0);
        assert_eq!(SessionId::from_str(&str).unwrap(), sid0);

        let str = format!("{:?}", sid0);
        assert!(SessionId::from_str(&str).is_err());

        let sid00 = SessionId::try_from(sid0.to_vec());
        assert!(sid00.is_ok());
        assert_eq!(sid00.unwrap(), sid0);

        let sid00 = SessionId::try_from(&sid0.to_vec());
        assert!(sid00.is_ok());
        assert_eq!(sid00.unwrap(), sid0);

        let v: Vec<u8> = sid0.into();
        assert_eq!(v, sid0.to_vec());
    }
    #[test]
    fn test_session_id_compatible() {
        let sid0raw = [3; SESSION_ID_SIZE];
        let sid0 = SessionId::from(sid0raw.clone());
        let v0 = sid0.to_vec();
        let v1 = SessionId::from([4; SESSION_ID_SIZE]).to_vec();
        let none = None as Option<Vec<u8>>;

        assert!(Some(vec![1u8]).to_session_id().is_err());
        assert!(Some(v0.clone()).to_session_id().is_ok());
        assert!(none.to_session_id().is_err());

        assert_eq!(none.to_debug_string(), "<NOID>");

        assert_eq!((&Some(v0.clone())).to_debug_string(), format!("{:?}", sid0));
        assert_eq!(
            (&Some(v0.clone())).to_debug_string(),
            (Some(v0.clone())).to_debug_string(),
        );
        assert!(!(&Some(v0.clone())).eq_slice(&Some(v1.clone())));
        assert!(!(&Some(v0.clone())).eq_slice(&none));
        assert!(none.eq_slice(&none));
        assert!(!(none).eq_slice(&Some(v0.clone())));

        assert!((&Some(v0.clone())).eq_slice(&Some(&sid0)));
        assert!((&Some(v0.clone())).eq_slice(&sid0));
        let ar: &[u8] = &sid0raw[..];
        assert!((&Some(v0.clone())).eq_slice(&ar));
        assert!((&Some(v0.clone())).eq_slice(&v0));
    }
}
