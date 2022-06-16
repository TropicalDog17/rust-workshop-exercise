// fn main(){

// }

// Exercise 1
// Mục đích: giải quyết vấn đề ownership and borrowing không dùng clone()
// fn main() {
//     let my_var:u32 = 20;   //Khai bao bien u32 mutable
//     let x = change_value(1, &mut my_var); //borrow tham chieu cua bien my_var
//     println!("x ={}", x);
// }

// fn change_value(input:u32, output: &mut u32) -> u32{ //&mut de thay doi gia tri cua output
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
//     let mut num: u64 = 1;
//     let mut primes: Vec<u64> = Vec::new();
//     primes.push(2);

//     while count < 10 {
//         num += 2;
//         if vector_is_prime(num, &primes) { //Ta chi borrow reference cua prime chu khong borrow gia tri, tranh gay loi ownership
//             count += 1;
//             primes.push(num);
//         }
//     }
//     println!("{:?}", primes);
// }

// fn vector_is_prime(num: u64, p: &[u64]) -> bool { //Thay doi kieu du lieu cua p thanh &[u64]
//     for &i in p { //Duyet theo tham chieu cua i, khi do i se la gia tri. Con neu dung "for i in p", trong
//         if num > i && num % i == 0 { //Sua "num %i != 0" thanh num % i ==0<Chia het thi k phai prime number>
//             return false;
//         }
//     }

//     true
// }

// Exercise 3
// Mục đích: giải quyết vấn đề ownership and borrowing ko dùng clone()
// fn main() {
//     let mut values = vec![10, 11, 12];
//     let v = &mut values;

//     let mut max = 0;

//     //for n in &mut values {
//     for n in v {
//         max = std::cmp::max(max, *n);
//     }
//     let v = &mut values; //Reborrowing
//     println!("max is {}", max);
//     println!("Converting to percentages of maximum value...");
//     //for n in &mut values {
//     for  n in v {
//         *n = 100 * (*n) / max;
//     }
//     println!("values: {:#?}", values);
// }

//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
fn main() {
    let mut a = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    let c = 0;
    loop {
        let (a, c) = test(&mut a);
        println!("{:?}{}", a, c);
        i = i + 1;
        if i >= 5 {
            break;
        }
    }
}

pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {
    let mut b: Vec<u8> = Vec::new();
    let mut c: u8 = 0;
    loop {
        if a.len() == 0 {
            break;
        }
        let d = a.pop().unwrap();
        c = c + d;
        b.push(d);
    }
    (b, c as i32)
}
