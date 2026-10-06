fn main() {
    //let num = 5;
    //num = 45;
    /* при cargo check будет ошибка
        чтобы такого не было нужно использовать ключевое слово mut    
    */
    let mut num = 5;
    println!("The value of num is: {}", num);
    num = 45;
    println!("The value of num is: {}", num);
    let num = 'a';
    println!("The value of num is: {}", num);
}
