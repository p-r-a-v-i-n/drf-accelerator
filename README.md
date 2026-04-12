# DRF Accelerator

🚀 **Turbocharge your Django REST Framework (DRF) list serialization by replacing Python's hot loop with a high-performance Rust extension.**

[![PyPI version](https://badge.fury.io/py/drf-accelerator.svg)](https://badge.fury.io/py/drf-accelerator)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

DRF Accelerator addresses one of the most common performance bottlenecks in Django applications: **returning large lists of data**. By offloading the innermost object-to-dict conversion loop to a Rust module (via PyO3), it bypasses the heavy Python interpreter overhead, resulting in massive throughput gains for your API endpoints. 

---

## 💡 Why DRF Accelerator?

If you've ever profiled a DRF application returning standard `many=True` responses, you know that creating hundreds or thousands of dictionaries in pure Python is exceptionally slow. 

**The Solution:** \
We swap DRF's standard `ListSerializer` with a custom-built Rust implementation (`FastListSerializer`). 

**The Guarantees:**
* **Zero API breakage**: You still get exactly the same standard DRF `serializer.data` structures.
* **Zero logic changes**: You don't need to rewrite your serializers into `.values()` queries or change how you query your database.
* **Minimal adoption effort**: Add exactly **one mixin** to your existing serializer classes. 

---

## ⚡ Performance

The largest speedups occur on list responses (`many=True`) containing **100s to 10,000s of items**, particularly when the fields are primitive types (`str`, `int`, `float`, `bool`, `None`). 

Because benchmarks are highly workload-dependent, this repository includes tooling for you to test the gains directly against your own hardware:

* **Micro-benchmarks**: Tests the pure Rust loop vs pure Python loop.
* **End-to-End benchmarks**: Tests an actual DRF request cycle.

To run the benchmarks locally (requires cloning the repository):
```bash
cd drf_accelerator
pytest benchmarks/ -v --benchmark-only
```

---

## 📦 Installation
Requirements:
* Python 3.10 – 3.13
* Django REST Framework >= 3.12

Install directly from PyPI:
```bash
pip install drf-accelerator
```
> **Note**: This package ships with pre-built binary wheels for major platforms (Linux, macOS, Windows). If your specific architecture isn't covered, `pip` will automatically attempt to compile it from source (which requires a standard Rust toolchain).

---

## 🚦 Quickstart & Usage

DRF Accelerator is designed to be completely unobtrusive. To accelerate a serializer, simply inherit from `FastSerializationMixin` as your first base class.

```python
from rest_framework import serializers
from drf_accelerator import FastSerializationMixin

class MyModelSerializer(FastSerializationMixin, serializers.ModelSerializer):
    class Meta:
        model = MyModel
        fields = ["id", "title", "author", "is_published"]
```

When you use this serializer for list endpoints (`many=True`), the Rust accelerator automatically takes over!

```python
# The FastListSerializer written in Rust will process this entire queryset
serializer = MyModelSerializer(queryset, many=True)
data = serializer.data 
```

---

## 🔍 How It Works & Supported Features

Under the hood, `FastSerializationMixin` overrides the `many_init` method to return our Rust-backed `FastListSerializer`. 

### Fast-Path Supported Types
The Rust extension has deeply optimized zero-overhead "fast paths" for standard conversions:
* `str`, `int`, `float`, `bool`, `None`
* `datetime`, `date`, `time`
* `uuid.UUID`, `decimal.Decimal`

### Universal Fallback
For any deeply custom fields (or object types the Rust layer doesn't explicitly know how to fast-path), the serializer gracefully falls back to calling the standard DRF field’s `to_representation(value)` method. **It will never break your data structure.**

### Supported Field Configurations
* Standard fields (`CharField`, `IntegerField`, etc.)
* Dotted sources (`source="author.user.email"`)
* `SerializerMethodField`
* Nested Serializers

---

## ⚠️ Scope & Limitations

* **Read-Only Focus**: This accelerator strictly improves **output serialization**. It does not speed up incoming data parsing, validation, or saving (`.is_valid()` or `.save()`).
* **List Serialization Only**: It accelerates iterables (`many=True`). Single-object serialization (`many=False`) remains unchanged because the overhead of calling the Rust extension outweighs the benefits for a single dictionary.
* **Fallbacks cause slowdowns**: Relying heavily on manual `MethodFields` or custom data types that require the Python fallback will naturally reduce your overall speedup multiplier, though it will still be faster than native DRF.
