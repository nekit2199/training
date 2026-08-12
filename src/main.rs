use std::io;

fn main() {
    let mut a = String::new;
    println!("Введите число");
    io::stdin()
        .read_line(&mut a)
        .expect("Введите число, а не текст");
}
