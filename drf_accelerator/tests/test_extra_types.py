"""Unit tests for UUID and Decimal serialization."""

import uuid
from decimal import Decimal

from drf_accelerator.drf_accelerator import FastSerializer


class SimpleObject:
    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)


# ============================================================================
# UUID Tests
# ============================================================================


def test_uuid_serialization():
    """Test standard UUID serialization."""
    uid = uuid.uuid4()
    obj = SimpleObject(uid=uid)
    serializer = FastSerializer([("uid", "uid")])
    result = serializer.serialize([obj])
    assert result[0]["uid"] == str(uid)


def test_uuid_none():
    """Test None value for UUID field."""
    obj = SimpleObject(uid=None)
    serializer = FastSerializer([("uid", "uid")])
    result = serializer.serialize([obj])
    assert result[0]["uid"] is None


def test_custom_uuid_subclass():
    """Test subclass of UUID."""

    class CustomUUID(uuid.UUID):
        pass

    test_val = 123456789012345678901234567890
    uid = CustomUUID(int=test_val)
    obj = SimpleObject(uid=uid)
    serializer = FastSerializer([("uid", "uid")])
    result = serializer.serialize([obj])
    assert result[0]["uid"] == str(uid)


# ============================================================================
# Decimal Tests
# ============================================================================


def test_decimal_serialization():
    """Test standard Decimal serialization."""
    val = Decimal("10.55")
    obj = SimpleObject(price=val)
    serializer = FastSerializer([("price", "price")])
    result = serializer.serialize([obj])
    assert result[0]["price"] == "10.55"


def test_decimal_high_precision():
    """Test high precision Decimal."""
    val = Decimal("12345.67890123456789012345")
    obj = SimpleObject(val=val)
    serializer = FastSerializer([("val", "val")])
    result = serializer.serialize([obj])
    assert result[0]["val"] == str(val)


def test_decimal_scientific():
    """Test scientific notation Decimal."""
    val = Decimal("1E+2")
    obj = SimpleObject(val=val)
    serializer = FastSerializer([("val", "val")])
    result = serializer.serialize([obj])
    # str(Decimal("1E+2")) is "1E+2", usually JSON prefers standard float
    # string but for Decimal consistency in DRF, strings are often
    # preferred to avoid float precision loss. Our implementation uses
    # str(), so valid.
    assert result[0]["val"] == "1E+2"


def test_decimal_subclass():
    """Test subclass of Decimal."""

    class Money(Decimal):
        pass

    val = Money("99.99")
    obj = SimpleObject(val=val)
    serializer = FastSerializer([("val", "val")])
    result = serializer.serialize([obj])
    assert result[0]["val"] == "99.99"


# ============================================================================
# Mixed & Edge Cases
# ============================================================================


def test_mixed_types_complex():
    """Test mixing primitives, uuid, and decimal."""
    uid = uuid.uuid4()
    price = Decimal("42.00")

    obj = SimpleObject(id=1, uid=uid, price=price, is_active=True)

    serializer = FastSerializer(
        [
            ("id_col", "id"),
            ("user_id", "uid"),
            ("cost", "price"),
            ("active", "is_active"),
        ]
    )

    result = serializer.serialize([obj])[0]

    assert result["id_col"] == 1
    assert result["user_id"] == str(uid)
    assert result["cost"] == "42.00"
    assert result["active"] is True
