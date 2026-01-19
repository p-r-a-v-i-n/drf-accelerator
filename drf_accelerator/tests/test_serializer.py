import pytest

from drf_accelerator.drf_accelerator import FastSerializer


class SimpleObject:
    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)


def test_fast_serializer_basic():
    fields = [("id", "id"), ("name", "name")]
    serializer = FastSerializer(fields)

    obj = SimpleObject(id=1, name="Test")
    data = serializer.serialize([obj])

    assert len(data) == 1
    assert data[0]["id"] == 1
    assert data[0]["name"] == "Test"


def test_fast_serializer_primitives():
    fields = [("s", "s"), ("i", "i"), ("f", "f"), ("b", "b"), ("n", "n")]
    serializer = FastSerializer(fields)

    obj = SimpleObject(s="string", i=42, f=3.14, b=True, n=None)
    data = serializer.serialize([obj])

    assert data[0]["s"] == "string"
    assert data[0]["i"] == 42
    assert data[0]["f"] == 3.14
    assert data[0]["b"] is True
    assert data[0]["n"] is None


def test_fast_serializer_unsupported_type():
    from datetime import date

    fields = [("d", "d")]
    serializer = FastSerializer(fields)

    obj = SimpleObject(d=date(2023, 1, 1))
    with pytest.raises(TypeError) as excinfo:
        serializer.serialize([obj])

    assert "unsupported type" in str(excinfo.value)


def test_fast_serializer_missing_attribute():
    fields = [("m", "missing")]
    serializer = FastSerializer(fields)

    obj = SimpleObject()
    with pytest.raises(AttributeError):
        serializer.serialize([obj])
