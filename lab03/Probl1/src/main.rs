fn is_prime(n: u16)->bool{
    if n<2{
        return false;
    }

    for i in 2..=((n as f64).sqrt() as u16){
        if n.is_multiple_of(i){
            return false;
        }
    }

    true
}

fn next_prime(x: u16)->Option<u16>{
    let mut nr= x.checked_add(1)?;
    while nr<u16::MAX{
        if is_prime(nr){
            return Some(nr);
        }
        nr+=1;
    }
    None
}

fn main() {

    
     let mut n = 2;
    while let Some(p) = next_prime(n) {
        println!("{p}");
        n = p;
        if n > 50 { 
            break;
        }
    }
}
