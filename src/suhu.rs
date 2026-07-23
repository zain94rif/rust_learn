// pub fn suhu() {
//     let suhu = 30;
//     println!("{}", suhu);
// }

// pub fn suhu() {
//     let suhu = "Celcius";
//     println!("{}", suhu);
// }

// fn tambah(a: i32, b: i32) -> i32 {
//     a + b
// }

// pub fn suhu() {
//     println!("{}", tambah(10, 5));
// }

use std::io;
/*
pub fn suhu() {
    println!("=== Konversi Celsius ke Fahrenheit ===");
    println!("Masukkan suhu dalam Celsius:");
    let mut input = String::new(); // dan diisi dengan string kosong baru (String::new()) untuk menampung ketikan pengguna

    io::stdin() // Memanggil modul standard input/output (io) untuk membaca ketikan dari keyboard
        .read_line(&mut input) // Mengambil teks yang diketik pengguna dan memasukkannya ke dalam variabel 'input'
        .expect("Gagal membaca input"); // Jika proses membaca input gagal/error, program akan berhenti dan memunculkan pesan ini

    let celsius: f64 = input.trim().parse().expect("Masukkan angka");
    // Mengubah teks (String) dari variabel 'input' menjadi angka pecahan (f64)
    // .trim() digunakan untuk membuang spasi atau tombol Enter (\n) di ujung teks
    // .parse() digunakan untuk mengubah tipe data teks menjadi tipe data angka
    // .expect() akan menghentikan program jika pengguna memasukkan teks bukan angka (misal: "abc")
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    // Menghitung rumus konversi Celsius ke Fahrenheit
    // Hasil perhitungan disimpan ke dalam variabel baru bernama 'fahrenheit'
    println!("{celsius}°C = {fahrenheit}°F");
}
*/
/*
pub fn suhu() {
    println!("=== Konversi Fahrenheit ke Celsius ===");
    println!("Masukkan suhu dalam Fahrenheit:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    let fahrenheit: f64 = input.trim().parse().expect("Masukkan angka");
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F = {celsius}°C");
}
*/

pub fn suhu() {
    println!("=== Konversi Suhu ===");
    println!("Pilih 1. Celsius → Fahrenheit");
    println!("      2. Fahrenheit → Celsius");
    let mut pilih = String::new();
    io::stdin()
        .read_line(&mut pilih)
        .expect("Gagal membaca input");
    let pilihan: u32 = pilih.trim().parse().expect("Input harus berupa angka!");
    match pilihan {
        1 => println!("Masukkan suhu dalam Celsius:"),
        2 => println!("Masukkan suhu dalam Fahrenheit:"),
        _ => panic!("Pilihan salah! Hanya boleh angka 1 atau 2."), // Berhenti jika angka lain
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    if pilihan == 1 {
        let celsius: f64 = input.trim().parse().expect("Masukkan angka");
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        println!("{celsius}°C = {fahrenheit}°F");
    } else {
        let fahrenheit: f64 = input.trim().parse().expect("Masukkan angka");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("{fahrenheit}°F = {celsius}°C");
    }
}
