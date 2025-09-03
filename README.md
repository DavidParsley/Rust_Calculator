# 🧮 Rust Calculator

A **fun, stylish Rust calculator with personality**! 🎉  
This project goes beyond simple arithmetic by interacting with the user, remembering their name, and making math feel engaging with a touch of humor and emojis.  

[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Last Updated](https://img.shields.io/badge/last%20updated-04--09--2025-blueviolet)](#)
[![Rust Version](https://img.shields.io/badge/rust-1.89.0-93450a)](https://www.rust-lang.org/)

---

## ✨ Features

- 👋 Welcomes the user with a personalized greeting.  
- 🕵️ Asks for the user’s name and remembers it during calculations.  
- ➕ Addition  
- ➖ Subtraction  
- ✖ Multiplication  
- ➗ Division (with division-by-zero error handling).  
- 🚀 Fun, stylish CLI experience with emojis.  

---

## 🛠️ Technologies Used

- [Rust](https://www.rust-lang.org/)  
- Standard Library: `std::io`  

---

## 📦 Installation Requirements

1. **Install Rust**  
   👉 [Rust Installation Guide](https://www.rust-lang.org/tools/install)

2. **Install VS Code Rust Extension**  
   👉 [Rust Analyzer for VS Code](https://code.visualstudio.com/docs/languages/rust)

3. **Set up Cargo project**  
   ```bash
   # 1. Open terminal
   # 2. Navigate to folder where you want to save the project
   cd path/to/folder

   # 3. Create new Cargo project (use snake_case for folder name)
   cargo new rust_calculator

   # 4. Move into project folder
   cd rust_calculator

   # 5. Build the project
   cargo build

   # 6. Run the project
   cargo run

## 📚 Cargo Documentation
- [Cargo Book](https://doc.rust-lang.org/stable/cargo/index.html)

## 🚀 Usage Example
    ```bash
    $ cargo run
    Hello 👋 from Davids First Rust Calculator!
    I was created on September 3rd 2025 👨‍🍼

    Whats Your Name 🕵️‍♂️?
    > David

    Hello David 🚀! Lets do some Maths 👨‍🏫!
    Please enter the first number (e.g.,1️⃣, 2️⃣, 3️⃣, 4️⃣)
    > 10

    Please Enter an operator (e.g.,➕, ➖, *, /):
    > *

    Please enter the second number (e.g.,5️⃣, 6️⃣, 7️⃣, 8️⃣)
    > 5 

    David The Result is : 50 🥳
    ```


## ⚙️ Configuration Options
- Currently, the calculator runs interactively in the terminal. Future enhancements could include:

- Loop mode (do multiple calculations until "exit").


## 🐛 Known Issues
- Emojis with non-yellow skin tones may display oddly in some terminals.
- 👉 Use the default yellow skin tone for best compatibility.

## 📚 Resources Used
- The Rust Book

- Cargo Book

- Tech With Tim Rust YouTube Playlist

- Get Emoji

## 🗂️ Code Structure

rust_calculator/
├── src/
│   └── main.rs          # Main program source
├── target/              # Auto-generated build artifacts
│   └── debug/
│   └── .rustc_info.json
│   └── CACHEDIR.TAG
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md

## 🛠️ Troubleshooting

- Error: command not found: cargo
  → Ensure Rust and Cargo are properly installed and added to your PATH.

- Program crashes when entering text instead of a number
  → Input parsing requires numeric values (e.g., 10, 3.5).

- Emojis look strange in CLI
  → Some terminals may not support all emojis. Stick to basic ones.

## 🤝 Contributing

- Contributions are welcome!
- To contribute:

- Fork the repository.

- Create a new branch (feature/amazing-feature).

- Commit your changes.

- Push to the branch.

- Open a Pull Request.

## 📜 License
- This project is open source under the [MIT License](https://opensource.org/license/mit).
- You are free to use, modify, and distribute it as you wish.

