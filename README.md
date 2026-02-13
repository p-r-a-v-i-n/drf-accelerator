# DRF Accelerator

> [!CAUTION]
> **Experimental / Under Development**
> This project is currently an experiment to improve DRF performance using Rust. It is NOT production-ready and has strict scope limitations.

A high-performance Rust-backed accelerator for Django Rest Framework.

## Performance Benchmark (1,000 items)
| Method | Primitives (1k) | Complex (1k) | Speedup |
| :--- | :--- | :--- | :--- |
| **Standard DRF** | 4.63ms | 38.4ms | 1x |
| **drf-accelerator** | **0.26ms** | **3.2ms** | **~12x to ~17x** |

*Benchmark run on 1,000 objects with 6 fields (including DateTime, UUID, and Decimal) using `pytest-benchmark`.*

## Installation & Setup

### For Users (Stable)
Install directly from PyPI:

```bash
pip install drf-accelerator
```

> [!NOTE]
> Since this is a Rust extension, you will need a modern Python environment. Pre-built wheels are provided for common platforms.

### From Source
If you want to install the latest development version:

1. **Prerequisites**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. **Clone & Install**:
   ```bash
   git clone https://github.com/p-r-a-v-i-n/drf-accelerator.git
   cd drf-accelerator
   pip install -e .
   ```

### For Developers (Try it out)
If you want to run the benchmarks yourself:

1. **Build the extension**:
   ```bash
   cd drf_accelerator
   maturin develop --release
   cd ..
   ```

2. **Run Benchmarks**:
   ```bash
   cd drf_accelerator
   pytest benchmarks/ -v --benchmark-only
   ```

## Usage
Simply inherit from `FastSerializationMixin` in your `ModelSerializer`:

```python
from drf_accelerator import FastSerializationMixin
from rest_framework import serializers

class MySerializer(FastSerializationMixin, serializers.ModelSerializer):
    class Meta:
        model = MyModel
        fields = ["id", "title", "author", "is_published"]
```

## Limitations (Strict)
To maintain high performance and safety, the following are **not supported**:
- **Dotted Sources**: `source="user.profile.age"` will error.
- **Nested Serializers**: Cannot be used inside an accelerated serializer.
- **Method Fields**: `SerializerMethodField` is not supported.
- **Complex Types**: Only `int`, `str`, `float`, `bool`, `None`, `datetime`, `date`, `time`, `uuid`, and `decimal` are supported.

## How it works
The Mixin swaps the standard DRF `ListSerializer` for a `FastListSerializer` that offloads the object-to-dict conversion loop to a Rust extension using PyO3. This significantly reduces Python interpreter overhead for large list responses.
