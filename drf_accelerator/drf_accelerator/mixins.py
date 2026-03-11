import logging

from rest_framework.serializers import ListSerializer

from .drf_accelerator import FastSerializer

logger = logging.getLogger(__name__)


class FastListSerializer(ListSerializer):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._fast_field_config = self._build_field_config()
        # Build the Rust serializer once per DRF serializer instance.
        self._fast_serializer = FastSerializer(self._fast_field_config)

    def _build_field_config(self):
        child = self.child
        config = []
        from rest_framework.serializers import BaseSerializer, SerializerMethodField
        
        for field_name, field in child.fields.items():
            source = field.source or field_name

            if isinstance(field, SerializerMethodField):
                method = getattr(child, field.method_name)
                config.append((field_name, source, "method", method))
            elif isinstance(field, BaseSerializer):
                config.append((field_name, source, "nested", field))
            elif "." in source:
                config.append((field_name, source, "dotted", field))
            else:
                config.append((field_name, source, "simple", field))
                
        return config

    def to_representation(self, data):
        return self._fast_serializer.serialize(data)


class FastSerializationMixin:
    @classmethod
    def many_init(cls, *args, **kwargs):
        kwargs["child"] = cls(*args, **kwargs)
        return FastListSerializer(*args, **kwargs)
