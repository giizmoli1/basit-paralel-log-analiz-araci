use std::fs;
use std::thread;

#[derive(Debug, Default)]
struct LogSonucu {
    info: usize,
    warn: usize,
    error: usize,
    toplam: usize,
}

fn loglari_analiz_et(satirlar: Vec<String>) -> LogSonucu {
    let mut sonuc = LogSonucu::default();

    for satir in satirlar {
        sonuc.toplam += 1;

        if satir.contains("INFO") {
            sonuc.info += 1;
        } else if satir.contains("WARN") {
            sonuc.warn += 1;
        } else if satir.contains("ERROR") {
            sonuc.error += 1;
        }
    }

    sonuc
}

fn main() {
    let dosya_icerigi =
        fs::read_to_string("logs.txt").expect("Dosya okunamadı");

    let satirlar: Vec<String> =
        dosya_icerigi.lines().map(|x| x.to_string()).collect();

    let orta = satirlar.len() / 2;

    let ilk = satirlar[..orta].to_vec();
    let ikinci = satirlar[orta..].to_vec();

    let t1 = thread::spawn(move || loglari_analiz_et(ilk));
    let t2 = thread::spawn(move || loglari_analiz_et(ikinci));

    let s1 = t1.join().unwrap();
    let s2 = t2.join().unwrap();

    println!("Toplam: {}", s1.toplam + s2.toplam);
    println!("INFO: {}", s1.info + s2.info);
    println!("WARN: {}", s1.warn + s2.warn);
    println!("ERROR: {}", s1.error + s2.error);
}
    println!("----------------------");

if (s1.error + s2.error) > 0 {
    println!("Dikkat: Sistem içerisinde hata kayitlari bulundu.");
} else {
    println!("Hata kaydi bulunmadi.");
}

    println!("Log analizi basariyla tamamlandi.");
