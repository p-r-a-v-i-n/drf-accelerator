from django.conf import settings


def pytest_configure():
    if not settings.configured:
        settings.configure(
            INSTALLED_APPS=[
                "django.contrib.contenttypes",
                "rest_framework",
            ],
            REST_FRAMEWORK={},
        )
        import django

        django.setup()
