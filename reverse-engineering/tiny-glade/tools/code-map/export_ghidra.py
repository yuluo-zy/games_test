"""Run ONLY through ReVa run-script in a unique worker; reads the database.

The caller replaces OUTPUT with an exclusive, absolute local output filename.
"""
import gzip
import hashlib
import json
from pathlib import Path

expected = "cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323"
actual = str(currentProgram.getExecutableSHA256()).lower()
if actual != expected:
    raise RuntimeError("Wrong analysis binary: " + actual)
target = Path(OUTPUT)
count = 0
with target.open("xb") as raw:
    with gzip.GzipFile(filename="", fileobj=raw, mode="wb", mtime=0) as compressed:
        header = {"kind": "identity", "sha256": actual,
                  "program": str(currentProgram.getName()),
                  "image_base": int(str(currentProgram.getImageBase()), 16)}
        compressed.write((json.dumps(header) + "\n").encode())
        for function in currentProgram.getFunctionManager().getFunctions(True):
            monitor.checkCancelled()
            ranges = []
            for span in function.getBody().getAddressRanges():
                ranges.append([int(str(span.getMinAddress()), 16),
                               int(str(span.getMaxAddress()), 16) + 1])
            row = {"kind": "function", "va": int(str(function.getEntryPoint()), 16),
                   "name": str(function.getName(True)), "ranges": ranges,
                   "thunk": bool(function.isThunk()), "external": bool(function.isExternal())}
            compressed.write((json.dumps(row, ensure_ascii=False) + "\n").encode())
            count += 1
print(json.dumps({"path": str(target), "functions": count, "sha256": actual,
                  "export_sha256": hashlib.sha256(target.read_bytes()).hexdigest()}))
