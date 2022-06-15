// Exercise 1
// fn main() {
//     let mut output = 20;
//     let x = change_value(10,&mut output);
// }

// fn change_value(input:u32, output: &mut u32) -> u32{
//     if input ==1 {
//         *output =3;
//     }
//     else {
//         *output = 4;
//     }

//     *output
// }



//Exercise 2
// Mục đích: giải quyết vấn đề ownership và borrowing ko dùng clone()
// Các bạn có thể sửa thêm logic để đúng với mục đichs bài này là liệt kê các số nguyên tố
// fn main() {
//     let mut count: u32 = 1;
//     let mut num: u64 = 2;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 1;
//         if vector_is_prime(num, &primes) {
//             primes.push(num);
//             count += 1;
//         }
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: &Vec<u64>) -> bool {
//     for i in 0..p.len() {
//         if num % p[i] == 0 {
//             return false;
//         }
//     }

//     true
// }



//Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;
    
//     //for n in &mut values {
//     for n in 0..v.len() {
//         max = std::cmp::max(max, v[n]);
//     }

//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
//     //for n in &mut values {
//     for n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }



//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
// fn main(){
//     let  a = vec![1,2,3,4,5];
 
//     let (b, c) = test(&a);
//     println!("a = {:?}, b = {:?}, c = {}", a, b, c);
// }

// pub fn test(a: &Vec<u8>) -> (Vec<u8>, i32) {
//     let mut b:Vec<u8>  = Vec::new();
//     let mut c:u8 = 0;
//     for i in 0..a.len() {
//         let index = a.len() - 1 - i;
//         let d = a[index];
//         c = c+d;
//         b.push(d);
//     }
//     (b, c as i32)
// }