
//PROBLEMA 1

// fn prim(x:i32)->bool{
//     if x < 2{
//         return false;
//     }

//     let mut i =2;

//     while i*i<=x{
//         if x%i==0{
//             return false;
//         }
//         i+=1;
//     }

//     return true;
    

// }


// fn main() {
//     for i in 0..=100
//     {
//         if prim(i){
//             print!("{} ",i);
//         }
//     }
// }



//PROBLEMA 2



// fn coprim(mut a:i32,mut b:i32)->bool{

//     while b!=0{
//         let r = a%b;
//         a=b;
//         b=r;
//     }
//     return a==1;
// }


// fn main()
// {
//     for i in 0..=100{
//         for j in 0..=100{
//             if coprim(i,j) == true
//             {
//                 print!("({}, {})",i,j);
//             }
//         }
//     }
// }



//PROBLEMA 3


// fn main()
// {
//     for i in (1..=99).rev()
//     {
//         if i !=1{
//         print!("{} bottles of beer on the wall,
//     {} bottles of beer.
//                 Take one down, pass it around,
//     {} bottles of beer on the wall.",i,i,i-1);
//         }
//         else {
//            print!("{} bottles of beer on the wall,
//     {} bottles of beer.
//                 Take one down, pass it around,
//     No bottles of beer on the wall.",i,i); 
//         }

//     }
// }