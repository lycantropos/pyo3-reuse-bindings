# Preparation (optional)

Create virtual environment with `Python3.7+`
```bash
python -m venv venv
```
... and activate it
```bash
. venv/bin/activate
```

# Installation

Install `base` library
```bash
python -m pip install -e base
```
Install `derived` library
```bash
python -m pip install -e derived
```

# Test
Open `python` CLI
```bash
python
```
after that
```python
>>> from base import MyClass  # class is defined in one module
>>> from derived import use_my_class  # function is defined in another module
>>> use_my_class(MyClass())  # want to be able to pass instances of `base.MyClass` class to a `derived.use_my_class` function, but instead getting error
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
TypeError: argument 'value': 'MyClass' object cannot be converted to 'MyClass'
```
