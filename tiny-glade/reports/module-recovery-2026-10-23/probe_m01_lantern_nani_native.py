"""原灯实例数值块差分数据；不重写TRS、半精度打包或flag算法。"""
from pathlib import Path
import hashlib
import json
import math
import random
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = ROOT.parent / "B版本内容/tiny-glade.rne"
SOURCE = Path(__file__).with_name("probe_m01_lantern_nani_x86.c")
FIXTURE = ROOT / "tests/fixtures/native/m01-lantern-nani.json"
REPORT = Path(__file__).with_name("m01-lantern-nani-native.json")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def bits(value):
    return struct.unpack("<I", struct.pack("<f", value))[0]


def inputs():
    rng = random.Random(0x207A4F0)
    special = [0, 0x80000000, bits(1), bits(-2), bits(65504), bits(65520),
               bits(2**-24), bits(2**-25), 0x7F800000, 0xFF800000,
               0x7FC12345, 0x7F812345, bits(1.00048828125), bits(1.00146484375)]
    colors = [0, 1, 7, 0x3FFFFFFF, 0x40000000, 0x80000000, 0xFFFFFFFF]
    flags = [0, 1, 2, 3, 4, 127, 128, 255]
    for index in range(448):
        # 有效单位四元数；生产Transform要求有限，尾部半精度参数另含非有限边界。
        angle = (index % 17 - 8) * 0.125
        axis = index % 3
        rotation = [0.0, 0.0, 0.0, math.cos(angle)]
        rotation[axis] = math.sin(angle)
        translation = [rng.uniform(-100, 100) for _ in range(3)]
        scales = [1.0, -1.0, 0.0, -0.0, 0.125, 8.25, 0.000001]
        scale = [scales[(index + n * 3) % len(scales)] for n in range(3)]
        transform = [bits(v) for v in rotation + translation + scale] + [0xA1B2C3D4, 0xF0E1D2C3]
        params = [rng.getrandbits(32) for _ in range(9)]
        for n in range(4):
            params[3 + n] = special[(index + 3*n) % len(special)]
        params[8] = (params[8] & 0xFFFFFF00) | flags[index % len(flags)]
        yield {"id": index, "transform": transform, "params": params,
               "color": colors[index % len(colors)], "semi": (index // 7) & 1}


def main():
    original = ORIGINAL.read_bytes()
    assert sha(original) == "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
    rows = list(inputs())
    raw = b"".join(struct.pack("<24I", row["id"], *row["transform"], *row["params"], row["color"], row["semi"]) for row in rows)
    with tempfile.TemporaryDirectory(prefix="m01-lantern-nani-") as directory:
        temporary = Path(directory)
        executable, input_file, output_file = [temporary / name for name in ["probe", "input.bin", "output.bin"]]
        input_file.write_bytes(raw)
        subprocess.run(["clang", "-arch", "x86_64", "-O2", str(SOURCE), "-o", str(executable)], check=True)
        completed = subprocess.run([str(executable), str(ORIGINAL), str(input_file), str(output_file)], capture_output=True, text=True, check=True)
        output = output_file.read_bytes()
        assert len(output) == len(rows) * 72
        for row, result in zip(rows, struct.iter_unpack("<18I", output)):
            assert result[0] == row["id"]
            row["expected_words"] = list(result[1:17])
            row["expected_sort"] = result[17]
        fixture = {"original_sha256": sha(original), "entry": "0x14207a4f0",
                   "numeric_start": "0x14207ad95", "numeric_end_exclusive": "0x14207af29",
                   "scope": "Original numeric instructions and original pack_2x16f/f16 dependencies; not whole Bevy query execution.",
                   "cases": rows}
        FIXTURE.write_text(json.dumps(fixture, separators=(",", ":")) + "\n")
        REPORT.write_text(json.dumps({"original_sha256": sha(original), "source_sha256": sha(SOURCE.read_bytes()),
            "executable_sha256": sha(executable.read_bytes()), "input_sha256": sha(raw), "output_sha256": sha(output),
            "fixture_sha256": sha(FIXTURE.read_bytes()), "cases": len(rows), "stderr": completed.stderr,
            "instrumentation": "Entry registers supplied; output-only jump at af29 after original numeric block. No numeric/helper bridge.",
            "whole_function_native_execution": False}, indent=2) + "\n")
        print(completed.stderr.strip())


if __name__ == "__main__":
    main()
