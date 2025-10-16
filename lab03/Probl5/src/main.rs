fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Impartire la zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Rezultat: {}", result),
        Err(e) => println!("Eroare: {}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("Rezultat: {}", result),
        Err(e) => println!("Eroare: {}", e),
    }
}
