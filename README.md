# Pass Vault

Pass Vault is a simple password manager application built with Rust. It allows users to securely store and manage their passwords. The application provides functionalities to add, list, and search for password entries.

![WhatsApp Image 2024-08-08 at 15 04 54_4104333d](https://github.com/user-attachments/assets/7cae1918-2cf7-4749-b8f5-bb1408455880)


## Features

- Add new password entries
- List all stored password entries
- Search for a specific password entry by service name

## Installation

To get started with this project, follow these steps:

1. **Install Rust:**

   Follow the [official Rust installation guide](https://www.rust-lang.org/learn/get-started) to install Rust and Cargo (Rust's package manager and build system).

2. **Clone the Repository:**

   ```bash
   git clone https://github.com/YOUR_GITHUB_USERNAME/password-vault.git
   cd password-vault

3. **Build the Project:**

   ```bash
   cargo build
   ```

4. **Run the Application:**

   ```bash
   cargo run
   ```

## Usage

Provide instructions and examples on how to use your project. For example:

1. **Add password**: Enter a service name, username, and password to add a new entry.
2. **List passwords**: View all stored password entries.
3. **Search Entry**: Search for a password entry by service name.
4. **Quit**: Exit the application.

## File Structure

- `src/main.rs`: Main application logic and user interface.
- `src/pentry.rs`: Contains the `ServiceInfo` struct and related functionality for managing password entries.
- `passwords.json`: File used to store password entries in JSON format.

## Contributing

Contributions are welcome! If you'd like to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Create a new Pull Request.

## License

This project is licensed under the [MIT License](LICENSE). See the [LICENSE](LICENSE) file for more details.

## Contact

For any questions or issues, please open an issue on the [GitHub repository](https://github.com/nipudas29/password-vault/issues).

---

Thank you for using Password-Vault!
