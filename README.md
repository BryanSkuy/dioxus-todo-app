# ✨ Dioxus Todo App

A modern, fast, and beautiful Todo List application built with **Rust** and the **Dioxus** framework. This app supports both **Desktop** and **Web** platforms with seamless data persistence.

## 🚀 Features

- **💾 Persistence**: Your tasks are saved automatically!
  - **Desktop**: Saved to `todos.json` in the project root.
  - **Web**: Saved to the browser's `LocalStorage`.
- **🔍 Filtering**: Easily switch between **All**, **Active**, and **Completed** tasks.
- **✏️ Inline Editing**: Double-click any task to edit its text instantly.
- **🎨 Modern UI**: A premium dark-themed design with smooth animations and glassmorphism effects.
- **📱 Cross-Platform**: Runs as a native desktop app or in your favorite web browser.

## 🛠️ Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Framework**: [Dioxus](https://dioxuslabs.com/)
- **Serialization**: [Serde](https://serde.rs/) & `serde_json`
- **Styling**: Vanilla CSS with modern best practices.

## 🏃 How to Run

### Prerequisites

Ensure you have Rust installed. If not, get it at [rustup.rs](https://rustup.rs/).

### 🖥️ Desktop Version

Simply run:
```bash
cargo run
```

### 🌐 Web Version (Browser)

1. Install the Dioxus CLI:
   ```bash
   cargo install dioxus-cli
   ```
2. Serve the app:
   ```bash
   dx serve
   ```
3. Open your browser at `http://localhost:8080`.

## 📂 Project Structure

- `src/main.rs`: Core application logic and state management.
- `style.css`: Custom premium styles and animations.
- `Cargo.toml`: Project dependencies and configuration.

--- 
Built with ❤️ using [Dioxus](https://dioxuslabs.com/)
