fn main() {
    //int: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    let num: u8 = 5;
    println!("The value of num(u) is: {}", num);
    let num: i32 = -45;
    println!("The value of num(i) is: {}", num);
    let num: u128 = 123456789012345678901234567890;
    println!("The value of num(i128) is: {}", num);

    //float: f32, f64
    let num: f32 = 5.343;
    println!("The value of num(f32) is: {}", num);
    let num: f64 = 7.45678;
    println!("The value of num(f64) is: {}", num);

    //bool: true, false
    let is_true: bool = true;
    println!("The value of is_true is: {}", is_true);
    let is_false: bool = false;
    println!("The value of is_false is: {}", is_false);

    //char: 'a', '1', '!', etc.
    let sym: char = 'A';
    println!("The value of sym is: {}", sym);
}
