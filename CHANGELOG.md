# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-alpha.1] - 2026-01-23

### Added
- Initial release of the Rust-backed DRF accelerator.
- `FastSerializationMixin` for high-performance list serialization.
- Support for primitive types: `int`, `float`, `bool`, `str`, and `None`.

### Performance
- Local benchmarks show 10x-15x speedup for large list serializations (e.g., 10,000 items) compared to standard DRF serializers.

### Fixed
- Identified and documented Browsable API rendering overhead for large datasets.
