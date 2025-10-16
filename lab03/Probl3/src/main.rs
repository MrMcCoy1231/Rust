#[derive(Debug)]
enum MathError {
    Overflow,
}

fn checked_add(a:u32,b:u32)->Result<u32,MathError>{
    let sum = (a as u64)+(b as u64);
    if sum<=u32::MAX as u64{
        Ok(sum as u32)
    }else{
        Err(MathError::Overflow)
    }
}

fn checked_mul(a: u32, b: u32) -> Result<u32, MathError> {
    let product = (a as u64) * (b as u64);
    if product <= u32::MAX as u64 {
        Ok(product as u32)
    } else {
        Err(MathError::Overflow)
    }
}

fn compute(a: u32, b: u32) -> Result<u32, MathError> {
    
    let sum = checked_add(a, b)?;
    
    let result = checked_mul(sum, 2)?;
    Ok(result)
}

fn main() {
    
 match compute(10, 20) {
        Ok(val) => println!("Success: Result is {}", val),
        Err(e) => println!("Error: {:?}", e),
    }

    
    match compute(u32::MAX, 1) {
        Ok(val) => println!("Unexpected success: {}", val),
        Err(e) => println!("Expected error: {:?}", e),
    }

    
    match compute(1_000_000_000, 2_000_000_000) {
        Ok(val) => println!("Unexpected success: {}", val),
        Err(e) => println!("Expected error: {:?}", e),
    }

}
