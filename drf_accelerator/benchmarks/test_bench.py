import datetime
import uuid
from decimal import Decimal

from rest_framework import serializers

from drf_accelerator import FastSerializationMixin
from drf_accelerator.drf_accelerator import FastSerializer


class SimpleObject:
    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)


class StandardSerializer(serializers.Serializer):
    id = serializers.IntegerField()
    name = serializers.CharField()
    created_at = serializers.DateTimeField()
    price = serializers.DecimalField(max_digits=10, decimal_places=2)
    uid = serializers.UUIDField()
    is_active = serializers.BooleanField()


def setup_data(count):
    return [
        SimpleObject(
            id=i,
            name=f"Item {i}",
            created_at=datetime.datetime(
                2026, 1, 23, 12, 0, 0, tzinfo=datetime.timezone.utc
            ),
            price=Decimal("19.99"),
            uid=uuid.uuid4(),
            is_active=True,
        )
        for i in range(count)
    ]


def test_bench_fast_serializer(benchmark):
    data = setup_data(1000)
    # The fields configuration: (output_name, source_attr)
    # Note: On master branch, FastSerializer expects Vec<(String, String)>
    fields = [
        ("id", "id", "simple", None),
        ("name", "name", "simple", None),
        ("created_at", "created_at", "simple", None),
        ("price", "price", "simple", None),
        ("uid", "uid", "simple", None),
        ("is_active", "is_active", "simple", None),
    ]
    fast_ser = FastSerializer(fields)
    benchmark(fast_ser.serialize, data)


def test_bench_standard_serializer(benchmark):
    data = setup_data(1000)

    def run_standard():
        ser = StandardSerializer(data, many=True)
        return ser.data

    benchmark(run_standard)


def test_bench_fast_serializer_primitives(benchmark):
    data = [SimpleObject(id=i, name=f"Name {i}") for i in range(1000)]
    fields = [("id", "id", "simple", None), ("name", "name", "simple", None)]
    fast_ser = FastSerializer(fields)
    benchmark(fast_ser.serialize, data)


class PrimitiveSerializer(serializers.Serializer):
    id = serializers.IntegerField()
    name = serializers.CharField()


def test_bench_standard_serializer_primitives(benchmark):
    data = [SimpleObject(id=i, name=f"Name {i}") for i in range(1000)]

    def run_standard():
        ser = PrimitiveSerializer(data, many=True)
        return ser.data

    benchmark(run_standard)


# End-to-end (DRF) benchmarks: compare the real `many=True` path that end users hit.
class E2EChildSerializer(serializers.Serializer):
    name = serializers.CharField()


class E2EStandardSerializer(serializers.Serializer):
    id = serializers.IntegerField()
    name = serializers.CharField()
    created_at = serializers.DateTimeField()
    price = serializers.DecimalField(max_digits=10, decimal_places=2)
    uid = serializers.UUIDField()
    is_active = serializers.BooleanField()
    child = E2EChildSerializer()
    dotted_child_name = serializers.CharField(source="child.name")
    computed = serializers.SerializerMethodField()

    def get_computed(self, obj):
        # Non-trivial but deterministic work.
        return obj.id + 1


class E2EFastSerializer(FastSerializationMixin, E2EStandardSerializer):
    pass


def setup_data_e2e(count):
    return [
        SimpleObject(
            id=i,
            name=f"Item {i}",
            created_at=datetime.datetime(
                2026, 1, 23, 12, 0, 0, tzinfo=datetime.timezone.utc
            ),
            price=Decimal("19.99"),
            uid=uuid.uuid4(),
            is_active=True,
            child=SimpleObject(name=f"Child {i}"),
        )
        for i in range(count)
    ]


def test_bench_e2e_fast_mixin(benchmark):
    data = setup_data_e2e(1000)

    def run_fast():
        ser = E2EFastSerializer(data, many=True)
        return ser.data

    benchmark(run_fast)


def test_bench_e2e_standard(benchmark):
    data = setup_data_e2e(1000)

    def run_standard():
        ser = E2EStandardSerializer(data, many=True)
        return ser.data

    benchmark(run_standard)
