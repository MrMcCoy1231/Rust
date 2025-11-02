#[allow(unused)]
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use serde_derive::Deserialize;

#[derive(Clone)]
struct Student {
    nume: String,
    telefon: String,
    varsta: u32,
}




fn problema1(nume_fisier: &str) -> Option<(Student, Student)> {
    let fisier = File::open(nume_fisier).ok()?;
    let randuri = io::BufReader::new(fisier).lines();
    let mut studenti = vec![];

    for rand in randuri {
        let linie = rand.ok()?;
        let parti: Vec<&str> = linie.split(',').collect();
        if parti.len() != 3 {
            continue;
        }
        let varsta = parti[2].trim().parse::<u32>().ok()?;
        studenti.push(Student {
            nume: parti[0].trim().to_string(),
            telefon: parti[1].trim().to_string(),
            varsta,
        });
    }
    if studenti.is_empty() {
        return None;
    }
    let mut cel_mai_mic = &studenti[0];
    let mut cel_mai_mare = &studenti[0];
    for student in &studenti {
        if student.varsta < cel_mai_mic.varsta {
            cel_mai_mic = student;
        }
        if student.varsta > cel_mai_mare.varsta {
            cel_mai_mare = student;
        }
    }
    Some((cel_mai_mic.clone(), cel_mai_mare.clone()))
}

use std::fmt;

struct Panza {
    matrice: [[char; 100]; 55],
}

impl Panza {
    fn nou() -> Self {
        Panza { matrice: [[' '; 100]; 55] }
    }

    fn seteaza_pixel(&mut self, x: usize, y: usize, valoare: char) {
        if y < 55 && x < 100 {
            self.matrice[y][x] = valoare;
        }
    }

    fn set_pixels(&mut self, pixeli: &[(usize, usize, u8)]) {
        for &(x, y, val) in pixeli {
            if let Some(caracter) = std::char::from_u32(val as u32) {
                self.seteaza_pixel(x, y, caracter);
            }
        }
    }
}

impl fmt::Display for Panza {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for linie in &self.matrice {
            let text: String = linie.iter().collect();
            writeln!(f, "{}", text)?;
        }
        Ok(())
    }
}

fn new_canvas() -> Panza {
    Panza::nou()
}


#[derive(Deserialize, Clone)]
struct StudentJson {
    nume: String,
    telefon: String,
    varsta: u32,
}

fn problema3(nume_fisier: &str) -> Option<(StudentJson, StudentJson)> {
    let fisier = File::open(nume_fisier).ok()?;
    let randuri = io::BufReader::new(fisier).lines();
    let mut studenti = vec![];

    for rand in randuri {
        let linie = rand.ok()?;
        let student: StudentJson = serde_json::from_str(&linie).ok()?;
        studenti.push(student);
    }
    if studenti.is_empty() {
        return None;
    }
    let mut cel_mai_mic = &studenti[0];
    let mut cel_mai_mare = &studenti[0];
    for student in &studenti {
        if student.varsta < cel_mai_mic.varsta {
            cel_mai_mic = student;
        }
        if student.varsta > cel_mai_mare.varsta {
            cel_mai_mare = student;
        }
    }
    Some((cel_mai_mic.clone(), cel_mai_mare.clone()))
}

fn main() {
    if let Some((cel_mic, cel_mare)) = problema1("studenti.txt") {
        println!("Cel mai tanar: {} {} {}", cel_mic.nume, cel_mic.telefon, cel_mic.varsta);
        println!("Cel mai in varsta: {} {} {}", cel_mare.nume, cel_mare.telefon, cel_mare.varsta);
    }

     let mut canvas = new_canvas();
    let c = &mut canvas;

    Panza :: set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    Panza :: set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    Panza :: set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    Panza :: set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    Panza :: set_pixels(c, &[(2, 16, 95), (5, 35, 92), (6, 3, 95), (2, 11, 95), (5, 3, 95)]);
    Panza :: set_pixels(c, &[(2, 38, 95), (4, 9, 40), (3, 41, 124), (2, 37, 95), (2, 25, 124)]);
    Panza :: set_pixels(c, &[(5, 27, 124), (2, 27, 124), (4, 0, 124), (3, 35, 47), (2, 18, 95)]);
    Panza :: set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    Panza :: set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    Panza :: set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    Panza :: set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    Panza :: set_pixels(c, &[(6, 30, 95), (5, 45, 95), (4, 31, 124), (4, 7, 124), (3, 43, 39)]);
    Panza :: set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    Panza :: set_pixels(c, &[(3, 44, 95), (3, 19, 92), (5, 23, 95), (3, 8, 47), (2, 10, 95)]);
    Panza :: set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    Panza :: set_pixels(c, &[(3, 10, 95), (4, 44, 95), (2, 9, 95), (0, 32, 95), (5, 2, 95)]);
    Panza :: set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    Panza :: set_pixels(c, &[(3, 46, 92), (5, 25, 44), (1, 43, 124), (5, 46, 47), (3, 15, 47)]);
    Panza :: set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    Panza :: set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    Panza :: set_pixels(c, &[(5, 44, 95), (2, 2, 95), (5, 10, 95), (5, 9, 95), (4, 43, 124)]);
    Panza :: set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    Panza :: set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    Panza :: set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    Panza :: set_pixels(c, &[(4, 40, 124), (3, 3, 95), (4, 4, 124), (6, 31, 47), (3, 4, 96)]);
    Panza :: set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    Panza :: set_pixels(c, &[(3, 12, 92), (2, 32, 95), (5, 37, 95), (5, 26, 95), (5, 39, 47)]);
    Panza :: set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    Panza :: set_pixels(c, &[(5, 36, 95), (7, 30, 95), (6, 4, 47), (4, 24, 95), (1, 32, 95)]);
    Panza :: set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    Panza :: set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    Panza :: set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    Panza :: set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    Panza :: set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    Panza :: set_pixels(c, &[(5, 43, 46)]);

    println!("{}", canvas);



    if let Some((cel_mic, cel_mare)) = problema3("studenti_json.txt") {
        println!("Cel mai tanar: {} {} {}", cel_mic.nume, cel_mic.telefon, cel_mic.varsta);
        println!("Cel mai in varsta: {} {} {}", cel_mare.nume, cel_mare.telefon, cel_mare.varsta);
    }
}
