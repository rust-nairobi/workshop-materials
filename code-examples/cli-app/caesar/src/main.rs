extern crate clap;

use clap::{App, Arg, SubCommand};

fn encrypt(plaintext: &str, key: usize) -> String {
    let plaintext = &plaintext.to_uppercase();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut cipher = String::from("");

    for c in plaintext.chars() {
        match alphabet.find(c) {
            Some(index) => {
                let cindex = (index + key) % 26;
                let cchar = alphabet.chars().nth(cindex).unwrap();
                cipher.push(cchar);
            }
            None => cipher.push(c),
        }
    }

    cipher
}

fn decrypt(ciphertext: &str, key: usize) -> String {
    let ciphertext = &ciphertext.to_uppercase();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut plaintext = String::from("");

    for c in ciphertext.chars() {
        match alphabet.find(c) {
            Some(index) => {
                let cindex = (index - key) % 26;
                let cchar = alphabet.chars().nth(cindex).unwrap();
                plaintext.push(cchar);
            }
            None => plaintext.push(c),
        }
    }

    plaintext
}

fn main() {
    let encrypt_subcommand = SubCommand::with_name("encrypt")
        .about("encrypt plaintext")
        .arg(
            Arg::with_name("plaintext")
                .long("plaintext")
                .short("p")
                .takes_value(true)
                .help("plaintext to be encrypted")
                .required(true),
        )
        .arg(
            Arg::with_name("key")
                .long("key")
                .short("k")
                .takes_value(true)
                .help("encryption key")
                .required(true),
        );
    let decrypt_subcommand = SubCommand::with_name("decrypt")
        .about("decrypt ciphertext")
        .arg(
            Arg::with_name("ciphertext")
                .long("ciphertext")
                .short("c")
                .takes_value(true)
                .help("ciphertext to be decrypted")
                .required(true),
        )
        .arg(
            Arg::with_name("key")
                .long("key")
                .short("k")
                .takes_value(true)
                .help("decryption key")
                .required(true),
        );

    let cli = App::new("caesar")
        .version("0.0.1")
        .author("Matt Gathu")
        .about("caesar cipher")
        .subcommand(encrypt_subcommand)
        .subcommand(decrypt_subcommand);

    let matches = cli.get_matches();

    if let Some(matches) = matches.subcommand_matches("encrypt") {
        let plaintext = matches.value_of("plaintext").unwrap();
        let key: usize = matches.value_of("key").unwrap().parse().unwrap();
        let ciphertext = encrypt(plaintext, key);
        println!("ciphertext: {}", ciphertext);
    } else if let Some(matches) = matches.subcommand_matches("decrypt") {
        let ciphertext = matches.value_of("ciphertext").unwrap();
        let key: usize = matches.value_of("key").unwrap().parse().unwrap();
        let plaintext = decrypt(ciphertext, key);
        println!("plaintext: {}", plaintext);
    }
}

#[cfg(test)]
mod tests {
    use encrypt;

    #[test]
    fn encryption_works() {
        assert_eq!(encrypt("hello", 3), "KHOOR");
    }
}
