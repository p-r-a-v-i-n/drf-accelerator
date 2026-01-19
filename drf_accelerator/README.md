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
1. **Prerequisites**: Ensure you have Rust and `maturin` installed.
2. **Build and Install**:
   ```bash
   cd drf_accelerator
   maturin develop --release
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
