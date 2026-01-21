from rest_framework import viewsets
from .models import Product
from .serializers import ProductSerializer, FastProductSerializer

class ProductViewSet(viewsets.ModelViewSet):
    queryset = Product.objects.all()
    serializer_class = ProductSerializer

class FastProductViewSet(viewsets.ModelViewSet):
    queryset = Product.objects.all()
    serializer_class = FastProductSerializer
