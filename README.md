# basit-paralel-log-analiz-araci
# Basit Paralel Log Analiz Aracı

## Proje Hakkında

Bu proje Rust programlama dili kullanılarak geliştirilmiş basit bir paralel log analiz aracıdır.

Program, log dosyalarını okuyarak INFO, WARN ve ERROR kayıtlarını analiz eder. Analiz işlemi iki ayrı thread üzerinde paralel olarak gerçekleştirilir ve sonuçlar birleştirilerek kullanıcıya sunulur.

## Özellikler

* Log dosyası okuma
* INFO kayıtlarını sayma
* WARN kayıtlarını sayma
* ERROR kayıtlarını sayma
* Paralel analiz (2 thread)
* Sonuçları terminalde gösterme
* Rust ownership ve thread yapılarının kullanılması

## Kullanılan Teknolojiler

* Rust
* Cargo
* GitHub

## Çalıştırma

```bash
cargo run
```

## Örnek Çıktı

```text
Toplam: 7
INFO: 3
WARN: 2
ERROR: 2
```

## Geliştirilebilecek Özellikler

* Grafiksel kullanıcı arayüzü (GUI)
* Dinamik thread sayısı
* CSV veya JSON çıktısı
* Gerçek zamanlı log izleme

## Geliştirici

Gizem Taşar, Gamze Bayözü
Yönetim Bilişim Sistemleri

