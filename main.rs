use to_binary::{BinaryString,BinaryError};
use std::io::stdin;
use clap::{Arg, Command};

fn main() {

    let matches = Command::new("To String Converter")
        .version("0.1.0")
        .author("lux-cut <lucas.charignon@protonmail.com>")
        .about("String Converter")
        .arg(Arg::new("binary")
                 .short('b')
                 .long("binary")
                 .takes_value(false)
                 .help("Traduct string in binary"))
        .arg(Arg::new("hexadecimal")
                 .short('x')
                 .long("hexa")
                 .takes_value(false)
                 .help("Traduct string to hexadecimale"))
        .arg(Arg::new("octal")
                 .short('o')
                 .long("octal")
                 .takes_value(false)
                 .help("Traduct string to octal"))

        .get_matches();

    if matches.is_present("binary") {

	binary();

    }

    if matches.is_present("hexadecimal") {

        hexa();

    }  

    if matches.is_present("octal") {

        octal();

    }


}

fn binary() {

    let mut name = String::new().to_string();
    let mut name_in_binary = "".to_string();

    stdin()

        .read_line(&mut name)
        .expect("Erreur");

    for character in name.clone().as_bytes() {
        name_in_binary += &format!("0{:b} ", character);
    }
    println!("{} in binary is {}", name, name_in_binary);
}

fn hexa() {

    let mut name = String::new().to_string();
    let mut name_in_binary = "".to_string();

    stdin()

        .read_line(&mut name)
        .expect("Erreur");

    for character in name.clone().as_bytes() {
        name_in_binary += &format!("0{:x} ", character);
    }
    println!("{} in hexadecimal lowercase is {}", name, name_in_binary);

 }

fn octal() {

    let mut name = String::new().to_string();
    let mut name_in_binary = "".to_string();

    stdin()

        .read_line(&mut name)
        .expect("Erreur");

    for character in name.clone().as_bytes() {
        name_in_binary += &format!("0{:o} ", character);
    }
    println!("{}: in octal is {}", name, name_in_binary);
}
