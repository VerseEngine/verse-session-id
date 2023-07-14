use crate::errors;
use crate::SessionId;
use anyhow::Result;
use ed25519_dalek::Digest;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Session ID and private key pair (ED25519).
pub type SessionIdPair = ed25519_dalek::Keypair;

/// Signature Salt Size
pub const SIGNATURE_SALT_SIZE: usize = 8;
/// Signature Size
pub const SIGNATURE_SIZE: usize = ed25519_dalek::Signature::BYTE_SIZE;

/// Session ID as public key
pub trait SessionIdPublic {
    /// Verify signature
    fn verify(&self, payload: Vec<&[u8]>, sigset: &SignatureSet) -> Result<()>;
}

impl SessionIdPublic for SessionId {
    fn verify(&self, payload: Vec<&[u8]>, sigset: &SignatureSet) -> Result<()> {
        let pk =
            ed25519_dalek::PublicKey::from_bytes(self.as_ref()).map_err(errors::signature!())?;
        let mut hasher = ed25519_dalek::Sha512::new();
        hasher.update(sigset.salt);
        for p in payload {
            hasher.update(p);
        }
        let signature = ed25519_dalek::Signature::from_bytes(&sigset.signature)
            .map_err(errors::signature!())?;
        Ok(pk
            .verify_prehashed(hasher, None, &signature)
            .map_err(errors::signature!())?)
    }
}

pub trait ISessionIdPair {
    /// Get session ID
    fn get_id(&self) -> SessionId;
    /// Create a signature for input data
    fn sign(&self, payload: Vec<&[u8]>) -> Result<SignatureSet>;
}

/// Generate SessionIdPair
pub fn new_session_id_pair() -> Result<SessionIdPair> {
    let sk = &mut [0u8; ed25519_dalek::SECRET_KEY_LENGTH];
    getrandom::getrandom(sk)?;
    let sk = ed25519_dalek::SecretKey::from_bytes(sk).map_err(errors::signature!())?;

    Ok(ed25519_dalek::Keypair {
        public: ed25519_dalek::PublicKey::from(&sk),
        secret: sk,
    })
}

impl ISessionIdPair for SessionIdPair {
    fn get_id(&self) -> SessionId {
        self.public.to_bytes().into()
    }
    fn sign(&self, payload: Vec<&[u8]>) -> Result<SignatureSet> {
        let mut salt = [0u8; SIGNATURE_SALT_SIZE];
        getrandom::getrandom(&mut salt)?;
        let mut hasher = ed25519_dalek::Sha512::new();
        hasher.update(salt);
        for p in payload {
            hasher.update(p);
        }
        let signature = self
            .sign_prehashed(hasher, None)
            .map_err(errors::signature!())?;

        Ok(SignatureSet {
            signature: signature.to_bytes(),
            salt,
        })
    }
}

/// Signature
#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct SignatureSet {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub signature: [u8; SIGNATURE_SIZE],
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub salt: [u8; SIGNATURE_SALT_SIZE],
}

impl fmt::Display for SignatureSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = Vec::<u8>::with_capacity(SIGNATURE_SIZE + SIGNATURE_SALT_SIZE);
        buf.extend_from_slice(&self.signature);
        buf.extend_from_slice(&self.salt);
        write!(f, "{}", base64::encode(buf))
    }
}
impl std::str::FromStr for SignatureSet {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        base64::decode(s)?.try_into()
    }
}
impl TryFrom<Vec<u8>> for SignatureSet {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != SIGNATURE_SIZE + SIGNATURE_SALT_SIZE {
            return Err(errors::convert!(format!(
                "{:?} != {:?}",
                value.len(),
                SIGNATURE_SIZE + SIGNATURE_SALT_SIZE
            )));
        }
        let ss = SignatureSet {
            signature: value[0..SIGNATURE_SIZE]
                .try_into()
                .map_err(|_v| errors::convert!())?,
            salt: value[SIGNATURE_SIZE..]
                .try_into()
                .map_err(|_v| errors::convert!())?,
        };
        Ok(ss)
    }
}

fn as_base64<const N: usize, S: Serializer>(
    val: &[u8; N],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&base64::encode(val))
}

fn from_base64<'de, const N: usize, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<[u8; N], D::Error> {
    use serde::de;

    let res = <&str>::deserialize(deserializer).and_then(|s| {
        base64::decode(s)
            .map_err(|e| de::Error::custom(format!("invalid base64 string: {}, {}", s, e)))
    })?;
    res.try_into()
        .map_err(|_| de::Error::custom(format!("invalid array size: {}", N)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keypair() {
        let kp = new_session_id_pair();
        // console_log!("{0:?}", kp);
        assert!(kp.is_ok());
    }

    #[test]
    fn test_sign_verify() {
        let kp = new_session_id_pair().unwrap();
        let ss = kp
            .sign(vec!["1234".as_bytes(), "testdata".as_bytes()])
            .unwrap();
        // console_log!("sig: {0:?}, salt: {1:?}", ss.signature, ss.salt);

        let session_id = kp.get_id();
        let res = session_id.verify(vec!["1234".as_bytes(), "testdata".as_bytes()], &ss);
        assert!(res.is_ok());

        let res = session_id.verify(vec!["0234".as_bytes(), "testdata".as_bytes()], &ss);
        assert!(res.is_err());

        let kp = new_session_id_pair().unwrap();
        let session_id = kp.get_id();
        let res = session_id.verify(vec!["1234".as_bytes(), "testdata".as_bytes()], &ss);
        assert!(res.is_err());

        let ss1 = SignatureSet {
            signature: ss.signature.clone(),
            salt: Default::default(),
        };
        let res = session_id.verify(vec!["1234".as_bytes(), "testdata".as_bytes()], &ss1);
        assert!(res.is_err());

        let ss1 = SignatureSet {
            signature: [0; SIGNATURE_SIZE],
            salt: ss.salt.clone(),
        };
        let res = session_id.verify(vec!["1234".as_bytes(), "testdata".as_bytes()], &ss1);
        assert!(res.is_err());
    }
    #[test]
    fn test_ss_serialize() {
        let ss = SignatureSet {
            signature: [1; SIGNATURE_SIZE],
            salt: [2; SIGNATURE_SALT_SIZE],
        };
        let serialized = serde_json::to_string(&ss).unwrap();
        let deserialized: SignatureSet = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ss, deserialized);
    }
    #[test]
    fn test_ss_serialize_str() {
        let ss = SignatureSet {
            signature: [1; SIGNATURE_SIZE],
            salt: [2; SIGNATURE_SALT_SIZE],
        };
        let serialized = ss.to_string();
        let deserialized: SignatureSet = serialized.parse().unwrap();
        assert_eq!(ss, deserialized);
    }
}
