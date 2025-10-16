#[derive(Debug)]
enum CharError {
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}


fn to_uppercase(c: char) -> Result<char, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_ascii_uppercase())
}


fn to_lowercase(c: char) -> Result<char, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_alphabetic() {
        return Err(CharError::NotLetter);
    }
    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<(), CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_graphic() && !c.is_ascii_whitespace() {
        return Err(CharError::NotPrintable);
    }
    println!("Caracter: {}", c);
    Ok(())
}

fn char_to_number(c: char) -> Result<u8, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(CharError::NotDigit);
    }
    Ok(c as u8 - b'0')
}

fn char_to_number_hex(c: char) -> Result<u8, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_hexdigit() {
        return Err(CharError::NotBase16);
    }
    Ok(c.to_digit(16).unwrap() as u8)
}

fn print_error(e: CharError) {
    match e {
        CharError::NotAscii => println!("Eroare: caracterul nu este ASCII."),
        CharError::NotDigit => println!("Eroare: caracterul nu este o cifra zecimala."),
        CharError::NotBase16 => println!("Eroare: caracterul nu este cifra hexazecimala."),
        CharError::NotLetter => println!("Eroare: caracterul nu este litera."),
        CharError::NotPrintable => println!("Eroare: caracterul nu este printabil."),
    }
}


fn main() {
    match to_uppercase('a') {
        Ok(c) => println!("Uppercase: {}", c),
        Err(e) => print_error(e),
    }
    match to_uppercase('1') {
        Ok(c) => println!("Uppercase: {}", c),
        Err(e) => print_error(e),
    }
    match to_lowercase('Z') {
        Ok(c) => println!("Lowercase: {}", c),
        Err(e) => print_error(e),
    }
    if let Err(e) = print_char('!') {
        print_error(e);
    }
    if let Err(e) = print_char('\x07') {
        print_error(e);
    }
    match char_to_number('5') {
        Ok(n) => println!("Number: {}", n),
        Err(e) => print_error(e),
    }
    match char_to_number('b') {
        Ok(n) => println!("Number: {}", n),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('C') {
        Ok(n) => println!("Hex number: {}", n),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('G') {
        Ok(n) => println!("Hex number: {}", n),
        Err(e) => print_error(e),
    }
}

