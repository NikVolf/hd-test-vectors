extern crate bitcoin;

use bitcoin::util::base58::FromBase58;
use std::fmt::Write;

fn hex(slice: &[u8]) -> String {
    let mut s = String::new();
    for &byte in slice {
        write!(&mut s, "{:02x}", byte).unwrap();
    }
    s
}

fn show_private(s: &str) {
    let v: Vec<u8> = FromBase58::from_base58check(s).unwrap();
    assert_eq!(v.len(), 78);

    let private = v[46..78].to_vec();
    let mut reversed_private = private.clone();
    reversed_private.reverse();

    assert_eq!(private.len(), 32);

    println!("private key: {} ", s);
    println!("     normal: {}", hex(&private));
    println!("   reversed: {}", hex(&reversed_private));
}

fn show_public(s: &str) {
   let v: Vec<u8> = FromBase58::from_base58check(s).unwrap();
    assert_eq!(v.len(), 78);

    let public = v[45..78].to_vec();
    let mut reversed_public = public.clone();
    reversed_public.reverse();

    assert_eq!(public.len(), 33);

    println!(" public key: {}", s);
    println!("     normal: {}", hex(&public));
    println!("   reversed: {}\n", hex(&reversed_public));
}

fn main() {
    show_private("xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");
    show_public("xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8");

    show_private("xprv9uHRZZhk6KAJC1avXpDAp4MDc3sQKNxDiPvvkX8Br5ngLNv1TxvUxt4cV1rGL5hj6KCesnDYUhd7oWgT11eZG7XnxHrnYeSvkzY7d2bhkJ7");
    show_public("xpub68Gmy5EdvgibQVfPdqkBBCHxA5htiqg55crXYuXoQRKfDBFA1WEjWgP6LHhwBZeNK1VTsfTFUHCdrfp1bgwQ9xv5ski8PX9rL2dZXvgGDnw");

    show_private("xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs");
    show_public("xpub6ASuArnXKPbfEwhqN6e3mwBcDTgzisQN1wXN9BJcM47sSikHjJf3UFHKkNAWbWMiGj7Wf5uMash7SyYq527Hqck2AxYysAA7xmALppuCkwQ");
}
