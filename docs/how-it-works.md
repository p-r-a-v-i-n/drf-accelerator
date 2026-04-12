# History & Design

To understand why `drf-accelerator` exists, we need to look into how Django REST Framework (DRF) processes data and where the true performance bottlenecks lie in modern API development.

---

## 🐢 The Bottleneck: It's Not JSON

When a developer experiences slow API responses in DRF, the first instinct is usually to blame the JSON renderer. The common advice is to swap out Django's default JSON renderer for something faster, like `orjson`, ujson, or `msgspec`. 

While adopting `orjson` is always a good idea, developers are often disappointed to see that their API times only improve by a marginal amount. Why?

**Because `Dict -> JSON String` is usually only 5-10% of the total request time.**

The real culprit is the **`Model -> Dict`** conversion. 

When you execute `serializer.data` on a `many=True` query, DRF executes a tight Python loop iterating over every single instance in your QuerySet. For every instance, it loops over every single field defined in your serializer, calling `getattr()` and executing Python methods to construct an output dictionary `to_representation`. 

For an endpoint returning 5,000 items with 10 fields each, DRF is executing **50,000+ Python function calls and attribute lookups**. The Python runtime loop and dictionary allocation overhead absolutely crushes CPU throughput.

## 💡 The Solution: Bypassing the Python Loop

If the problem is the Python interpreter's loop overhead, the solution is explicitly moving that loop out of Python.

`drf-accelerator` uses **PyO3** to build a native Rust extension that replaces the standard `ListSerializer`. When you call `serializer.data` using our `FastSerializationMixin`, we hand the raw QuerySet and field configuration over to Rust.

### How It Works Under the Hood

1. **Pre-caching**: When instantiated, the Rust layer inspects your serializer's fields and caches the precise extraction mechanisms (is it a model field? A `SerializerMethodField`? A dotted source?).
2. **The Rust Loop**: Rust iterates through your QuerySet and grabs attributes using the low-level CPython C-API. 
3. **Native Construction**: It constructs the resulting list of Python dictionaries entirely in Rust memory space, bypassing byte-code evaluation.
4. **Targeted Fallbacks**: For standard primitive types (`int`, `str`, `datetime`), Rust extracts the native types extremely fast. For deeply custom, complex fields, the Rust layer elegantly bounces back to the Python field's `to_representation` method, ensuring your custom logic is never bypassed.

### The Result

By eliminating 90% of the interpreted Python instructions during list serialization, memory allocation is vastly streamlined and CPU cache locality improves. The end result is that your Django application can output massive datasets significantly faster, reducing server CPU load and API latency—all without changing a single line of your domain logic.
