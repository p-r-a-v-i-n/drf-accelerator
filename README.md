# DRF Accelerator

Speed up Django REST Framework list serialization (`many=True`) by moving the hot object-to-dict loop into a Rust extension (PyO3).

## What You Get
- Big wins on large list responses where most fields are primitives (`str/int/float/bool/None`) and simple conversions.
- No changes to your API output: you still get standard DRF `serializer.data` structures.
- Minimal adoption: add one mixin to your serializer class.

## Performance (Run Your Own)
Benchmarks are highly workload- and machine-dependent. This repo includes both:
- A micro-benchmark of the Rust loop: `test_bench_fast_serializer` vs `test_bench_standard_serializer`.
- An end-to-end DRF benchmark (what users actually run): `test_bench_e2e_fast_mixin` vs `test_bench_e2e_standard`.

Run:
```bash
cd drf_accelerator
pytest benchmarks/ -v --benchmark-only
```

## Installation & Setup

### Install
Install from PyPI:

```bash
pip install drf-accelerator
```

> [!NOTE]
> This is a Rust extension. If a pre-built wheel is not available for your platform/Python version, you will need a Rust toolchain to build from source.

## Compatibility
- Python: CPython 3.10–3.13 (`requires-python >=3.10,<3.14`)
- DRF: `djangorestframework>=3.12`

### From Source
If you want to build locally:

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

## Production Notes
- Measure on your actual serializers and payload sizes; speedups vary a lot.
- Biggest wins are typically list endpoints returning 100s–10,000s of items.
- If your deployment platform doesn’t have wheels, add Rust to your build pipeline (or build wheels in CI).

## Scope
- Accelerates serialization of lists (`many=True`). Single-object serializers are unchanged.
- This is about output serialization only (not validation, parsing, or saving).

## Supported Features
- Field sources: normal fields, dotted sources (`source="a.b.c"`), `SerializerMethodField`, and nested serializers.
- Fast paths: `str`, `int`, `float`, `bool`, `None`, `datetime/date/time`, `uuid.UUID`, `decimal.Decimal`.
- Fallback: other values call the DRF field’s cached `to_representation(value)`.

## Current Limitations
- Dotted sources, nested serializers, and `None` values are supported, but may see smaller speedups because they involve more Python work per item.
- `FastSerializer` is primarily an internal implementation detail of the DRF mixin. If you construct it manually and pass `None` as the field object, only the fast-path types work; otherwise it raises `TypeError("unsupported type")` because there is no `to_representation` fallback.

## How it works
The Mixin swaps the standard DRF `ListSerializer` for a `FastListSerializer` that offloads the object-to-dict conversion loop to a Rust extension using PyO3. This significantly reduces Python interpreter overhead for large list responses.
