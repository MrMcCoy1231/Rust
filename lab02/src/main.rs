
// PROBLEMA 1
fn add_chars_n(mut s : String, c : char, mut nr : i32 ) ->String
{

    while nr != 0{
        s.push(c);
        nr-=1;
    }

    

    s

}


// PROBLEMA 2

fn add_chars_n_referinta(s: &mut String, c : char, mut nr : i32 )
{

    while nr != 0{
        s.push(c);
        nr-=1;
    }

}



//PROBLEMA 3

fn add_space(s: &mut String, mut nr:i32){

    while nr!=0{
        s.push(' ');
        nr-=1;
    }
}

fn add_str(s: &mut String, sir:&str)
{
    s.push_str(sir);
}

fn add_integer(s: &mut String, mut nr:i32){

    if nr == 0{
        s.push('0');
        return;
    }

    if nr < 0 {
        s.push('-');
        nr = -nr;
    }

    let mut buf = ['\0';16];
    let mut len =0;
    let mut count = 0;

    while nr > 0{
        buf[len] = ((nr%10) as u8 + b'0') as char;
        len+=1;
        nr/=10;
        count+=1;
        if count%3==0 && nr!=0{
            buf[len] = '_';
            len+=1;
        }
    }

    for i in (0..len).rev(){
        s.push(buf[i]);
    }

    

}

fn add_float(s: &mut String, x:f64, precizie:u32)
{
    let mut val = x;
    if val<0.0{
        s.push('-');
        val = - val;
    }

    let int_part=val as i32;
    add_integer(s,int_part);
    s.push('.');
    let mut fract_part = val-(int_part as f64);
    for _ in 0..precizie{
        fract_part*=10.0;
        let cifra = fract_part as i32;
        s.push((cifra as u8 + b'0') as char);
        fract_part -= cifra as f64;

    }
}




fn main() {


//Problema 1
println!("Problema 1\n");
let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
    

    //Problema 2

    println!("\n\n\n Problema 2 \n");
let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_referinta(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
    
//Problema 3

println!("\n\n\n Problema 3 \n");

    let mut s = String::new();
    
    add_space(&mut s, 39);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚");
    s.push('\n');
    add_space(&mut s, 39);
    add_str(&mut s, "RUST.");
    s.push('\n');
    s.push('\n');

    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is");
    s.push('\n');

    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038, 3);
    s.push('.');
    s.push('\n');

    print!("{}", s);
}