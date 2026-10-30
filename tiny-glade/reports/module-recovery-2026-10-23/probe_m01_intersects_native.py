"""重放已保留的原始输入位，不实现几何算法；直接执行原RNE完整函数。"""
from pathlib import Path
import hashlib
import json
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = ROOT.parent / "B版本内容/tiny-glade.rne"
FIXTURE = ROOT / "tests/fixtures/native/m01-rectangle-intersects.json"
SOURCE = Path(__file__).with_name("probe_m01_intersects_x86.c")
REPORT = Path(__file__).with_name("m01-pins-intersects-native.json")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    fixture = json.loads(FIXTURE.read_text())
    assert digest(ORIGINAL) == fixture["original_sha256"]
    cases = fixture["cases"]
    raw = b"".join(struct.pack("<13I", case["id"], *case["bits"]) for case in cases)
    with tempfile.TemporaryDirectory(prefix="m01-intersects-") as directory:
        temporary = Path(directory)
        executable = temporary / "probe-x86"
        inputs, outputs = temporary / "input.bin", temporary / "output.bin"
        inputs.write_bytes(raw)
        subprocess.run(["clang", "-arch", "x86_64", "-O2", str(SOURCE), "-o", str(executable)], check=True)
        run = subprocess.run([str(executable), str(ORIGINAL), str(inputs), str(outputs)], check=True, capture_output=True, text=True)
        actual = outputs.read_bytes()
        assert len(actual) == len(cases) * 8
        mismatches = []
        for case, (index, value) in zip(cases, struct.iter_unpack("<II", actual)):
            assert index == case["id"]
            if value != case["expected"]:
                mismatches.append({"id": index, "expected": case["expected"], "actual": value})
        report = {
            "original": str(ORIGINAL), "original_sha256": digest(ORIGINAL),
            "entry": "0x140c9b990", "native_body_bytes": 1002,
            "execution": "x86_64 Rosetta, PE sections mapped at original ImageBase; original itertools helper executed",
            "bridges": ["core::panicking::panic exit only; no normal-path or geometry bridge"],
            "fixture_sha256": digest(FIXTURE), "probe_source_sha256": digest(SOURCE),
            "executable_sha256": digest(executable),
            "input_sha256": hashlib.sha256(raw).hexdigest(),
            "output_sha256": hashlib.sha256(actual).hexdigest(),
            "cases": len(cases), "native_panics": sum(case["expected"] == 2 for case in cases),
            "mismatches": mismatches, "stderr": run.stderr,
            "rust_verification": "cargo test --offline -p utils complete_original_intersects_and_iterator_match_results_and_guards; separate log",
        }
        REPORT.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
        print(json.dumps({"cases": len(cases), "mismatches": len(mismatches), "report": str(REPORT)}, ensure_ascii=False))
        assert not mismatches


if __name__ == "__main__":
    main()
