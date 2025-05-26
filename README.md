# rcloc

A lightning-fast clone of [cloc](https://github.com/AlDanial/cloc) (Count Lines of Code) written in Rust with parallel processing for maximum performance.

## Features

- **⚡ Blazing Fast**: Leverages Rust's performance and parallel processing with Rayon
- **🔧 Easy to Use**: Simple command-line interface similar to the original cloc
- **📊 Detailed Statistics**: Counts files, blank lines, comment lines, and lines of code
- **🌍 Multi-Language Support**: Supports 15+ programming languages out of the box
- **🎯 Smart Filtering**: Automatically skips build directories, hidden files, and cache folders
- **📦 Cross-Platform**: Available for Windows, macOS, Linux, and even WebAssembly
- **🚫 Zero Dependencies**: Single binary with no external runtime dependencies

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/your-username/rcloc/releases):

#### Linux
```bash
# x86_64
curl -L https://github.com/your-username/rcloc/releases/latest/download/rcloc-linux-x86_64 -o rcloc
chmod +x rcloc

# ARM64
curl -L https://github.com/your-username/rcloc/releases/latest/download/rcloc-linux-arm64 -o rcloc
chmod +x rcloc

# Static builds (no libc dependency)
curl -L https://github.com/your-username/rcloc/releases/latest/download/rcloc-linux-x86_64-static -o rcloc
chmod +x rcloc
```

#### macOS
```bash
# Intel Macs
curl -L https://github.com/your-username/rcloc/releases/latest/download/rcloc-macos-x86_64 -o rcloc
chmod +x rcloc

# Apple Silicon (M1/M2)
curl -L https://github.com/your-username/rcloc/releases/latest/download/rcloc-macos-arm64 -o rcloc
chmod +x rcloc
```

#### Windows
Download `rcloc-windows-x86_64.exe` from the releases page.

### Build from Source

#### Prerequisites
- [Rust](https://rustup.rs/) (1.70 or later)

#### Build
```bash
git clone https://github.com/your-username/rcloc.git
cd rcloc
cargo build --release
```

The binary will be available at `target/release/rcloc` (or `rcloc.exe` on Windows).

### Install via Cargo
```bash
cargo install --git https://github.com/your-username/rcloc.git
```

## Usage

### Basic Usage
```bash
# Analyze current directory
rcloc

# Analyze specific directory
rcloc /path/to/project

# Analyze specific file
rcloc src/main.rs
```

### Example Output
```
Language             Files      Blank    Comment       Code
----------------------------------------------------------------------
Rust                     1         45         12        234
JavaScript               3         23          8        156
TypeScript               2         15          5         98
Python                   1          8          3         67
----------------------------------------------------------------------
SUM                      7         91         28        555

Analysis completed in 0.05 seconds
```

### Command Line Options
```bash
rcloc [OPTIONS] [PATH]

Arguments:
  [PATH]  Directory or file to analyze [default: .]

Options:
      --exclude-dirs <DIRS>  Exclude additional directories (comma-separated)
  -h, --help                 Print help
  -V, --version              Print version
```

## Supported Languages

rcloc currently supports the following programming languages:

| Language | Extensions |
|----------|------------|
| **Rust** | `.rs` |
| **C/C++** | `.c`, `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp` |
| **Python** | `.py`, `.pyw` |
| **JavaScript** | `.js`, `.jsx`, `.mjs` |
| **TypeScript** | `.ts`, `.tsx` |
| **Java** | `.java` |
| **C#** | `.cs` |
| **Go** | `.go` |
| **Shell** | `.sh`, `.bash`, `.zsh` |
| **PowerShell** | `.ps1`, `.psm1`, `.psd1` |
| **HTML/XML** | `.html`, `.htm`, `.xml` |
| **CSS** | `.css` |
| **SQL** | `.sql` |
| **Ruby** | `.rb` |
| **PHP** | `.php` |
| **YAML** | `.yaml`, `.yml` |
| **JSON** | `.json` |
| **Markdown** | `.md`, `.markdown` |

## Performance

rcloc is designed for speed and can analyze large codebases quickly:

- **Parallel Processing**: Uses all available CPU cores via Rayon
- **Smart Filtering**: Skips irrelevant directories and files upfront
- **Optimized I/O**: Efficient file reading with buffered I/O
- **Low Memory Usage**: Streams file content without loading entire files into memory

## Automatic Directory Exclusion

rcloc automatically skips common build and cache directories:

- `target/` (Rust)
- `node_modules/` (Node.js)
- `.git/`, `.svn/`, `.hg/` (Version control)
- `build/`, `dist/`, `out/` (Build outputs)
- `__pycache__/`, `.pytest_cache/` (Python)
- `.vs/`, `.vscode/`, `.idea/` (IDEs)
- All hidden files and directories (starting with `.`)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
```bash
git clone https://github.com/your-username/rcloc.git
cd rcloc
cargo build
cargo test
```

### Adding New Languages

To add support for a new programming language:

1. Add the language configuration in the `add_languages()` method in `src/main.rs`
2. Specify the file extensions, line comment syntax, and block comment syntax
3. Test with sample files in that language
4. Update this README

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by the original [cloc](https://github.com/AlDanial/cloc) tool by Al Danial
- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [Rayon](https://github.com/rayon-rs/rayon) for parallel processing
- Uses [WalkDir](https://github.com/BurntSushi/walkdir) for efficient directory traversal

## Changelog

**Star this repo if you find it useful! ⭐**
