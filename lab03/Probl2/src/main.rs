fn checked_add(a: u32, b: u32) -> u32 {
    let sum = (a as u64) + (b as u64);
    if sum <= u32::MAX as u64 {
        sum as u32
    } else {
        panic!("Overflow detectat in adunare!")
    }
}

fn checked_mul(a: u32, b: u32) -> u32 {
    let product = (a as u64) * (b as u64);
    if product <= u32::MAX as u64 {
        product as u32
    } else {
        panic!("Overflow detectat in inmultire!")
    }
}

fn main() {
    let sum = checked_add(100, 200);
    println!("100 + 200 = {}", sum);

    let product = checked_mul(100, 200);
    println!("100 * 200 = {}", product);
}
