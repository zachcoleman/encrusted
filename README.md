![GitHub Workflow Status (branch)](https://img.shields.io/github/actions/workflow/status/zachcoleman/encrusted/tests.yml?branch=main)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/encrusted)
![PyPI - Wheel](https://img.shields.io/pypi/wheel/encrusted)
[![License](https://img.shields.io/badge/license-Apache2.0-green)](./LICENSE)

# encrusted 
The project was developed using the [maturin](https://maturin.rs) framework.

## Installation
From PyPI:
```shell
pip install encrusted
```

Build from source:
```
maturin build -r -i=path/to/python
pip install .../encrusted/target/wheels/<whl file name>.whl
```

## Usage
```python
import numpy as np
import encrusted

arr = np.random.randint(0, 1, size=(100, 100))
encoded_arr = encrusted.encode(arr)
assert np.array_equal(encrusted.decode(encoded_arr), arr)
```


## Running Tests
Tests are run with `pytest`.
