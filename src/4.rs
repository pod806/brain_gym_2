use std::io;

fn main() {
    // Kullanıcıdan girdi alma
    let mut metin = String::new();
    let mut kaydirma = String::new();

    println!("Şifrelemek istediğiniz metni girin:");
    io::stdin().read_line(&mut metin).expect("Girdi alınamadı");

    println!("Şifreleme anahtarını (sayı) girin:");
    io::stdin().read_line(&mut kaydirma).expect("Girdi alınamadı");

    let kaydirma: i32 = kaydirma.trim().parse().expect("Geçersiz sayı girişi");

    let sifrelenmis_metin = sezar_sifreleme(&metin, kaydirma);

    println!("Şifrelenmiş metin: {}", sifrelenmis_metin);
}

fn sezar_sifreleme(metin: &str, kaydirma: i32) -> String {
    let kaydirma = kaydirma % 26; // Her 26 harfte bir döngü tekrar eder

    metin.chars()
        .map(|karakter| {
            if karakter.is_ascii_alphabetic() {
                let baslangic = if karakter.is_ascii_lowercase() { 'a' } else { 'A' };
                let kaydirilmis = ((karakter as u8 - baslangic as u8 + kaydirma as u8) % 26 + baslangic as u8) as char;
                kaydirilmis
            } else {
                karakter
            }
        })
        .collect()
}