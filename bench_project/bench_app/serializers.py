from rest_framework import serializers

from drf_accelerator import FastSerializationMixin

from .models import Book


class BookSerializer(FastSerializationMixin, serializers.ModelSerializer):
    class Meta:
        model = Book
        fields = ["id", "title", "author", "description"]
