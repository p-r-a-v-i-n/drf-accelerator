import os
import django

# Setup Django environment
os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'api_test.settings')
django.setup()

from api_test.models import Product, Category
from api_test.serializers import FastProductSerializer

def run():
    # Setup database records
    Category.objects.get_or_create(id=1, name="Electronics")
    cat = Category.objects.get(id=1)
    
    Product.objects.get_or_create(
        id=101, 
        name="Smartphone", 
        description="A cool phone", 
        price="599.99", 
        stock=50, 
        category=cat
    )
    
    # Query objects
    products = Product.objects.all()
    
    # Serialize using FastProductSerializer
    serializer = FastProductSerializer(products, many=True)
    
    print("=== Serialization Output (Complex Types Supported) ===")
    import json
    print(json.dumps(serializer.data, indent=2))

if __name__ == "__main__":
    run()
