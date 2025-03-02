use jwt_compact::{
    alg::{Rsa, RsaPrivateKey, RsaPublicKey},
    jwk::{JsonWebKey, KeyType},
    AlgorithmExt, Claims, Header, UntrustedToken,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Apple oauth2 certs
// From: https://appleid.apple.com/auth/keys
static APPLE_PUB_CERTS: &str = r#"
{
  "keys": [
    {
      "kty": "RSA",
      "kid": "W6WcOKB",
      "use": "sig",
      "alg": "RS256",
      "n": "2Zc5d0-zkZ5AKmtYTvxHc3vRc41YfbklflxG9SWsg5qXUxvfgpktGAcxXLFAd9Uglzow9ezvmTGce5d3DhAYKwHAEPT9hbaMDj7DfmEwuNO8UahfnBkBXsCoUaL3QITF5_DAPsZroTqs7tkQQZ7qPkQXCSu2aosgOJmaoKQgwcOdjD0D49ne2B_dkxBcNCcJT9pTSWJ8NfGycjWAQsvC8CGstH8oKwhC5raDcc2IGXMOQC7Qr75d6J5Q24CePHj_JD7zjbwYy9KNH8wyr829eO_G4OEUW50FAN6HKtvjhJIguMl_1BLZ93z2KJyxExiNTZBUBQbbgCNBfzTv7JrxMw",
      "e": "AQAB"
    },
    {
      "kty": "RSA",
      "kid": "YuyXoY",
      "use": "sig",
      "alg": "RS256",
      "n": "1JiU4l3YCeT4o0gVmxGTEK1IXR-Ghdg5Bzka12tzmtdCxU00ChH66aV-4HRBjF1t95IsaeHeDFRgmF0lJbTDTqa6_VZo2hc0zTiUAsGLacN6slePvDcR1IMucQGtPP5tGhIbU-HKabsKOFdD4VQ5PCXifjpN9R-1qOR571BxCAl4u1kUUIePAAJcBcqGRFSI_I1j_jbN3gflK_8ZNmgnPrXA0kZXzj1I7ZHgekGbZoxmDrzYm2zmja1MsE5A_JX7itBYnlR41LOtvLRCNtw7K3EFlbfB6hkPL-Swk5XNGbWZdTROmaTNzJhV-lWT0gGm6V1qWAK2qOZoIDa_3Ud0Gw",
      "e": "AQAB"
    }
  ]
}
"#;

//  Google oauth2 cert
//  From: https://www.googleapis.com/oauth2/v3/certs
static GOOGLE_PUB_CERTS: &str = r#"
{
  "keys": [
    {
      "kty": "RSA",
      "kid": "f5f4bf46e52b31d9b6249f7309ad0338400680cd",
      "use": "sig",
      "alg": "RS256",
      "n": "q5hcowR4IuPiSvHbwj9Rv9j2XRnrgbAAFYBqoLBwUV5GVIiNPKnQBYa8ZEIK2naj9gqpo3DU9lx7d7RzeVlzCS5eUA2LV94--KbT0YgIJnApj5-hyDIaevI1Sf2YQr_cntgVLvxqfW1n9ZvbQSitz5Tgh0cplZvuiWMFPu4_mh6B3ShEKIl-qi-h0cZJlRcIf0ZwkfcDOTE8bqEzWUvlCpCH9FK6Mo9YLjw5LroBcHdUbOg3Keu0uW5SCEi-2XBQgCF6xF3kliciwwnv2HhCPyTiX0paM_sT2uKspYock-IQglQ2TExoJqbYZe6CInSHiAA68fkSkJQDnuRZE7XTJQ",
      "e": "AQAB"
    },
    {
      "kid": "f833e8a7fe3fe4b878948219a1684afa373ca86f",
      "e": "AQAB",
      "alg": "RS256",
      "n": "uB-3s136B_Vcme1zGQEg-Avs31_voau8BPKtvbYhB0QOHTtrXCF_wxIH5vWjl-5ts8up8Iy2kVnaItsecGohBAy_0kRgq8oi-n_cZ0i5bspAX5VW0peh_QU3KTlKSBaz3ZD9xMCDWuJFFniHuxLtJ4QtL4v2oDD3pBPNRPyIcZ_LKhH3-Jm-EAvubI5-6lB01zkP5x8f2mp2upqAmyex0jKFka2e0DOBavmGsGvKHKtTnE9oSOTDlhINgQPohoSmir89NRbEqqzeZVb55LWRl_hkiDDOZmcM_oJ8iUbm6vQu3YwCy-ef9wGYEij5GOWLmpYsws5vLVtTE2U-0C_ItQ",
      "kty": "RSA",
      "use": "sig"
    }
  ]
}

"#;

// Generated with https://www.scottbrady91.com/tools/jwt
static TEST_PUB_CERTS: &str = r#"
{
    "keys": [
        {
            "alg": "RS256",
            "e": "AQAB",
            "key_ops": [
              "verify"
            ],
            "kty": "RSA",
            "n": "toWX9ru6c-7agMKRf-KuZiJhUEv5myv2FDjcuqcK7t-sAX4t4oMeOgIBmkdgeXt3Q3p5GKinovS80LXAqLaSw_Gtr1YA6beEK21zCnh9i17ZBHxU868JtS4v13Kdb24gJ5DbcknMYPTQUl7TRFdnX8ONVFZc__sItgj1nJNoaulF6EOQ8fjO5imvKQCqsQPZ-t42zVJUJo_Qk5yi5HRHXtYSaI5pfbsZWn8Us_8cTE-7vQ4DU311kt7bmxXInMpc8ygEenD1A4umjv37rhy4OV2LsFzeEX6ye0q9yr3QIkxNJEZdah3LjbFv-uqIQC5U6yrtBeNyBxxt5kQQMLuOgQ",
            "use": "sig",
            "kid": "8e4ea9f5b2d8234e79f2c73a4b1683a4"
          }
    ]
}
"#;

// Generated with https://www.scottbrady91.com/tools/jwt
pub static TEST_PRIV_CERTS: &str = r#"
{
    "keys": [
        {
            "alg": "RS256",
            "d": "EiNBqa3LTPAEdXbGuYJ2OdIOATCIZzVXW96wS9NQg9JlKAGfX64Pixld8JqwaWh6i15LufnKLUH_oKHr7LG2V5Ok4qw72DETbxlbE3d-_YjF-h5WdtM2T_N9PGS4ZM74-Dv6teddg8w9BqJ257vjziFprvxyFQ7TAi8iJGNScguXYJEcopvvg6cPYW3WOhj-zY7ULrkQ4jDmXPDZTzOrXcMSA27_lP93IxU51bYOp1A3ygR8pvXD7GlPJOmnoDKfotpYsNfIETxiBpeo_zKlWqsOGY2yYVOY1fnNSUJxUfTwUS8xwEW5rIxCVU7wgeaAqVhel94LLh5G920EX17BDQ",
            "dp": "a76UynqtL_c3vw7ZRSzUvwNcMH6nqDemluwkifU1nXwjTC4Q2N3EKJOMXslHDk5zFwkjXrI5FT2Wo0pB_dYS0z7Gl9FaYAiv4t7ykHWCKdTvPne8pRCpbZO1xvytUAH0KvmWND9LQ9RAue5zGzBmINGbr1oG97bLjY4h5EBZtqE",
            "dq": "eNPsyk-zca8PozKWJ22ckNuuKk8OhYOMhhyUaFyrDeWlWBRwoX-J1MccWoDlv_9ZaRkFrItZS22kAksb4RBUyqWPeRi8Pijex_9LNpkAVlbPoVN3IbNeR8ZAAM_pXeaMA1PLkF2wOLbSHbhXc8GzQUXhKsxAJK527OhOksfCz9c",
            "e": "AQAB",
            "key_ops": [
              "sign"
            ],
            "kty": "RSA",
            "n": "toWX9ru6c-7agMKRf-KuZiJhUEv5myv2FDjcuqcK7t-sAX4t4oMeOgIBmkdgeXt3Q3p5GKinovS80LXAqLaSw_Gtr1YA6beEK21zCnh9i17ZBHxU868JtS4v13Kdb24gJ5DbcknMYPTQUl7TRFdnX8ONVFZc__sItgj1nJNoaulF6EOQ8fjO5imvKQCqsQPZ-t42zVJUJo_Qk5yi5HRHXtYSaI5pfbsZWn8Us_8cTE-7vQ4DU311kt7bmxXInMpc8ygEenD1A4umjv37rhy4OV2LsFzeEX6ye0q9yr3QIkxNJEZdah3LjbFv-uqIQC5U6yrtBeNyBxxt5kQQMLuOgQ",
            "p": "4nAtQjIbaHV9W94Mgt_E_XCG7W_6A6wO1K1YDd8ovY01AIld-MOvqo4HgY6P484ZQ50t578LwFmthRB2R-oR8KUv3YU00FhlwmM2ovCFtVIJWo6a908PWnnrd0kQyRcFOpA7OIZ_bz-96pVR6OlJkM-A3JUiKkaxW3-qxhfjjfM",
            "q": "zlmuuNSM0idNEmgkuw21eYAwti4aKuG4dTQjLQQ4BuOfhaOO0blt-UTAlugWDAdLTHWWnzDB4Mkxo3ebDjDN3EySFg5FhJSVP-bwPFWD5sckjScMaMBkTLGHBYsEs6m-RsBfBd9GpFqcdykfCnYOk3NCCnTNZK0fmKydlwAjKrs",
            "qi": "jwXXfD21DCkZZmH-qYXZEdakDGLrsM2-kiekE8feA8_miyp0P_SsAxoOGZ-GZThLdgmOcrmoiH_QqYj2KAjXiEdk8SkqA5yb89h7B3D71ztqP1b4WaLpOkxHTRciQ0VEiTFI5g3RQJEXgQByOOjgOZkgNaFXjdp7xJgHzDhHLRA",
            "use": "sig",
            "kid": "8e4ea9f5b2d8234e79f2c73a4b1683a4"
        }
    ]
}
"#;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum DecodedClaims {
    Apple(AppleClaims),
    Google(GoogleClaims),
    Test(CustomClaims),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomClaims {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub at_hash: String,
    pub iat: Option<u64>,
    pub exp: Option<u64>,
    pub nonce: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppleClaims {
    pub iss: String,
    pub aud: String,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
    pub sub: String,
    pub nonce: String,
    pub c_hash: String,
    pub email: String,
    pub email_verified: String,
    pub is_private_email: String,
    pub auth_time: u64,
    pub nonce_supported: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GoogleClaims {
    pub iss: String,
    pub nbf: Option<u64>,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub hd: String,
    pub email: String,
    pub email_verified: bool,
    pub azp: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub iat: Option<u64>,
    pub exp: Option<u64>,
    pub jti: String,
}

impl CustomClaims {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let claims = serde_json::from_reader(reader)?;
        Ok(claims)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClaimsList {
    pub claims: Vec<CustomClaims>,
}

impl ClaimsList {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let claims = serde_json::from_reader(reader)?;
        Ok(claims)
    }
}
impl IntoIterator for ClaimsList {
    type Item = CustomClaims;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.claims.into_iter()
    }
}

trait ClaimId {
    fn primary_id(&self) -> Option<String>;
}

impl ClaimId for CustomClaims {
    fn primary_id(&self) -> Option<String> {
        if self.email.is_empty() {
            None
        } else {
            Some(self.email.clone())
        }
    }
}

impl ClaimId for AppleClaims {
    fn primary_id(&self) -> Option<String> {
        if self.email.is_empty() {
            None
        } else {
            Some(self.email.clone())
        }
    }
}

impl ClaimId for GoogleClaims {
    fn primary_id(&self) -> Option<String> {
        if self.email.is_empty() {
            None
        } else {
            Some(self.email.clone())
        }
    }
}

impl ClaimId for DecodedClaims {
    fn primary_id(&self) -> Option<String> {
        match self {
            DecodedClaims::Apple(claims) => claims.primary_id(),
            DecodedClaims::Google(claims) => claims.primary_id(),
            DecodedClaims::Test(claims) => claims.primary_id(),
        }
    }
}

#[derive(Error, Debug)]
pub enum OidcError {
    #[error("Failed to parse certificate")]
    CertificateParseError,
    #[error("Failed to decode token")]
    TokenDecodeError,
    #[error("Algorithm not found")]
    AlgorithmNotFoundError,
    #[error("Failed to generate token")]
    TokenGenerationError,
    #[error("Failed to validate token")]
    TokenValidationError,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum IdentityProvider {
    Apple,
    Google,
    Test,
}

pub fn encode_token(
    alg: &Rsa,
    claims: &CustomClaims,
    pkey: &RsaPrivateKey,
) -> Result<String, OidcError> {
    let header = Header::default();
    let claims = Claims::new(claims);
    let token = alg.token(header, &claims, pkey);
    match token {
        Ok(token) => Ok(token),
        Err(err) => {
            println!("Failed to generate token: {err:?}");
            Err(OidcError::TokenGenerationError)
        }
    }
}

pub fn decode_token(token: &str, provider: &IdentityProvider) -> Result<DecodedClaims, OidcError> {
    let token = UntrustedToken::new(token).map_err(|_| OidcError::TokenDecodeError)?;
    let certs = provider.certs()?;

    for key in certs.keys {
        let key_type = key.key_type();
        let (alg, vkey) = match key_type {
            KeyType::Rsa => {
                let vkey =
                    RsaPublicKey::try_from(&key).map_err(|_| OidcError::CertificateParseError)?;
                (Rsa::rs256(), vkey)
            }
            _ => return Err(OidcError::AlgorithmNotFoundError),
        };

        match provider {
            IdentityProvider::Apple => {
                let res = alg.validate_integrity::<AppleClaims>(&token, &vkey);
                if let Ok(validated_token) = res {
                    return Ok(DecodedClaims::Apple(
                        validated_token.claims().custom.clone(),
                    ));
                } else {
                    println!("Failed to validate token: {res:?}");
                }
            }
            IdentityProvider::Google => {
                let res = alg.validate_integrity::<GoogleClaims>(&token, &vkey);
                if let Ok(validated_token) = res {
                    return Ok(DecodedClaims::Google(
                        validated_token.claims().custom.clone(),
                    ));
                } else {
                    println!("Failed to validate token: {res:?}");
                }
            }
            IdentityProvider::Test => {
                let res = alg.validate_integrity::<CustomClaims>(&token, &vkey);
                if let Ok(validated_token) = res {
                    return Ok(DecodedClaims::Test(validated_token.claims().custom.clone()));
                } else {
                    println!("Failed to validate token: {res:?}");
                }
            }
        }
    }

    // If none of the keys worked, then the token is invalid.
    Err(OidcError::TokenValidationError)
}

impl<'a> IdentityProvider {
    pub fn certs(&self) -> Result<JwkKeys<'a>, OidcError> {
        match self {
            Self::Apple => serde_json::from_str::<JwkKeys>(APPLE_PUB_CERTS)
                .map_err(|_| OidcError::CertificateParseError),
            Self::Google => serde_json::from_str::<JwkKeys>(GOOGLE_PUB_CERTS)
                .map_err(|_| OidcError::CertificateParseError),

            Self::Test => serde_json::from_str::<JwkKeys>(TEST_PUB_CERTS)
                .map_err(|_| OidcError::CertificateParseError),
        }
    }

    pub fn validate(&self, token: &str) -> Result<(String, String), OidcError> {
        let decoded_token = decode_token(token, self)?;
        let identifier = decoded_token
            .primary_id()
            .ok_or(OidcError::TokenValidationError)?;
        let nonce = match decoded_token {
            DecodedClaims::Apple(claims) => claims.nonce.clone(),
            DecodedClaims::Google(claims) => claims.nonce.clone(),
            DecodedClaims::Test(claims) => claims.nonce.clone(),
        };
        Ok((identifier, nonce))
    }
}

#[derive(Deserialize, Serialize)]
pub struct JwkKeys<'a> {
    pub keys: Vec<JsonWebKey<'a>>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::IdentityProvider;
    // Test Payload:
    // {
    //     "iss": "https://accounts.google.com",
    //     "azp": "1234987819200.apps.googleusercontent.com",
    //     "aud": "1234987819200.apps.googleusercontent.com",
    //     "sub": "10769150350006150715113082367",
    //     "at_hash": "HK6E_P6Dh8Y93mRNtsDB1Q",
    //     "hd": "example.com",
    //     "email": "jsmith@example.com",
    //     "email_verified": true,
    //     "iat": 1353601026,
    //     "exp": 1853604926,
    //     "nonce": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    // }
    const TEST_JWT: &str = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjhlNGVhOWY1YjJkODIzNGU3OWYyYzczYTRiMTY4M2E0In0.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIxMjM0OTg3ODE5MjAwLmFwcHMuZ29vZ2xldXNlcmNvbnRlbnQuY29tIiwiYXVkIjoiMTIzNDk4NzgxOTIwMC5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsInN1YiI6IjEwNzY5MTUwMzUwMDA2MTUwNzE1MTEzMDgyMzY3IiwiYXRfaGFzaCI6IkhLNkVfUDZEaDhZOTNtUk50c0RCMVEiLCJoZCI6ImV4YW1wbGUuY29tIiwiZW1haWwiOiJqc21pdGhAZXhhbXBsZS5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiaWF0IjoxMzUzNjAxMDI2LCJleHAiOjE4NTM2MDQ5MjYsIm5vbmNlIjoiMHhmMzlGZDZlNTFhYWQ4OEY2RjRjZTZhQjg4MjcyNzljZmZGYjkyMjY2In0.C92XFrT8pm_Hg_omZ7l7SqSFMIYTaSk6eSGUPv0trW2I3ucKxXIVlMrFBywXq3-iCMAQ0Bv4W5BkML6okMbgRe7h6HPdoGvMGHjS3_oTxdWQMIWHrdrrMoh_jigenMamEQ_ONOHTiORdY0k5kQ6Z5cUoCV8sVMmhMZB55nS7wse_B7wAmNrAaqXmPkOCkX9qkbwPhPLCWzeRF7PYMHqgM4sHoDOjyEBVKBqbkHJnxVpwzXyMpwqpJvE7VMWU55QGaMqA8rt-KndBfLFTtxkyZn-go1MdvvxdNUAwq9bwNbuOitLaw3gIOTNxHFAF6Era0wQ8Nedz1wDIgjo9ppeDPA"#;

    // Payload from google:
    //     {
    //   "iss": "https://accounts.google.com",
    //   "azp": "873787331262-7bflj4fhoup1enlb055ggipqcjiuq68u.apps.googleusercontent.com",
    //   "aud": "873787331262-7bflj4fhoup1enlb055ggipqcjiuq68u.apps.googleusercontent.com",
    //   "sub": "108378151968747898733",
    //   "hd": "risczero.com",
    //   "email": "hans@risczero.com",
    //   "email_verified": true,
    //   "at_hash": "Zxs_8aXXov-eYopa9s5vZQ",
    //   "iat": 1693025866,
    //   "exp": 1693029466
    // }
    // const GOOGLE_JWT: &str = r#"eyJhbGciOiJSUzI1NiIsImtpZCI6ImY1ZjRiZjQ2ZTUyYjMxZDliNjI0OWY3MzA5YWQwMzM4NDAwNjgwY2QiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiI4NzM3ODczMzEyNjItN2JmbGo0ZmhvdXAxZW5sYjA1NWdnaXBxY2ppdXE2OHUuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiI4NzM3ODczMzEyNjItN2JmbGo0ZmhvdXAxZW5sYjA1NWdnaXBxY2ppdXE2OHUuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMDgzNzgxNTE5Njg3NDc4OTg3MzMiLCJoZCI6InJpc2N6ZXJvLmNvbSIsImVtYWlsIjoiaGFuc0ByaXNjemVyby5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiYXRfaGFzaCI6IkQwZEV0Xzl3YjBOSWk4blJZSUppcnciLCJub25jZSI6IjB4ZWZkRjk4NjFGM2VEYzI0MDQ2NDNCNTg4Mzc4RkUyNDJGQ2FkRTY1OCIsImlhdCI6MTY5OTEzODk5NCwiZXhwIjoxNjk5MTQyNTk0fQ.U5rDQ2dmMD6-Nfd-l-QaM5d9suETDg5GLA0aBCkDm4AQKy5YlKu3mqHvgwrW586qvbnuUYt4T9T5na9GiQzYUs-QeRqcaH4J2N4oFF9YYs7Uf4cZ1W9-RHsZMuFAMbFSFBLBh_WjTGGat1fgx5khI9N7-_0P48GT_anDvQcbvgPIQyZ_hjRC5AxwzOObPh2hVJXAVgyYY5bcbhlRa4HzxpIbJcVWtEeaz95uLlL-YxXdNrkc_Tjj-GtA9MuH1Hm7rr-T8EGKREz-jH0Q2qw3Y1qomp50vEfLBLeRXwYfjdnNiSBQAfvh_g0jJ86zTQfYirjna7XEHNLYKvQt2DZY4Q"#;
    //
    //

    // Payload from google:
    // {
    //   "iss": "https://accounts.google.com",
    //   "nbf": 1699257448,
    //   "aud": "873787331262-7bflj4fhoup1enlb055ggipqcjiuq68u.apps.googleusercontent.com",
    //   "sub": "108378151968747898733",
    //   "nonce": "0xefdF9861F3eDc2404643B588378FE242FCadE658",
    //   "hd": "risczero.com",
    //   "email": "hans@risczero.com",
    //   "email_verified": true,
    //   "azp": "873787331262-7bflj4fhoup1enlb055ggipqcjiuq68u.apps.googleusercontent.com",
    //   "name": "Hans Martin",
    //   "picture": "https://lh3.googleusercontent.com/a/ACg8ocICKeD4gcH7nrSh7gqQT1rk0mZRSSpyL28aL4rzDJ2p=s96-c",
    //   "given_name": "Hans",
    //   "family_name": "Martin",
    //   "iat": 1699257748,
    //   "exp": 1699261348,
    //   "jti": "d4c3a453c373e20fe4f9683e6af7e3247617099f"
    // }
    //
    const GOOGLE_JWT: &str = r#"eyJhbGciOiJSUzI1NiIsImtpZCI6ImY1ZjRiZjQ2ZTUyYjMxZDliNjI0OWY3MzA5YWQwMzM4NDAwNjgwY2QiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJuYmYiOjE2OTkyNTc0NDgsImF1ZCI6Ijg3Mzc4NzMzMTI2Mi03YmZsajRmaG91cDFlbmxiMDU1Z2dpcHFjaml1cTY4dS5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsInN1YiI6IjEwODM3ODE1MTk2ODc0Nzg5ODczMyIsIm5vbmNlIjoiMHhlZmRGOTg2MUYzZURjMjQwNDY0M0I1ODgzNzhGRTI0MkZDYWRFNjU4IiwiaGQiOiJyaXNjemVyby5jb20iLCJlbWFpbCI6ImhhbnNAcmlzY3plcm8uY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsImF6cCI6Ijg3Mzc4NzMzMTI2Mi03YmZsajRmaG91cDFlbmxiMDU1Z2dpcHFjaml1cTY4dS5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsIm5hbWUiOiJIYW5zIE1hcnRpbiIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BQ2c4b2NJQ0tlRDRnY0g3bnJTaDdncVFUMXJrMG1aUlNTcHlMMjhhTDRyekRKMnA9czk2LWMiLCJnaXZlbl9uYW1lIjoiSGFucyIsImZhbWlseV9uYW1lIjoiTWFydGluIiwiaWF0IjoxNjk5MjU3NzQ4LCJleHAiOjE2OTkyNjEzNDgsImp0aSI6ImQ0YzNhNDUzYzM3M2UyMGZlNGY5NjgzZTZhZjdlMzI0NzYxNzA5OWYifQ.Su-su8VuGAT_kJY0tUZVuRjM37cLfP96qav0Ekk5xtZrSZmB8QaOjFTn-QkRYw17nTgxV_Zq_qv6uYOQ6IcfJUtN-BD_qT91BX7aQQFPYGNJ9rHcyzoh9yK_uz1cGVka4cCKRwW3wEZ1nwPKRxYguDFO_A3UrjwPeZFLj9ns82FBqbvD0AkEWqPm-gZwUzwzAPjz3017UbcjJkng5O_2-hlG0dLKoS-KIfXr6r8nkVjUXHoSxGx3RbTj16HLUDEklSr7qu-0cSSjDFVd1m3HxJeKnQBOctTEOzbQRovHPTNhDSBMXn6Bw47nd3-fmQ_wcnBEWuF92CNQp8cV0AhuSw"#;

    // Payload from apple:
    // {
    //   "iss": "https://appleid.apple.com",
    //   "aud": "com.r0.bonsai.pay.demo.sid",
    //   "exp": 1699306091,
    //   "iat": 1699219691,
    //   "sub": "001650.a6dce727c8404e0997afce4b5ac65691.2256",
    //   "nonce": "0xefdF9861F3eDc2404643B588378FE242FCadE658",
    //   "c_hash": "LxferYMTNsDT-1dSiITyng",
    //   "email": "tsfs5tj5vp@privaterelay.appleid.com",
    //   "email_verified": "true",
    //   "is_private_email": "true",
    //   "auth_time": 1699219691,
    //   "nonce_supported": true
    // }
    const APPLE_JWT: &str = r#"eyJraWQiOiJXNldjT0tCIiwiYWxnIjoiUlMyNTYifQ.eyJpc3MiOiJodHRwczovL2FwcGxlaWQuYXBwbGUuY29tIiwiYXVkIjoiY29tLnIwLmJvbnNhaS5wYXkuZGVtby5zaWQiLCJleHAiOjE2OTkzMDYwOTEsImlhdCI6MTY5OTIxOTY5MSwic3ViIjoiMDAxNjUwLmE2ZGNlNzI3Yzg0MDRlMDk5N2FmY2U0YjVhYzY1NjkxLjIyNTYiLCJub25jZSI6IjB4ZWZkRjk4NjFGM2VEYzI0MDQ2NDNCNTg4Mzc4RkUyNDJGQ2FkRTY1OCIsImNfaGFzaCI6Ikx4ZmVyWU1UTnNEVC0xZFNpSVR5bmciLCJlbWFpbCI6InRzZnM1dGo1dnBAcHJpdmF0ZXJlbGF5LmFwcGxlaWQuY29tIiwiZW1haWxfdmVyaWZpZWQiOiJ0cnVlIiwiaXNfcHJpdmF0ZV9lbWFpbCI6InRydWUiLCJhdXRoX3RpbWUiOjE2OTkyMTk2OTEsIm5vbmNlX3N1cHBvcnRlZCI6dHJ1ZX0.mWwIl7-DPwtA7GpEOjsnbKjBPbafdMD5iz1ctfPtMD66p8kg6n15c2uMWSbrGme4VU2CZj0pJxnE3PQHzIXjAP7T_AY3SbqaOM8XUCKkIDP9pohjrvRgZHtINh3GZyRF0SyvjfgeJ65P__Vx3gYdnoESEOuZzzvMFxMIvwpC6KWgurbfqsnyS1QAyKAFRUgN-hDr3IVMMW6rchu6CgfZxbwt9GtF_35KvnOmliAQsNn9MEiGjDyP0qVZL5EvPmTu1rCC9qirJAgvnnR_cxRuDUeaUPC7R45gT_QUqNakTbIQDZcsL2y1ccyh_QrMn8CxDHt7hAK4MaL1cvEoT7cm1w"#;

    #[test]
    fn check_google_certs() {
        let provider = IdentityProvider::Google;
        let _certs = provider.certs();
    }

    #[test]
    fn check_test_certs() {
        let provider = IdentityProvider::Test;
        let _certs = provider.certs();
    }

    #[test]
    fn test_certificate_parsing() {
        let google_certs: serde_json::Value = serde_json::from_str(GOOGLE_PUB_CERTS).unwrap();
        let test_priv_certs: serde_json::Value = serde_json::from_str(TEST_PRIV_CERTS).unwrap();

        assert_eq!(google_certs["keys"][0]["alg"], "RS256");
        assert_eq!(test_priv_certs["keys"][0]["alg"], "RS256");
    }

    #[test]
    fn test_error_handling() {
        let result = CustomClaims {
            iss: "".to_string(),
            azp: "".to_string(),
            aud: "".to_string(),
            sub: "".to_string(),
            email: "".to_string(),
            email_verified: false,
            at_hash: "".to_string(),
            iat: Some(0),
            exp: Some(0),
            nonce: "".to_string(),
        }
        .primary_id();

        assert!(result.is_none());
    }

    #[test]
    fn test_validate_jwt_valid_token() {
        let valid_token = TEST_JWT;
        let identity_provider = IdentityProvider::Test;

        let (ident, _addr) = identity_provider.validate(valid_token).unwrap();

        assert_eq!(ident, "jsmith@example.com");
    }

    #[test]
    fn test_validate_jwt_invalid_token() {
        let valid_token = TEST_JWT;
        let identity_provider = IdentityProvider::Google; // Wrong provider

        let result = identity_provider.validate(valid_token);

        assert!(matches!(result, Err(OidcError::TokenValidationError)));
    }

    #[test]
    // ignored to update the google JWT with a nonce
    fn test_validate_google_jwt_valid_token() {
        let valid_token = GOOGLE_JWT;
        let identity_provider = IdentityProvider::Google;

        let (ident, _addr) = identity_provider.validate(valid_token).unwrap();

        assert_eq!(ident, "hans@risczero.com");
    }

    #[test]
    fn test_validate_apple_jwt_valid_token() {
        let valid_token = APPLE_JWT;
        let identity_provider = IdentityProvider::Apple;

        let (ident, _addr) = identity_provider.validate(valid_token).unwrap();

        assert_eq!(ident, "tsfs5tj5vp@privaterelay.appleid.com");
    }

    #[test]
    fn test_encode_jwt() {
        // Replicate the payload from the test example
        let claims = CustomClaims {
            iss: "https://accounts.google.com".to_string(),
            azp: "1234987819200.apps.googleusercontent.com".to_string(),
            aud: "1234987819200.apps.googleusercontent.com".to_string(),
            sub: "10769150350006150715113082367".to_string(),
            email: "jsmith@example.com".to_string(),
            email_verified: true,
            at_hash: "HK6E_P6Dh8Y93mRNtsDB1Q".to_string(),
            iat: Some(1353601026),
            exp: Some(1853604926),
            nonce: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".to_string(),
        };
        let identity_provider = IdentityProvider::Test;
        let result_a = identity_provider.validate(TEST_JWT).unwrap();

        let key = serde_json::from_str::<JwkKeys>(TEST_PRIV_CERTS).unwrap();
        let alg = Rsa::rs256();
        let pkey = RsaPrivateKey::try_from(&key.keys[0]).unwrap();
        let token = encode_token(&alg, &claims, &pkey).unwrap();

        let result_b = identity_provider.validate(&token).unwrap();

        assert_eq!(result_a, result_b);
    }

    #[test]
    fn test_decode_jwt() {
        // decode_token
        let identity_provider = IdentityProvider::Google;
        let result = decode_token(GOOGLE_JWT, &identity_provider).unwrap();

        println!("Result: {:?}", result);
    }
}
