use clap::{App, Arg};
use sled::{open, IVec};

fn main() {
    let matches = App::new("CFS Geez")
        .version("0.1")
        .author("Nithsua <nivasmuthu452@gmail.com>")
        .about("Cryptographic File System for your text files")
        .arg(
            Arg::with_name("encrypt")
                .short("c")
                .long("encrypt")
                .help("Encypt your text file"),
        )
        .arg(
            Arg::with_name("decrypt")
                .short("x")
                .long("decrypt")
                .help("Decrypt your text file"),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .help("delete a entry from the fs"),
        )
        .arg(
            Arg::with_name("FILE")
                .help("name of the input or output file")
                .required(true),
        )
        .arg(
            Arg::with_name("KEY")
                .help("key to store or retrive the file")
                .required(true),
        )
        .get_matches();

    let mut args = matches.args;
    // println!("{:?}", args);
    let temp = args.remove("FILE").unwrap();
    let path = temp.vals[0].to_str().unwrap();

    let temp = args.remove("KEY").unwrap();
    let key = temp.vals[0].to_str().unwrap();

    if args.len() == 0 {
        println!("Give a option(FLAG), use --help for more info");
        return;
    } else if args.len() > 1 {
        println!("Give atmost one option(FLAG), use --help for more info");
        return;
    }

    let option = *args.keys().nth(0).unwrap();
    match option {
        "encrypt" => {
            println!("path: {}\nkey: {}\nEncryption", path, key);
            encrypt(path, key);
        }
        "decrypt" => {
            println!("path: {}\nkey: {}\nDecryption", path, key);
            decrypt(path, key);
        }
        "delete" => {
            delete_from_local_store(key);
        }
        _ => {}
    }
}

fn encrypt<'a>(file_path: &'a str, key: &str) -> Result<(), &'a str> {
    if key_exists(key) {
        return Err("Key Exists");
    }

    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            add_to_local_store(key, content);
            Ok(())
        }
        Err(_) => Err("Unable to open the file, file not available or not enough permission"),
    }
}

fn decrypt<'a>(file_path: &'a str, key: &str) -> Result<(), &'a str> {
    let content = read_from_local_store(key);
    println!("{} {}", content, file_path);
    std::fs::write(file_path, content);
    Ok(())
}

fn key_exists(key: &str) -> bool {
    let home_dir = std::env::var("HOME").unwrap();
    let local_store =
        open(format!("{}/.cerostore", home_dir)).expect("Error while opening the container fs");
    local_store.contains_key(key).unwrap()
}

fn add_to_local_store<K: std::convert::AsRef<[u8]>, V: serde::Serialize>(
    key: K,
    value: V,
) -> Result<Option<IVec>, sled::Error> {
    let home_dir = std::env::var("HOME").unwrap();
    // println!("{}", home_dir);
    let local_store = open(format!("{}/.cerostore", home_dir)).expect("Open");
    let serialized_data = bincode::serialize(&value).unwrap();
    local_store.insert(key, serialized_data)
}

pub fn read_from_local_store<K: std::convert::AsRef<[u8]>>(key: K) -> String {
    let home_dir = std::env::var("HOME").unwrap();
    let local_store = open(format!("{}/.cerostore", home_dir)).expect("Open");
    let serialized_data = local_store.get(key).unwrap().unwrap().to_vec();
    let content: String = bincode::deserialize(&serialized_data).unwrap();
    content
}

pub fn delete_from_local_store<K: AsRef<[u8]>>(key: K) -> Result<(), &'static str> {
    let home_dir = std::env::var("HOME").unwrap();
    let local_store = open(format!("{}/.cerostore", home_dir)).expect("Open");
    match local_store.remove(key) {
        Ok(_) => Ok(()),
        Err(_) => Err("Key doesn't exists"),
    }
}
