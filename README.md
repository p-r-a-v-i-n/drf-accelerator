# DRF Accelerator

> [!CAUTION]
> **Experimental / Under Development**
> This project is currently an experiment to improve DRF performance using Rust. It is NOT production-ready and has strict scope limitations.

A high-performance Rust-backed accelerator for Django Rest Framework.

## Performance Benchmark
| Method | Time (10k items) | Speedup |
| :--- | :--- | :--- |
| **Standard DRF** | 0.1002s | 1x |
| **drf-accelerator** | **0.0351s** | **~2.86x** |

*Benchmark run on 10,000 simple models with 4 primitive fields.*

## Installation & Setup

### For Users (Stable)
Currently, the package is in early development. To install it from source:

1. **Prerequisites**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. **Clone the repository**:
   ```bash
   git clone https://github.com/p-r-a-v-i-n/drf-accelerator.git
   cd drf-accelerator
   ```
3. **Build and Install**:
   ```bash
   pip install -e .
   ```

### For Developers (Try it out)
If you want to run the benchmarks and see the speedup yourself:

1. **Run the setup script**:
   ```bash
   python setup_env.py
   ```
   This will create a virtual environment, a sample Django project, and populate the database.

2. **Run the benchmark**:
   ```bash
   venv/bin/python bench_project/benchmark.py
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
- **Non-Primitives**: Only `int`, `str`, `float`, `bool`, and `None` are supported. Non-primitive types (like `Decimal` or `Date`) will currently trigger a `TypeError`.

## How it works
The Mixin swaps the standard DRF `ListSerializer` for a `FastListSerializer` that offloads the object-to-dict conversion loop to a Rust extension using PyO3. This significantly reduces Python interpreter overhead for large list responses.
