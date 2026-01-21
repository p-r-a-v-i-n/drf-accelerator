# Contributing to DRF Accelerator

Thank you for your interest in contributing! This project is an experimental effort to speed up Django Rest Framework using Rust.

## Development Setup

### Prerequisites
- Python 3.8+
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- `maturin` (installed via `pip install maturin`)

### Setting Up the Environment
1. **Clone and create a venv**:
   ```bash
   git clone https://github.com/p-r-a-v-i-n/drf-accelerator.git
   cd drf-accelerator
   python -m venv venv
   source venv/bin/activate  # On Windows use `venv\Scripts\activate`
   ```

2. **Install development dependencies**:
   ```bash
   pip install -e . --group dev
   # Or manually:
   pip install maturin ruff pre-commit djangorestframework
   ```

3. **Install pre-commit hooks**:
   ```bash
   pre-commit install
   ```

## Development Workflow

### Building the Rust Extension
The project uses `maturin` to bridge Rust and Python. To build the extension in development mode (which symlinks the binary into your site-packages):
```bash
maturin develop
```
For performance testing, use:
```bash
maturin develop --release
```

### Running Benchmarks
We use a sample Django project to measure performance:
```bash
python setup_env.py  # Run once to set up the bench project
python bench_project/benchmark.py
```

## Guidelines
- **Code Style**: We use `ruff` for linting and formatting. Ensure your changes pass `pre-commit`.
- **Rust Code**: Follow standard idiomatic Rust patterns. Keep the boundary between Rust and Python thin for maximum performance.
- **Safety**: While we aim for performance, avoid `unsafe` Rust unless absolutely necessary and well-documented.

## Reporting Issues
Please use GitHub Issues to report bugs or suggest features. Since this is experimental, feedback on performance across different data types is highly appreciated!
