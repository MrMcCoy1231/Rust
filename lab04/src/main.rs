use std :: {io,fs};
use std::collections::HashMap;

fn ex1()->Result<(),io::Error>
{
    let s= fs::read_to_string( "C:/Users/Tudor Alexie/Desktop/Rust/lab04/fisier_de_citit.txt")?;

    let mut maxi_bytes=("",0);
    let mut maxi_chars = ("",0);

    for linie in s.lines(){

        if maxi_bytes.1<linie.len(){
            maxi_bytes = (linie,linie.len());
        }

        if maxi_chars.1 < linie.chars().count(){
            maxi_chars= (linie,linie.chars().count());
        }
    }


    println!("Exercitiul 1\n\n");

    println!("Linia cu cei mai mulți bytes: {}\nLungime: {}", maxi_bytes.0, maxi_bytes.1);
    println!("Linia cu cei mai mulți caractere: {}\nLungime: {}", maxi_chars.0, maxi_chars.1);

    println!("\n\n");
    Ok(())
}

fn ex2(nume_fisier: &str){
    
    match fs::read_to_string(nume_fisier){
        Err(e)=>{
            println!("Eroare la citire: {}",e);
        },
        Ok(text)=>{
            let mut rezultat = String::new();
            for c in text.chars(){
                if !c.is_ascii(){
                    println!("Eroare: caracter NON-ASCII");
                    return;
                }
                let cod=c as u8;
                let nou = match c{
                    'a'..='z'=>(((cod-b'a'+13)%26)+b'a')as char,
                    'A'..='Z'=>(((cod-b'A'+13)%26)+b'A')as char,
                    _=>c,

                };
                rezultat.push(nou);
            }
            println!("Exercitiul 2\n");
            println!("{}",rezultat);
            println!("\n\n");
        }
    }
}

fn ex3(nume_fisier: &str) {
    let abrevieri: HashMap<&str, &str> = [
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna")
    ].iter().cloned().collect();

    match fs::read_to_string(nume_fisier) {
        Err(e) => {
            println!("Eroare la citire: {}", e);
        },
        Ok(continut) => {
            let mut fraze_extinse = Vec::new();
            for cuv in continut.split_whitespace() {
                match abrevieri.get(cuv) {
                    Some(&explicatie) => fraze_extinse.push(explicatie.to_string()),
                    None => fraze_extinse.push(cuv.to_string()),
                }
            }
            println!("Exercitiul 3\n");
            println!("{}", fraze_extinse.join(" "));
            println!("\n\n");
        }
    }
}

fn ex4(path: &str) {
    match fs::read_to_string(path) {
        Err(e) => {
            println!("Eroare la citire: {}", e);
            
        },
        Ok(content) => {
            println!("Exercitiul 4\n\n");
            for linie in content.lines() {
                let linie = linie.trim();
                
                if linie.is_empty() || linie.starts_with('#') {
                    continue;
                }
                let cols: Vec<&str> = linie.split_whitespace().collect();
                if cols.len() >= 2 {
                    
                    println!("{} => {}", cols[1], cols[0]);
                }
            }
        }
    }
}


fn main() {
let _=ex1();
ex2("input_ex2.txt");
ex3("input_ex3.txt");
ex4("C:/Users/Tudor Alexie/Desktop/Rust/lab04/hosts.txt");
}
