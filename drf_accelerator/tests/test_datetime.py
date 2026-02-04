"""Unit tests for DateTime serialization in drf_accelerator."""

import datetime
from datetime import timezone

from drf_accelerator.drf_accelerator import FastSerializer


class SimpleObject:
    """Helper class for creating test objects."""

    def __init__(self, **kwargs):
        for k, v in kwargs.items():
            setattr(self, k, v)


# ============================================================================
# DateTime Serialization Tests
# ============================================================================


def test_datetime_naive():
    """Test naive datetime serialization (no timezone)."""
    obj = SimpleObject(created_at=datetime.datetime(2026, 1, 23, 14, 30, 45))
    serializer = FastSerializer([("created_at", "created_at")])
    result = serializer.serialize([obj])

    assert len(result) == 1
    assert result[0]["created_at"] == "2026-01-23T14:30:45"


def test_datetime_with_microseconds():
    """Test datetime with microseconds."""
    obj = SimpleObject(created_at=datetime.datetime(2026, 1, 23, 14, 30, 45, 123456))
    serializer = FastSerializer([("created_at", "created_at")])
    result = serializer.serialize([obj])

    assert result[0]["created_at"] == "2026-01-23T14:30:45.123456"


def test_datetime_utc():
    """Test UTC datetime serialization."""
    obj = SimpleObject(
        created_at=datetime.datetime(
            2026, 1, 23, 14, 30, 45, tzinfo=datetime.timezone.utc
        )
    )
    serializer = FastSerializer([("created_at", "created_at")])
    result = serializer.serialize([obj])

    assert result[0]["created_at"] == "2026-01-23T14:30:45Z"


def test_datetime_positive_offset():
    """Test datetime with positive timezone offset (e.g., IST +5:30)."""
    ist = timezone(datetime.timedelta(hours=5, minutes=30))
    obj = SimpleObject(created_at=datetime.datetime(2026, 1, 23, 20, 0, 0, tzinfo=ist))
    serializer = FastSerializer([("created_at", "created_at")])
    result = serializer.serialize([obj])

    assert result[0]["created_at"] == "2026-01-23T20:00:00+05:30"


def test_datetime_negative_offset():
    """Test datetime with negative timezone offset (e.g., EST -5:00)."""
    est = timezone(datetime.timedelta(hours=-5))
    obj = SimpleObject(created_at=datetime.datetime(2026, 1, 23, 9, 30, 0, tzinfo=est))
    serializer = FastSerializer([("created_at", "created_at")])
    result = serializer.serialize([obj])

    assert result[0]["created_at"] == "2026-01-23T09:30:00-05:00"


# ============================================================================
# Date Serialization Tests
# ============================================================================


def test_date_only():
    """Test date serialization."""
    obj = SimpleObject(birth_date=datetime.date(1990, 5, 15))
    serializer = FastSerializer([("birth_date", "birth_date")])
    result = serializer.serialize([obj])

    assert result[0]["birth_date"] == "1990-05-15"


# ============================================================================
# Time Serialization Tests
# ============================================================================


def test_time_only():
    """Test time serialization."""
    obj = SimpleObject(start_time=datetime.time(9, 30, 0))
    serializer = FastSerializer([("start_time", "start_time")])
    result = serializer.serialize([obj])

    assert result[0]["start_time"] == "09:30:00"


def test_time_with_microseconds():
    """Test time with microseconds."""
    obj = SimpleObject(start_time=datetime.time(9, 30, 0, 500000))
    serializer = FastSerializer([("start_time", "start_time")])
    result = serializer.serialize([obj])

    assert result[0]["start_time"] == "09:30:00.500000"


# ============================================================================
# Mixed Fields Tests
# ============================================================================


def test_multiple_datetime_fields():
    """Test multiple datetime fields in same object."""
    obj = SimpleObject(
        created_at=datetime.datetime(2026, 1, 1, 0, 0, 0),
        updated_at=datetime.datetime(2026, 12, 31, 23, 59, 59),
        birth_date=datetime.date(1990, 1, 1),
    )
    serializer = FastSerializer(
        [
            ("created_at", "created_at"),
            ("updated_at", "updated_at"),
            ("birth_date", "birth_date"),
        ]
    )
    result = serializer.serialize([obj])

    assert result[0]["created_at"] == "2026-01-01T00:00:00"
    assert result[0]["updated_at"] == "2026-12-31T23:59:59"
    assert result[0]["birth_date"] == "1990-01-01"


def test_none_datetime():
    """Test None datetime value."""
    obj = SimpleObject(created_at=None)
    serializer = FastSerializer([("created_at", "created_at")])
    result = serializer.serialize([obj])

    assert result[0]["created_at"] is None


def test_mixed_primitives_and_datetime():
    """Test primitives mixed with datetime fields."""
    obj = SimpleObject(
        id=1,
        name="Test",
        created_at=datetime.datetime(2026, 1, 23, 12, 0, 0),
        is_active=True,
    )
    serializer = FastSerializer(
        [
            ("id", "id"),
            ("name", "name"),
            ("created_at", "created_at"),
            ("is_active", "is_active"),
        ]
    )
    result = serializer.serialize([obj])

    assert result[0]["id"] == 1
    assert result[0]["name"] == "Test"
    assert result[0]["created_at"] == "2026-01-23T12:00:00"
    assert result[0]["is_active"] is True
