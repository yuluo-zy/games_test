#!/usr/bin/env python3
"""Copy four current packages into an empty, disposable verification workspace.

Seed dependency resolution with the current shared lock. The first Cargo check
may update ONLY this temporary lock to accommodate workspace membership changes;
subsequent tests/checks should use --locked. Production source/locks are read-only.
"""
import argparse
import shutil
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path, help="new or empty output directory")
    args = parser.parse_args()
    project = Path(__file__).resolve().parent.parent
    output = args.output.resolve()
    if output == project or output.is_relative_to(project / "crates"):
        parser.error("output must be outside the source package directories")
    if output.exists() and (not output.is_dir() or any(output.iterdir())):
        parser.error("output directory must be empty")
    output.mkdir(parents=True, exist_ok=True)
    for name in ("paths", "lang-lib", "tabloid", "tg-iced"):
        shutil.copytree(
            project / "crates" / name,
            output / "crates" / name,
            ignore=shutil.ignore_patterns("target", "__pycache__", ".DS_Store"),
        )
    (output / "Cargo.toml").write_text(
        '[workspace]\nresolver = "2"\n'
        'members = ["crates/paths", "crates/lang-lib", "crates/tabloid", "crates/tg-iced"]\n'
        '[workspace.package]\nedition = "2024"\nrust-version = "1.86"\npublish = false\n'
        '[workspace.dependencies]\nanyhow = "=1.0.86"\n',
        encoding="utf-8",
    )
    shutil.copyfile(project / "Cargo.lock", output / "Cargo.lock")
    print(output)


if __name__ == "__main__":
    main()
