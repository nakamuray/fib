from setuptools import setup
from setuptools_rust import RustExtension


setup(name='fib',
      version='0.1',
      rust_extensions=[
          RustExtension('fib', 'extensions/Cargo.toml')],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)
