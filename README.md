# ✨ Aplikasi Daftar Tugas

Aplikasi Daftar Tugas yang modern, cepat, dan cantik dibangun dengan **Rust** dan framework **Dioxus**. Aplikasi ini mendukung platform **Desktop** dan **Web** dengan penyimpanan data yang konsisten dan mulus.

## 🚀 Fitur

- **💾 Penyimpanan Otomatis (Persistence)**: Tugas Anda disimpan secara otomatis!
  - **Desktop**: Disimpan ke file `todos.json` di direktori utama (root) proyek.
  - **Web**: Disimpan ke `LocalStorage` pada peramban (browser) Anda.
- **🔍 Penyaringan Tugas (Filtering)**: Beralih dengan mudah antara tugas **Semua (All)**, **Aktif (Active)**, dan **Selesai (Completed)**.
- **✏️ Edit Langsung (Inline Editing)**: Klik ganda (double-click) pada baris tugas mana saja untuk mengedit teksnya secara langsung.
- **🎨 Antarmuka (UI) Modern**: Desain premium bertema gelap dengan animasi yang halus dan efek *glassmorphism*.
- **📱 Lintas Platform**: Berjalan sebagai aplikasi desktop native atau di peramban web favorit Anda.

## 🛠️ Teknologi yang Digunakan

- **Bahasa**: [Rust](https://www.rust-lang.org/)
- **Framework**: [Dioxus](https://dioxuslabs.com/)
- **Serialisasi**: [Serde](https://serde.rs/) & `serde_json`
- **Gaya (Styling)**: CSS Vanilla dengan standar modern terbaik.

## 🏃 Cara Menjalankan Aplikasi

### Prasyarat

Pastikan Anda telah menginstal Rust. Jika belum, Anda bisa mengunduhnya di [rustup.rs](https://rustup.rs/).

### 🖥️ Versi Desktop

Cukup jalankan perintah berikut di terminal:
```bash
cargo run
```

### 🌐 Versi Web (Browser)

1. Instal Dioxus CLI terlebih dahulu:
   ```bash
   cargo install dioxus-cli
   ```
2. Jalankan aplikasi web (serve):
   ```bash
   dx serve
   ```
3. Buka peramban (browser) Anda dan akses alamat `http://localhost:8080`.

## 📂 Struktur Proyek

- `src/main.rs`: Logika inti aplikasi dan manajemen *state* (status).
- `style.css`: Gaya premium kustom dan animasi UI.
- `Cargo.toml`: Daftar dependensi dan konfigurasi proyek Rust.

---
Dibuat dengan ❤️ menggunakan [Dioxus](https://dioxuslabs.com/)
