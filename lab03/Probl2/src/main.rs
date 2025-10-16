

fn checked_add(a:u32, b:u32) ->Option<u32>{
    let sum= (a as u64) + (b as u64);
    if sum <= u32::MAX as u64
    {
        Some(sum as u32)
    }
    else
    {
        None
    }
}

fn checked_mul(a:u32, b:u32) ->Option<u32>{
    let product= (a as u64) * (b as u64);
    if product <= u32::MAX as u64
    {
        Some(product as u32)
    }
    else
    {
        None
    }
}

fn main() {
  
   match checked_add(u32::MAX, 1) {
        Some(result) => println!("100 + 200 = {}", result),
        None => println!("Overflow detectat"),
    }

    match checked_add(100, 200) {
        Some(result) => println!("100 + 200 = {}", result),
        None => println!("Overflow detectat"),
    }

    match checked_mul(u32::MAX, 2) {
        Some(result) => println!("100 * 200 = {}", result),
        None => println!("Overflow detectat"),
    }

    match checked_mul(100, 200) {
        Some(result) => println!("100 * 200 = {}", result),
        None => println!("Overflow detectat"),
    }

}
