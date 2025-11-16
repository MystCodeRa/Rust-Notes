use std::env;
use std::fs::OpenOptions;
use std::io::{Write, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Kullanım:");
        println!("  notes add \"mesaj\"");
        println!("  notes list");
        println!("  notes clear");
        return;
    }

    let command = args[1].as_str();

    match command {
        "add" => add_note(&args),
        "list" => list_notes(),
        "clear" => clear_notes(),
        _ => println!("Bilinmeyen komut: {}", command),
    }
}

fn add_note(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Lütfen bir not girin.");
        return;
    }

    let note = &args[2];

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("Dosya açılamadı!");

    writeln!(file, "{}", note).unwrap();

    println!("Not eklendi: {}", note);
}

fn list_notes() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("notes.txt")
        .unwrap_or_else(|_| OpenOptions::new().create(true).open("notes.txt").unwrap());

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    if content.trim().is_empty() {
        println!("Hiç not yok.");
    } else {
        println!("--- Notlar ---");
        for (i, line) in content.lines().enumerate() {
            println!("{}. {}", i + 1, line);
        }
    }
}

fn clear_notes() {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(b"").unwrap();
    println!("Tüm notlar silindi!");
}
