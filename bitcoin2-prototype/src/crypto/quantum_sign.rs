use oqs::*;
use std::error::Error;

pub struct QuantumSigner {
    sig: sig::Sig,
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

impl QuantumSigner {
    pub fn new(alg_name: &str) -> Result<Self, Box<dyn Error>> {
        let sig = sig::Sig::new(sig::Algorithm::from_name(alg_name)?;
        let (public_key, secret_key) = sig.keypair()?;
        
        Ok(Self {
            sig,
            public_key: public_key.into_vec(),
            secret_key: secret_key.into_vec(),
        })
    }

    pub fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let signature = self.sig.sign(message, &self.secret_key)?;
        Ok(signature.into_vec())
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool, Box<dyn Error>> {
        Ok(self.sig.verify(message, signature, &self.public_key)?)
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
}

pub struct QuantumEncryption {
    kem: kem::Kem,
    public_key: Vec<u8>,
    secret_key: Vec<u8>,
}

impl QuantumEncryption {
    pub fn new(alg_name: &str) -> Result<Self, Box<dyn Error>> {
        let kem = kem::Kem::new(kem::Algorithm::from_name(alg_name))?;
        let (public_key, secret_key) = kem.keypair()?;
        
        Ok(Self {
            kem,
            public_key: public_key.into_vec(),
            secret_key: secret_key.into_vec(),
        })
    }

    pub fn encrypt(&self, public_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let (ciphertext, shared_secret) = self.kem.encapsulate(public_key)?;
        Ok((ciphertext.into_vec(), shared_secret.into_vec()))
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let shared_secret = self.kem.decapsulate(ciphertext, &self.secret_key)?;
        Ok(shared_secret.into_vec())
    }
}