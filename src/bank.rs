// Struktur Bank menyimpan saldo setiap akun dalam sebuah Vector (Vec).
// Indeks dari vector ini mewakili nomor akun (dikurangi 1 karena vector menggunakan 0-based indexing).
struct Bank {
    accounts: Vec<i64>,
}

impl Bank {
    // Membuat instance Bank baru dengan saldo awal masing-masing akun.
    // Misal: Bank::new(vec![10, 100, 20, 50, 30]) akan membuat 5 akun:
    // - Akun 1: saldo 10
    // - Akun 2: saldo 100
    // - Akun 3: saldo 20
    // - Akun 4: saldo 50
    // - Akun 5: saldo 30
    fn new(accounts: Vec<i64>) -> Self {
        Bank { accounts }
    }

    // Fungsi untuk menyetor uang ke akun tertentu.
    // Mengembalikan true jika berhasil, atau false jika nomor akun tidak valid.
    fn deposit(&mut self, account: usize, money: i64) -> bool {
        // Validasi apakah nomor akun ada dalam rentang 1 sampai jumlah akun
        if account >= 1 && account <= self.accounts.len() {
            // Karena vector di Rust bertipe 0-indexed, nomor akun dikurangi 1
            self.accounts[account - 1] += money;
            true
        } else {
            false
        }
    }

    // Fungsi untuk menarik uang dari akun tertentu.
    // Mengembalikan true jika berhasil, atau false jika akun tidak valid atau saldo tidak cukup.
    fn withdraw(&mut self, account: usize, money: i64) -> bool {
        // Validasi apakah nomor akun valid
        if account >= 1 && account <= self.accounts.len() {
            // Validasi apakah saldo mencukupi untuk ditarik
            if self.accounts[account - 1] >= money {
                self.accounts[account - 1] -= money;
                true
            } else {
                false // Saldo tidak cukup
            }
        } else {
            false // Akun tidak ditemukan
        }
    }

    // Fungsi untuk mentransfer uang dari satu akun ke akun lainnya.
    // Mengembalikan true jika transfer berhasil, atau false jika ada akun tidak valid,
    // saldo pengirim tidak cukup, atau nomor akun pengirim dan penerima sama.
    fn transfer(&mut self, account1: usize, account2: usize, money: i64) -> bool {
        // Validasi nomor akun pengirim dan penerima harus valid dan tidak boleh sama
        if account1 >= 1
            && account1 <= self.accounts.len()
            && account2 >= 1
            && account2 <= self.accounts.len()
            && account1 != account2
        {
            // Validasi apakah saldo pengirim mencukupi
            if self.accounts[account1 - 1] >= money {
                self.accounts[account1 - 1] -= money; // Kurangi saldo pengirim
                self.accounts[account2 - 1] += money; // Tambah saldo penerima
                true
            } else {
                false // Saldo pengirim tidak cukup
            }
        } else {
            false // Akun tidak valid atau sama
        }
    }
}

pub fn banking() {
    // Inisialisasi Bank dengan 5 akun dengan saldo masing-masing [10, 100, 20, 50, 30]
    // Akun 1 = 10, Akun 2 = 100, Akun 3 = 20, Akun 4 = 50, Akun 5 = 30
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);

    // 1. Withdraw (Tarik) dari akun 3 sebanyak 10.
    // Akun 3 awalnya memiliki 20. Berhasil ditarik 10, sisa saldo menjadi 10.
    // Output diharapkan: true
    println!("bank.withdraw(3, 10)    -> {}", bank.withdraw(3, 10));

    // 2. Transfer dari akun 5 ke akun 1 sebanyak 20.
    // Akun 5 awalnya 30 menjadi 10. Akun 1 awalnya 10 menjadi 30.
    // Output diharapkan: true
    println!("bank.transfer(5, 1, 20) -> {}", bank.transfer(5, 1, 20));

    // 3. Deposit (Setor) ke akun 5 sebanyak 20.
    // Akun 5 yang memiliki 10 bertambah menjadi 30.
    // Output diharapkan: true
    println!("bank.deposit(5, 20)     -> {}", bank.deposit(5, 20));

    // 4. Transfer dari akun 3 ke akun 4 sebanyak 15.
    // Saldo akun 3 saat ini hanya 10 (dari penarikan pertama), tidak cukup untuk mentransfer 15.
    // Output diharapkan: false
    println!("bank.transfer(3, 4, 15) -> {}", bank.transfer(3, 4, 15));

    // 5. Withdraw (Tarik) dari akun 10 sebanyak 50.
    // Akun 10 tidak terdaftar karena bank hanya diinisialisasi dengan 5 akun.
    // Output diharapkan: false
    println!("bank.withdraw(10, 50)   -> {}", bank.withdraw(10, 50));
}
