use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_to_string<T: AsRef<[u8]>>(data: T) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(data.as_ref(), &salt)
        .map(|hash| hash.to_string())
}

pub fn verify_hash<T>(hash: &str, data: T) -> bool
where
    T: AsRef<[u8]>,
{
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(data.as_ref(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod test {
    use crate::crypto::verify_hash;

    use super::hash_to_string;

    #[test]
    fn should_hash_and_verify() {
        let string = "hello_world";
        let hash = hash_to_string(string).unwrap();
        assert!(verify_hash(&hash, string))
    }
}
