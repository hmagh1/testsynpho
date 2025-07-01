use symphonia::default::*;
use symphonia::core::io::MediaSourceStream;
use std::fs::File;

fn main() {
    let path = "example.mp3";
    let file = File::open(path).expect("Fichier introuvable.");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe().format(
        &Default::default(),
        mss,
        &Default::default(),
        &Default::default(),
    ).expect("Échec détection format");

    let format = probed.format;
    println!("Format détecté : {:?}", format);
}

