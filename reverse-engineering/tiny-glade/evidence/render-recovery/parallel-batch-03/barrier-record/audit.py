from pathlib import Path
import json
import re
import subprocess

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
calls = json.loads((ROOT / "evidence/render-recovery/workgraph/calls.json").read_text(encoding="utf8"))["functions"]
functions = {entry["va"]: entry for entry in json.loads((ROOT / "evidence/render-recovery/workgraph/functions.json").read_text(encoding="utf8"))}
targets = {"0x140be7ba0", "0x140be78b0", "0x1402c3d00", "0x1402c1d40"}
callers = []
for address, entry in calls.items():
    edges = [edge for edge in entry.get("direct", []) if edge["target"] in targets]
    if edges:
        callers.append({"address": address, "edges": edges, "names": functions.get(address, {}).get("names", [])})
        metadata = functions.get(address)
        if metadata:
            asm = subprocess.run(["llvm-objdump", "-d", "--start-address=" + address, "--stop-address=" + metadata["end_exclusive"], str(ROOT / "workspace/input/tiny-glade.exe")], text=True, capture_output=True, check=True).stdout
            (OUT / (address + ".caller.objdump.txt")).write_text(asm, encoding="utf8")
            callers[-1]["marker_like_store_candidates"] = [line for line in asm.splitlines() if re.search(r"movb\s+\$0x1, 0x2\(", line)]
population = (OUT / "populate.objdump.txt").read_text(encoding="utf8")
marker_one = [line for line in population.splitlines() if re.search(r"movb\s+\$0x1, 0x2\(", line)]
assert len(marker_one) == 2
local_calls = [line for line in population.splitlines() if any(address in line for address in ["0x1417e7f20", "0x1417e7c30", "0x1418bcc30", "0x1418bce70"])]
report = {"passed": True, "search_scope": "selected workgraph direct edges, original population disassembly, direct callers", "selected_function_count": len(functions), "caller_count": len(callers), "callers": callers, "population_marker_one_stores": marker_one, "local_hash_remove_edges": local_calls, "no_whole_program_absence_claim": True}
(OUT / "negative-search-audit.json").write_text(json.dumps(report, indent=2), encoding="utf8")
print(json.dumps({"passed": True, "callers": len(callers), "marker_one_stores": len(marker_one)}))
