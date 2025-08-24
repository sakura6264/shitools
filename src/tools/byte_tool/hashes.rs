use digest::Digest;

pub fn asconhash(input: &[u8]) -> String {
    let mut hasher = ascon_hash::AsconHash256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn asconahash(input: &[u8]) -> String {
    let mut hasher = ascon_hash::AsconHash256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn belt(input: &[u8]) -> String {
    let mut hasher = belt_hash::BeltHash::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn blake3(input: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result.as_bytes())
}

pub fn fsb160(input: &[u8]) -> String {
    let mut hasher = fsb::Fsb160::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn fsb256(input: &[u8]) -> String {
    let mut hasher = fsb::Fsb256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn fsb512(input: &[u8]) -> String {
    let mut hasher = fsb::Fsb512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn fsb224(input: &[u8]) -> String {
    let mut hasher = fsb::Fsb224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn fsb384(input: &[u8]) -> String {
    let mut hasher = fsb::Fsb384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn gost94ua(input: &[u8]) -> String {
    let mut hasher = gost94::Gost94UA::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn gost94cryptopro(input: &[u8]) -> String {
    let mut hasher = gost94::Gost94CryptoPro::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn gost94test(input: &[u8]) -> String {
    let mut hasher = gost94::Gost94Test::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn gost94s2015(input: &[u8]) -> String {
    let mut hasher = gost94::Gost94s2015::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn groestl224(input: &[u8]) -> String {
    let mut hasher = groestl::Groestl224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn groestl256(input: &[u8]) -> String {
    let mut hasher = groestl::Groestl256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn groestl384(input: &[u8]) -> String {
    let mut hasher = groestl::Groestl384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn groestl512(input: &[u8]) -> String {
    let mut hasher = groestl::Groestl512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn jh224(input: &[u8]) -> String {
    let mut hasher = jh::Jh224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn jh256(input: &[u8]) -> String {
    let mut hasher = jh::Jh256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn jh384(input: &[u8]) -> String {
    let mut hasher = jh::Jh384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn jh512(input: &[u8]) -> String {
    let mut hasher = jh::Jh512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn md2(input: &[u8]) -> String {
    let mut hasher = md2::Md2::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn md4(input: &[u8]) -> String {
    let mut hasher = md4::Md4::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn md5(input: &[u8]) -> String {
    let mut hasher = md5::Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn ripemd128(input: &[u8]) -> String {
    let mut hasher = ripemd::Ripemd128::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn ripemd160(input: &[u8]) -> String {
    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn ripemd256(input: &[u8]) -> String {
    let mut hasher = ripemd::Ripemd256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn ripemd320(input: &[u8]) -> String {
    let mut hasher = ripemd::Ripemd320::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha1(input: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha224(input: &[u8]) -> String {
    let mut hasher = sha2::Sha224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha256(input: &[u8]) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha384(input: &[u8]) -> String {
    let mut hasher = sha2::Sha384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha512(input: &[u8]) -> String {
    let mut hasher = sha2::Sha512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha512_224(input: &[u8]) -> String {
    let mut hasher = sha2::Sha512_224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha512_256(input: &[u8]) -> String {
    let mut hasher = sha2::Sha512_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha3_224(input: &[u8]) -> String {
    let mut hasher = sha3::Sha3_224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha3_256(input: &[u8]) -> String {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha3_384(input: &[u8]) -> String {
    let mut hasher = sha3::Sha3_384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sha3_512(input: &[u8]) -> String {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn keccak224(input: &[u8]) -> String {
    let mut hasher = sha3::Keccak224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn keccak256(input: &[u8]) -> String {
    let mut hasher = sha3::Keccak256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn keccak384(input: &[u8]) -> String {
    let mut hasher = sha3::Keccak384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn keccak512(input: &[u8]) -> String {
    let mut hasher = sha3::Keccak512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn keccak256full(input: &[u8]) -> String {
    let mut hasher = sha3::Keccak256Full::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn shabal192(input: &[u8]) -> String {
    let mut hasher = shabal::Shabal192::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn shabal224(input: &[u8]) -> String {
    let mut hasher = shabal::Shabal224::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn shabal256(input: &[u8]) -> String {
    let mut hasher = shabal::Shabal256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn shabal384(input: &[u8]) -> String {
    let mut hasher = shabal::Shabal384::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn shabal512(input: &[u8]) -> String {
    let mut hasher = shabal::Shabal512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sm3(input: &[u8]) -> String {
    let mut hasher = sm3::Sm3::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn streebog256(input: &[u8]) -> String {
    let mut hasher = streebog::Streebog256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn streebog512(input: &[u8]) -> String {
    let mut hasher = streebog::Streebog512::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn tiger(input: &[u8]) -> String {
    let mut hasher = tiger::Tiger::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn tiger2(input: &[u8]) -> String {
    let mut hasher = tiger::Tiger2::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn whirlpool(input: &[u8]) -> String {
    let mut hasher = whirlpool::Whirlpool::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}
