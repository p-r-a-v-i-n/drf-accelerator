import logging

from rest_framework.serializers import ListSerializer

from .drf_accelerator import FastSerializer

logger = logging.getLogger(__name__)


class FastListSerializer(ListSerializer):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._fast_field_config = self._build_field_config()

    def _build_field_config(self):
        child = self.child
        config = []
        for field_name, field in child.fields.items():
            source = field.source or field_name

            if "." in source:
                raise NotImplementedError(
                    f"FastSerializer does not support dotted sources: '{source}'. "
                    f"Field: '{field_name}'"
                )

            from rest_framework.serializers import BaseSerializer, SerializerMethodField

            if isinstance(field, BaseSerializer):
                raise NotImplementedError(
                    f"FastSerializer does not support nested serializers: "
                    f"'{field_name}'"
                )
            if isinstance(field, SerializerMethodField):
                raise NotImplementedError(
                    f"FastSerializer does not support SerializerMethodField: "
                    f"'{field_name}'"
                )

            config.append((field_name, source))
        return config

    def to_representation(self, data):
        serializer = FastSerializer(self._fast_field_config)
        return serializer.serialize(data)


class FastSerializationMixin:
    @classmethod
    def many_init(cls, *args, **kwargs):
        kwargs["child"] = cls(*args, **kwargs)
        return FastListSerializer(*args, **kwargs)
