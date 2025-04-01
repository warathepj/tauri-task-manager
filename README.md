# Task Manager

A modern desktop task management application built with Tauri and Dioxus.

## Features

- Create and manage tasks
- Cross-platform support (Windows, macOS, Linux)
- Native performance with a web-like UI
- Lightweight and fast

## Prerequisites

Before you begin, ensure you have installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (LTS version recommended)
- Platform-specific dependencies for Tauri
- [See Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Development Setup

1. Clone the repository:

```bash
git clone https://github.com/warathepj/tauri-task-manager.git
cd tauri-task-manager
```

2. Install Rust dependencies:

```bash
cargo install dioxus-cli
cargo install tauri-cli
```

3. Run the development server:

```bash
cargo tauri dev
```

## Building for Production

To create a production build:

```bash
cargo tauri build

```

The built application will be available in the `src-tauri/target/release` directory.

## Project Structure

- `src/` - Dioxus frontend code
- `src-tauri/` - Tauri backend code
- `assets/` - Static assets like CSS and images
- `dist/` - Build output directory

## IDE Setup

Recommended IDE: Visual Studio Code with the following extensions:

- Tauri
- rust-analyzer
- Dioxus

## License

This project is licensed under the MIT License - see the LICENSE file for details.
