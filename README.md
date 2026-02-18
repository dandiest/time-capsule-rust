<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Stable-brightgreen.svg" /> 
  <img src="https://img.shields.io/badge/Library-Serde-blue.svg" />
  <img src="https://img.shields.io/badge/Library-Chrono-red.svg" />
</p>

<h1 align="center">⌛ Time Capsule Rust - Persistent Memory System</h1>

<p align="center">
  A sophisticated Command Line Interface (CLI) designed to bridge the gap between sessions using persistent JSON storage, high-precision timestamps, and robust error handling.
</p>

---

## 🎓 Educational Disclaimer
This repository is a cornerstone of my **Personal Apprenticeship** in Rust. 
* **Purpose**: This project focuses on mastering the interaction between memory (Structs) and physical storage (File System).
* **Evolution**: Building upon basic logic, Time Capsule introduces "State Awareness"—the ability for a program to recognize its own previous execution state.
* **Focus**: Deep dive into **Serde serialization**, **UTC time management**, and the **Pattern Matching** philosophy of Rust.

## 🌟 Features
* **State Recognition**: Uses `std::fs` to detect existing data, providing a seamless "Welcome Back" experience by retrieving the last stored message.
* **Identity Tracking**: Implements an `author` field within the data structure to ensure every "memory" is attributed.
* **High-Precision Chronology**: Leverages the `Chrono` library to timestamp every entry with UTC precision, ensuring global time consistency.
* **Crash-Proof Logic**: Implements exhaustive `match` patterns to handle `io::ErrorKind::NotFound` without program interruption.

## 🛠️ Technical Deep Dive
* **JSON Serialization**: Converts complex Rust Structs into human-readable JSON using `serde_json` with "pretty-print" formatting.
* **Safe Input/Output**: Utilizes `io::stdout().flush()` to manage the standard output buffer, ensuring a professional and responsive user interface.
* **Ownership & Strings**: Demonstrates clean usage of `.trim().to_string()` to manage memory allocation and data cleaning from CLI inputs.

---

## 🚀 How to Run
1. Clone the repository:
   ```bash
   git clone [https://github.com/dandiest/time-capsule-rust.git](https://github.com/dandiest/time-capsule-rust.git)
   ```
2. Build and run:
    ```bash
    cargo run
   ```

---
   
## ⚖️ License & Copyright

Copyright (c) 2026 Samuel **[dandiest]**

This project is licensed under the MIT License.

*You are free to use, study, and modify this code for educational purposes. Professionalism starts with sharing knowledge.*
