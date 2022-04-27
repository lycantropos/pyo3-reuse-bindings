from pathlib import Path
from typing import (Iterator,
                    TYPE_CHECKING)

from setuptools import setup


def read_file(path_string: str) -> str:
    return Path(path_string).read_text(encoding='utf-8')


if TYPE_CHECKING:
    from setuptools_rust import RustExtension


class LazyRustExtensions(list):
    def __iter__(self) -> Iterator['RustExtension']:
        from setuptools_rust import RustExtension
        yield RustExtension('base')

    def __len__(self) -> int:
        return 1


setup(name='base',
      rust_extensions=LazyRustExtensions(),
      author='Azat Ibrakov',
      author_email='azatibrakov@gmail.com',
      python_requires='>=3.7',
      setup_requires=read_file('requirements-setup.txt'),
      include_package_data=True,
      zip_safe=False)
