use std::io;
fn main() {
    let n = 0;
    while n==0{let mut input = String::new();
         println!("Sayı giriniz, çıkmak için '0' girebilirsiniz");
         io::stdin().read_line(&mut input).expect("Hata!");
         let input: f64 = input.trim().parse().expect("invalid input");
         let sonuc = input % 2.0;
         let dort = input % 4.0;
         if input == 0.0{
             println!("Çıkılıyor");
             break;}
         if dort == 0.0{
            println!("Çift ve 4'e bölünüyor");
         }else if sonuc == 0.0{
            println!("Çift sayı");
         }else{
            println!("Tek sayı");
         }}
}
