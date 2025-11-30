use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use rand::Rng;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" | "help" => {
                println!("=== HANGMAN ===");
                println!("  hangman = joc interactiv");
                println!("  hangman --help | -h | help = afiseaza asta");
                return Ok(());
            }
            _ => {
                println!("Comanda necunoscuta: {}", args[1]);
                println!("Foloseste 'hangman --help' pentru ajutor");
                return Ok(());
            }
        }
    }

    loop {
        println!("\n=== HANGMAN ===");
        println!("Scrie 'go' pentru a incepe, 'help' pentru ajutor sau 'exit' sa iesi: ");
        
        let input = read_line()?;
        if input == "exit" {
            println!("Pa pa! 👋");
            break;
        }
        
        if input == "help" || input == "-h" {
            show_help();
            continue;
        }
        
        if input != "go" {
            println!("Scrie 'go' ca sa incepi sau 'exit' sa iesi!");
            continue;
        }
        
        if let Err(e) = hangman() {
            eprintln!("Eroare joc: {}", e);
        }
        
        println!("Joci din nou? (da sau nu) ");
        let again = read_line()?;
        if again == "nu" || again == "exit" {
            println!("Pa pa! 👋");
            break;
        }
    }
    Ok(())
}

fn read_line() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase().to_string())
}

fn show_help() {
    println!("\n=== HANGMAN AJUTOR ===");
    println!("  Trebuie sa ghicesti cuvantul secret litera cu litera.");
    println!("  Ai voie un numar limitat de greseli in functie de lungimea cuvantului.");
    println!("  'go' - incepe jocul");
    println!("  'help' - mesaj de ajutor");
    println!("  'exit' - iese din joc");
}

fn hangman() -> Result<(), Box<dyn Error>> {
    println!("\n=== HANGMAN ===");
    println!("Ce categorie? (sport/animale/fructe) sau 'exit': ");
    
    let categorie = read_line()?;
    if categorie == "exit" {
        println!("Ai iesit din jocul curent!");
        return Ok(());
    }
    
    let cuvinte = read_words(&categorie)?;
    if cuvinte.is_empty() {
        println!("Niciun cuvant in {}!", categorie);
        return Ok(());
    }
    
    let mut rng = rand::thread_rng();
    let index_random = rng.gen_range(0..cuvinte.len());
    let secret = cuvinte[index_random].clone();
    
    let mut afisat = vec!['_'; secret.len()];
    let mut gresite = 0;
    let mut incercate = [' '; 26];
    
    println!("\nAm ales un cuvant cu {} litere!", secret.len());
    
    loop {
        print!("Cuvant: ");
        for &lit in &afisat {
            print!("{} ", lit);
        }
        println!();
        println!("Gresite: {}", gresite);
        print!("Litera?: ");
        io::stdout().flush()?;
        
        let guess = read_line()?;
        if guess == "exit" {
            println!("Ai iesit din jocul curent!");
            return Ok(());
        }
        
        if guess.chars().count() > 1 {
            println!("O singura litera te rog! (sau 'exit')");
            continue;
        }
        
        if guess.is_empty() {
            println!("Trebuie sa introduci o litera macar!");
            continue;
        }
        
        let litera = guess.chars().next().ok_or("String gol")?;
        if !litera.is_alphabetic() || !litera.to_ascii_lowercase().is_ascii_lowercase() {
            println!("Doar litere ENGLEZE a-z! (fara ă,â,î,ș,ț)");
            continue;
        }
        
        let index = (litera.to_ascii_lowercase() as u8 - b'a') as usize;
        if incercate[index] != ' ' {
            println!("Ai incercat deja {}!", litera);
            continue;
        }
        incercate[index] = litera;
        
        let mut gasita = false;
        for (i, c) in secret.chars().enumerate() {
            if c == litera {
                afisat[i] = litera;
                gasita = true;
            }
        }
        
        if gasita {
            println!("Ai ghicit litera {}!", litera);
        } else {
            gresite += 1;
            println!("Nu e {} in cuvant!", litera);
        }
        
        if afisat.iter().all(|&x| x != '_') {
            println!("🎉 BRAVO! Cuvantul era: {}", secret);
            println!("Greseli: {}", gresite);
            salveaza(&categorie, &secret, gresite, true)?;
            return Ok(());
        }
        
        let max_greseli = secret.len() / 2 + 2;
        if gresite >= max_greseli {
            println!("💀 PIERDERE! Cuvantul era: {}", secret);
            println!("Greseli: {}", gresite);
            salveaza(&categorie, &secret, gresite, false)?;
            return Ok(());
        }
    }
}

fn read_words(categorie: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let cale = format!("{}.txt", categorie);
    let fisier = File::open(&cale)?;
    let reader = BufReader::new(fisier);
    
    let cuvinte: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .map(|l| l.trim().to_lowercase().to_string())
        .filter(|cuvant| !cuvant.is_empty())
        .collect();
    
    Ok(cuvinte)
}

fn salveaza(cat: &str, cuvant: &str, gres: usize, castigat: bool) -> Result<(), Box<dyn Error>> {
    let mut fisier = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("scoruri.txt")?;
    
    let rezultat = if castigat { "CASTIG" } else { "PIERDERE" };
    writeln!(fisier, "{} {} {} {}", cat, cuvant, gres, rezultat)?;
    Ok(())
}
