import os
import subprocess
import sys


def run_cmd(cmd):
    print(f"Running: {cmd}")
    subprocess.check_call(cmd, shell=True)


def setup():
    # 1. Create venv
    if not os.path.exists("venv"):
        run_cmd(f"{sys.executable} -m venv venv")

    # 2. Install dependencies
    pip = "venv/bin/pip"
    run_cmd(f"{pip} install django djangorestframework")

    # 3. Create Project
    python = "venv/bin/python"
    if not os.path.exists("bench_project"):
        run_cmd(f"{python} -m django startproject bench_project")

    # 4. Create App
    os.chdir("bench_project")
    if not os.path.exists("bench_app"):
        run_cmd(f"../{python} manage.py startapp bench_app")

    # 5. Configure Settings
    settings_path = "bench_project/settings.py"
    with open(settings_path) as f:
        content = f.read()

    if "rest_framework" not in content:
        content = content.replace(
            "INSTALLED_APPS = [",
            "INSTALLED_APPS = [\n    'rest_framework',\n    'bench_app',",
        )
        with open(settings_path, "w") as f:
            f.write(content)

    # 6. Create Model and Serializer
    models_code = """
from django.db import models

class Book(models.Model):
    title = models.CharField(max_length=255)
    author = models.CharField(max_length=255)
    description = models.TextField()
    published_date = models.DateField(auto_now_add=True)
    price = models.DecimalField(max_digits=10, decimal_places=2)
"""
    with open("bench_app/models.py", "w") as f:
        f.write(models_code)

    serializers_code = """
from rest_framework import serializers
from .models import Book

class BookSerializer(serializers.ModelSerializer):
    class Meta:
        model = Book
        fields = '__all__'
"""
    with open("bench_app/serializers.py", "w") as f:
        f.write(serializers_code)

    # 7. Migrate and Populate
    run_cmd(f"../{python} manage.py makemigrations")
    run_cmd(f"../{python} manage.py migrate")

    populate_script = """
import os
import django
import random
import time

os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'bench_project.settings')
django.setup()

from bench_app.models import Book
from bench_app.serializers import BookSerializer

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

    print("Benchmarking serialization...")
    queryset = Book.objects.all()
    start = time.time()
    serializer = BookSerializer(queryset, many=True)
    data = serializer.data
    end = time.time()

    print(f"Serialized {len(data)} items in {end - start:.4f} seconds")

if __name__ == "__main__":
    run_benchmark()
"""
    with open("benchmark.py", "w") as f:
        f.write(populate_script)

    print(
        "Setup complete. Run 'venv/bin/python bench_project/benchmark.py' to benchmark."
    )


if __name__ == "__main__":
    setup()
