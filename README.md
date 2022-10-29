![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/zachcoleman/encrusted/tests/main)
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
pip install .../fast-stats/target/wheels/<whl file name>.whl
```

## Running Tests
Tests are run with `pytest`.