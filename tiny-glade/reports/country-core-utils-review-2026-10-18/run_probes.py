"""Run bounded review probes against current sources, cleaning the temporary Cargo target."""
from pathlib import Path
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
TARGET = "/tmp/tg-country-core-utils-review-20261018-target"
temporary = ROOT / "crates/country-core/tests/country_utils_review_20261018.rs"
temporary.parent.mkdir(exist_ok=True)
source = (HERE / "probes.rs").read_bytes()
with temporary.open("xb") as output:
    output.write(source)
try:
    command = ["cargo", "test", "--offline", "--locked", "--target-dir", TARGET,
               "-p", "country-core", "--test", temporary.stem, "--", "--test-threads=1"]
    with (HERE / "probes.log").open("w") as output:
        output.write("$ " + " ".join(command) + "\n")
        output.flush()
        result = subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT)
    raise SystemExit(result.returncode)
finally:
    if temporary.read_bytes() == source:
        temporary.unlink()
    else:
        raise RuntimeError(f"temporary probe changed externally; preserving {temporary}")
