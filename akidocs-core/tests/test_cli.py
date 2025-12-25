import subprocess
from pathlib import Path


def test_cli_produces_pdf(tmp_path):
    input_file = tmp_path / "test.md"
    output_file = tmp_path / "test.pdf"

    input_file.write_text("# Hello\n\nWorld")

    result = subprocess.run(
        [
            "uv",
            "run",
            "python",
            "-m",
            "akidocs_core",
            str(input_file),
            str(output_file),
        ],
        capture_output=True,
    )

    assert result.returncode == 0
    assert output_file.exists()
    assert output_file.stat().st_size > 0
