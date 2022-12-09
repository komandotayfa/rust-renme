use std::io;
fn main() {
    println!("1. sayıyı giriniz: ");
    let mut sayi1 = String::new();
    io::stdin().read_line(&mut sayi1).expect("Hata");
    println!("2. sayıyı giriniz: ");
    let sayi1: f64 = sayi1.trim().parse().expect("invalid input");
    let mut sayi2 = String::new();
    io::stdin().read_line(&mut sayi2).expect("Hata");
    let sayi2: f64 = sayi2.trim().parse().expect("invalid input");
    println!("Toplama için 1, çıkarma için 2, çarpma için 3, bölme için 4 ");
    let mut islem = String::new();
    io::stdin().read_line(&mut islem).expect("Hata");
    let islem: i32 = islem.trim().parse().expect("invalid input");
    if islem == 1{
        let sonuc = sayi1 + sayi2;
        println!("{}",sonuc);
    } else if islem == 2{
        let sonuc = sayi1 - sayi2;
        println!("{}",sonuc);
    } else if islem == 3{
        let sonuc = sayi1 * sayi2;
        println!("{}",sonuc);
    } else if islem == 4{
        let sonuc = sayi1 / sayi2;
        println!("{}",sonuc);
    } else {
        println!("Geçersiz işlem girildi");
    }
}
