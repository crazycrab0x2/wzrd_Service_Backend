

use bip39::{Mnemonic, Language, MnemonicType}; 



fn main() {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase = mnemonic.phrase().to_string();
    println!("{}", phrase);
    let word_list:Vec<Vec<u8>> = phrase.split_whitespace().map(|word| word.as_bytes().to_vec()).collect();
    println!("{:?}", word_list)

    
    // let phrase = "soldier mind mammal narrow picture token foster horror enter matter audit cost";
    // let mnemonic = Mnemonic::from_phrase(phrase, Language::English).unwrap();
    // let seed = Seed::new(&mnemonic, "gjthkaudlk");

    // let root_xprv = XPrv::new(&seed).unwrap();
    // let child_path = "m/0/2147483647'/1/2147483646'";
    // let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse().unwrap()).unwrap();
    // let child_xpub = child_xprv.public_key();

    // println!("{:?}",child_xprv.private_key());
    // println!("{:?}",child_xpub.public_key());

    // println!("{}", hex::encode(seed.as_bytes()));
    // println!("{:x}", seed);
    // println!("{:X}", mnemonic);
}
