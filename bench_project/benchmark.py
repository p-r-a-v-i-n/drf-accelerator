
import os
import django
import random
import time

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'bench_project.settings')
django.setup()

from bench_app.models import Book
from bench_app.serializers import BookSerializer
from rest_framework import serializers

class BaselineBookSerializer(serializers.ModelSerializer):
    class Meta:
        model = Book
        fields = ["id", "title", "author", "description"]

def run_benchmark():
    count = 10000
    if Book.objects.count() < count:
        print("Populating database...")
        books = [
            Book(
                title=f"Book {i}", 
                author=f"Author {i}", 
                description="Some description " * 10, 
                price=random.uniform(10.0, 100.0)
            )
            for i in range(count)
        ]
        Book.objects.bulk_create(books)
        print("Done.")

    queryset = list(Book.objects.all())  # Force to list once to avoid DB diffs
    
    print(f"Benchmarking {len(queryset)} items...")

    # 1. Baseline DRF
    start = time.time()
    baseline_serializer = BaselineBookSerializer(queryset, many=True)
    baseline_data = baseline_serializer.data
    end = time.time()
    baseline_time = end - start
    print(f"  - Baseline DRF: {baseline_time:.4f}s")

    # 2. Accelerated (Integrated Mixin)
    start = time.time()
    accelerated_serializer = BookSerializer(queryset, many=True)
    accelerated_data = accelerated_serializer.data
    end = time.time()
    accelerated_time = end - start
    print(f"  - Accelerated (Mixin): {accelerated_time:.4f}s")
    
    print(f"\nSpeedup: {baseline_time / accelerated_time:.2f}x")

    # Correctness check
    assert len(baseline_data) == len(accelerated_data), "Length mismatch!"
    assert baseline_data[0] == accelerated_data[0], f"Data mismatch! \nBase: {baseline_data[0]}\nAcc: {accelerated_data[0]}"
    print("Correctness check passed!")

if __name__ == "__main__":
    run_benchmark()
