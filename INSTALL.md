# Installation
Before proceeding with the installation, ensure that your operating system is supported.

<details>
<summary>

### Supported Operating Systems and PHP Versions

</summary>


| Operating System | PHP Version | Status             | Notes                                  |
| ---------------- | ----------- | ------------------ | -------------------------------------- |
| Linux (Ubuntu)   | 8.1+        | Tested & Supported | Fully tested on Ubuntu 20.04 and 22.04 |
| Linux (Debian)   | 8.1+        | Tested & Supported | Fully tested on Debian                 |
| Linux (CentOS)   | 8.0+        | Tested             | Partial testing; minor issues reported |
| macOS            | 8.0+        | Tested & Supported | Tested on macOS Monterey and Ventura   |
| Windows 10/11    | 8.0+        | Tested & Supported | Fully functional with IIS and CLI      |
| Windows Server   | n/a         | Not Tested         | Requires manual testing                |
| Linux (Alpine)   | 8.0+        | Tested             | Compatible, tested in Dockerized env   |
| FreeBSD          | n/a         | Not Tested         | Compatibility unknown                  |
| Other UNIX-based | n/a         | Not Tested         | Requires additional testing            |

**Legend**
- **Tested & Supported**: Verified to work without issues.
- **Tested**: Tested but may require additional configuration or fixes.
- **Not Tested**: Compatibility not verified, may work but unsupported.

</details>


## Steps to install  `php-trace`
Follow these steps to install or update the `php-trace` extension.


### 1. Install Rust and Cargo
Rust and Cargo are required to build the extension from source. 

#### On Linux and macOS:
The recommended way to install Rust is via `rustup`, a toolchain installer for Rust.
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation. After installation, you may need to restart your terminal or source your profile:
```sh
source $HOME/.cargo/env
```

#### On Windows:
Download and run the `rustup-init.exe` installer from the [official Rust website](https://www.rust-lang.org/tools/install). Follow the on-screen instructions to complete the installation.

#### Verify installation:
After installation, open a new command prompt or PowerShell window and ensure that `cargo` and `rustc` are in your PATH by running:

```sh
rustc --version
cargo --version
```

### 2. Install Required Libraries
Appart from PHP, additional tools like `php-dev` and `clang` are required to build `php-trace`.

#### On Linux (Ubuntu/Debian-based Distros):
Run the following commands to install required tools:
```sh
sudo apt update
sudo apt install -y php-dev libclang-dev clang
```

#### On macOS:
Development files (`php-dev`) are included in the Homebrew PHP installation, so all we need is clang tools.
```sh
xcode-select --install
```

#### On Windows:
On Windows, development files (`php-dev`) are typically included in the PHP package. Ensure `phpize` is available in the extracted PHP folder.<br>
Download Clang from the [LLVM official website](https://llvm.org/) or use a package manager like Chocolatey:
```sh
choco install llvm
```

### 3. Clone the Repository and Build the Extension
You’ll need to clone the source code and compile the extension.

#### Clone the repository:
```sh
git clone https://github.com/rukanuel/php-trace.git
cd php-trace
```

#### Build the extension:
```sh
cargo build --release
```
This generates the library file in the `target/release` directory.

### 4. Add the Extension to Your PHP
Locate your PHP extensions directory. You can find the directory path by running:
```sh
php -i | grep extension_dir
```
Then, copy the appropriate file for your platform:

#### On Linux:
```sh
cp target/release/libphp_trace.so /usr/lib/php/extensions/
```

#### On macOS:
```sh
cp target/release/libphp_trace.dylib /usr/local/lib/php/extensions/
```

#### On Windows:
```sh
copy target\release\libphp_trace.dll C:\path\to\php\ext\
```

### 5. Update Your `php.ini` Configuration
Add the following line to your `php.ini` file to enable the extension:

#### On Linux and macOS:
```ini
extension=libphp_trace.so
```
#### On Windows:
```ini
extension=libphp_trace.dll
```

#### Locating the `php.ini` File
To find your active `php.ini` file, run:
```sh
php --ini
```

### 6. Restart Your Server or PHP-FPM
For the changes to take effect, restart your web server or PHP-FPM process:
Locate your PHP extensions directory. You can find the directory path by running:

#### Apache:
```sh
sudo systemctl restart apache2
# or
sudo service apache2 restart
```

#### Nginx (with PHP-FPM):
```sh
sudo systemctl restart php-fpm
sudo systemctl restart nginx
```

#### Windows (IIS): 
Restart the IIS service or the machine if needed.

### 7. Verify the Installation
Check if the extension is loaded by running:
```sh
php -m | grep php_trace
```

---

## Additional Notes

- Ensure that the path to the PHP extensions directory is correct. You can find the extension directory by running `php -i | grep extension_dir`.
- If you encounter any issues during the installation of Rust, refer to the [official Rust installation guide](https://www.rust-lang.org/tools/install) for troubleshooting tips.
- Make sure your PHP installation is compatible with the extension. You can check your PHP version by running `php -v`.

By following these steps, you can successfully build, install, and enable the `php-trace` extension on your system.

