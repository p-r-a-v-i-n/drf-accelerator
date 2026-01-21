import os
import django
import time
import json

# Setup Django
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'examples.settings')
django.setup()

from api_test.models import Product
from api_test.serializers import ProductSerializer, FastProductSerializer

def seed_data(count=1000):
    print(f"Seeding {count} products...")
    Product.objects.all().delete()
    products = [
        Product(
            name=f"Product {i}",
            description=f"Description for product {i} " * 5,
            price=19.99,
            stock=i,
            is_active=True
        )
        for i in range(count)
    ]
    Product.objects.bulk_create(products)
    print("Seeding complete.")

def benchmark():
    queryset = Product.objects.all()
    count = queryset.count()
    
    # Standard Serializer
    start_time = time.time()
    serializer = ProductSerializer(queryset, many=True)
    _ = serializer.data
    standard_time = time.time() - start_time
    print(f"Standard Serializer ({count} items): {standard_time:.4f}s")

    # Fast Serializer
    start_time = time.time()
    fast_serializer = FastProductSerializer(queryset, many=True)
    _ = fast_serializer.data
    fast_time = time.time() - start_time
    print(f"Fast Serializer ({count} items): {fast_time:.4f}s")

    if fast_time > 0:
        improvement = (standard_time / fast_time)
        print(f"Performance Improvement: {improvement:.2f}x")

    standard_data = json.loads(json.dumps(ProductSerializer(queryset, many=True).data))
    fast_data = json.loads(json.dumps(FastProductSerializer(queryset, many=True).data))
    
    if standard_data == fast_data:
        print("Verification SUCCESS: Data is identical.")
    else:
        print("Verification FAILURE: Data differs.")
        for i in range(min(len(standard_data), len(fast_data))):
            if standard_data[i] != fast_data[i]:
                print(f"First diff at index {i}:")
                print(f"Standard: {standard_data[i]}")
                print(f"Fast:     {fast_data[i]}")
                break

if __name__ == "__main__":
    seed_data(10000)
    benchmark()
