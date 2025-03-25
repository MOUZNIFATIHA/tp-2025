use std::io;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn main() {
   
    if env::args().len() != 3 {
        eprintln!("Usage: {} <source_file> <destination_file>", env::args().next().unwrap());
        process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    let source_path = &args[1];
    let destination_path = &args[2];

    let mut source_file = match File::open(source_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Erreur lors de l'ouverture du fichier source: {}", e);
            process::exit(1);
        }
    };

    let mut destination_file = match File::create(destination_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Erreur lors de la création du fichier destination: {}", e);
            process::exit(1);
        }
    };

    let mut buffer = Vec::new();
    if let Err(e) = source_file.read_to_end(&mut buffer) {
        eprintln!("Erreur lors de la lecture du fichier source: {}", e);
        process::exit(1);
    }

    if let Err(e) = destination_file.write_all(&buffer) {
        eprintln!("Erreur lors de l'écriture dans le fichier destination: {}", e);
        process::exit(1);
    }

    println!("Le fichier a été copié avec succès de '{}' vers '{}'", source_path, destination_path);
}

