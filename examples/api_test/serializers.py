from rest_framework import serializers
from .models import Product
from drf_accelerator import FastSerializationMixin

class ProductSerializer(serializers.ModelSerializer):
    class Meta:
        model = Product
        fields = ['id', 'name', 'description', 'price', 'stock', 'is_active', 'created_at']

class FastProductSerializer(FastSerializationMixin, serializers.ModelSerializer):
    class Meta:
        model = Product
        fields = ['id', 'name', 'description', 'price', 'stock', 'is_active', 'created_at']
