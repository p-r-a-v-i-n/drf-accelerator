from django.db import models

import uuid

class Category(models.Model):
    name = models.CharField(max_length=100)
    
    class Meta:
        app_label = 'api_test'

    def __str__(self):
        return self.name

class Product(models.Model):
    uuid = models.UUIDField(default=uuid.uuid4, editable=False)
    name = models.CharField(max_length=255)
    description = models.TextField()
    price = models.DecimalField(max_digits=10, decimal_places=2)
    stock = models.IntegerField()
    category = models.ForeignKey(Category, on_delete=models.SET_NULL, null=True, related_name='products')
    created_at = models.DateTimeField(auto_now_add=True)
    is_active = models.BooleanField(default=True)

    class Meta:
        app_label = 'api_test'

    def __str__(self):
        return self.name
