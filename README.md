# AkiDocs

Markdown to PDF converter. This project is in very early development.

## Install
```powershell
# Move to right directory from repository root
cd ./akidocs-core
# Install Akidocs globally
uv tool install .
```

## Usage
```powershell
# Usage after installed globally
aki input.md output.pdf
```

## Development

```powershell
# Move to right directory from repository root
cd ./akidocs-core
# Sync dependencies from lockfile
uv sync
# Install package in editable mode
uv pip install -e .
# Run tests
uv run python -m pytest
# Output test PDF
uv run python -m akidocs_core test.md output.pdf 
```

**Testing build**
```powershell
cd ./akidocs-core
uv tool install .
cd ..
aki test.md output.pdf
```
