use clap::{App, Arg};

fn main() {
    let matches = App::new("cfsgeez")
        .version("0.1")
        .author("Nithsua <nivasmuthu452@gmail.com>")
        .about("Cryptographic File System for your text files")
        .arg(
            Arg::with_name("encrypt")
                .short("c")
                .long("encrypt")
                .help("Encypt your text file"),
        )
        // .required(true))
        .arg(
            Arg::with_name("decrypt")
                .short("x")
                .long("decrypt")
                .help("Decrypt your text file"),
        )
        // .required(true))
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

    let path = &args.remove("FILE").unwrap().vals[0];
    let key = &args.remove("KEY").unwrap().vals[0];

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
            println!("path: {:?}\nkey: {:?}\nEncryption", path, key);
        }
        "decrypt" => {
            println!("path: {:?}\nkey: {:?}\nDecryption", path, key);
        }
        _ => {}
    }
}
