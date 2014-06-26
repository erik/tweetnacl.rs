use bindings::*;

pub struct CryptoBox {
    pub keypair: Keypair,
}

pub type SecretKey = [u8, ..32];
pub type PublicKey = [u8, ..32];

pub struct Keypair {
    pub pk: PublicKey,
    pub sk: SecretKey
}

impl Keypair {
    pub fn new() -> Keypair {
        unsafe {
            let mut pk = [0u8, ..32];
            let mut sk = [0u8, ..32];

            crypto_box_keypair(pk.as_mut_ptr(), sk.as_mut_ptr());

            Keypair { pk: pk, sk: sk }
        }
    }
}

impl CryptoBox {
    pub fn new() -> CryptoBox {
        CryptoBox { keypair: Keypair::new() }
    }

    pub fn new_with_key(key: Keypair) -> CryptoBox {
        CryptoBox { keypair: key }
    }

    pub fn encrypt(&self, msg: &[u8], key: PublicKey) -> (Vec<u8>, Vec<u8>) {
        unsafe {
            let mut nonce: Vec<u8> = Vec::with_capacity(24);
            let mut cipher: Vec<u8> = Vec::with_capacity(msg.len());

            randombytes(nonce.as_mut_ptr(), 24);
            nonce.set_len(24);

            crypto_box(cipher.as_mut_ptr(),
                       msg.as_ptr(),
                       msg.len() as u64,
                       nonce.as_ptr(),
                       key.as_ptr(),
                       self.keypair.sk.as_ptr());

            cipher.set_len(msg.len());
            (cipher, nonce)
        }
    }

    pub fn decrypt(&self, cipher: &[u8], nonce: &[u8], key: PublicKey) -> Vec<u8> {
        unsafe {
            let mut msg: Vec<u8> = Vec::with_capacity(cipher.len());

            crypto_box_open(msg.as_mut_ptr(),
                            cipher.as_ptr(),
                            cipher.len() as u64,
                            nonce.as_ptr(),
                            key.as_ptr(),
                            self.keypair.sk.as_ptr());

            msg.set_len(cipher.len());
            msg
        }
    }
}
