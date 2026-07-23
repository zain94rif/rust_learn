use rand::Rng; // Mengimpor trait Rng agar bisa menggunakan fungsi gen_range
use std::io;

pub fn main() {
    // let mut jumlah = 0;

    // loop {
    //     jumlah += 1;
    //     println!("{jumlah}");
    //     break;
    // }

    // salam();
    // sapa("budi");
    println!("=== Tebak Angka 0 - 100 ===");
    // let angka = 46;
    // Membuat generator angka acak lokal
    let mut rng = rand::thread_rng();
    // Membuat angka acak antara 1 sampai 100 (inklusif)
    let angka = rng.gen_range(1..=100);

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let pilih = match input.trim().parse::<u8>() {
            Ok(u) => u,
            Err(_) => {
                println!("Input harus berupa angka.");
                continue;
            }
        };

        if pilih > 100 {
            println!("Maximal 100, coba lagi:")
        } else if pilih > angka {
            println!("Terlalu besar, coba lagi:")
        } else if pilih < angka {
            println!("Terlalu kecil, coba lagi:")
        } else {
            println!("Tebakan benar, Angkanya adalah {angka}");
            break;
        }
    }

    // Setelah input sudah aman menggunakan match, coba tambahkan fitur berikut satu per satu:

    // ✅ Batasi maksimal 5 tebakan.
    // ✅ Tampilkan sisa kesempatan.
    // ✅ Hitung berapa kali pengguna menebak.
    // ✅ Setelah menang, tanyakan apakah ingin bermain lagi.
    // ✅ Gunakan angka acak (crate rand) sebagai pengganti 46.
}

/*
fn salam() {
    println!("Halo");
}

fn sapa(nama: &str) {
    println!("Halo {nama}");
}
*/
