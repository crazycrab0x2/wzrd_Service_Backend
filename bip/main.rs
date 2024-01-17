// use bip39::{Mnemonic, MnemonicType, Language, Seed}; 
// use hex;
// use bip32::XPrv;


// fn main() {
//     let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
//     let phrase = mnemonic.phrase();
//     let seed = Seed::new(&mnemonic, "");

//     // let phrase = "park remain person kitchen mule spell knee armed position rail grid ankle";
//     // let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();

//     let root_xprv = XPrv::new(&seed).unwrap();
//     let child_path = "m/0/2147483647'/1/2147483646'";
//     let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().unwrap()).unwrap();
//     let child_xpub = child_xprv.public_key();

//     println!("{:?}",child_xprv.private_key());
//     println!("{:?}",child_xpub.public_key());

//     println!("{}", phrase);
//     println!("{}", hex::encode(seed.as_bytes()));
//     println!("{:x}", seed);
//     println!("{:X}", mnemonic);
// }


use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

fn main() {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    let token_str = claims.sign_with_key(&key).unwrap();
    println!("{}", token_str);
    let veri_claims: BTreeMap<String, String> = token_str.as_str().verify_with_key(&key).unwrap();
    println!("{}", veri_claims["sub"]);
}