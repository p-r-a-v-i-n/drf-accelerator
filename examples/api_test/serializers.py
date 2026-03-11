from rest_framework import serializers
from .models import Product, Category
from drf_accelerator import FastSerializationMixin

class CategorySerializer(serializers.ModelSerializer):
    class Meta:
        model = Category
        fields = ['id', 'name']

class ProductSerializer(serializers.ModelSerializer):
    stock_status = serializers.SerializerMethodField()
    category = CategorySerializer(read_only=True)
    category_name = serializers.CharField(source='category.name', read_only=True, default=None)

    class Meta:
        model = Product
        fields = ['id', 'uuid', 'name', 'description', 'price', 'stock', 'stock_status', 'category', 'category_name', 'is_active', 'created_at']

    def get_stock_status(self, obj):
        return "In Stock" if obj.stock > 0 else "Out of Stock"

class FastProductSerializer(FastSerializationMixin, ProductSerializer):
    pass
