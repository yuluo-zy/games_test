import json
import verify as oracle
import runpy

R2 = oracle.ROOT / "evidence/render-recovery/parallel-batch-02/graph-compiler"
snapshot = json.loads((R2 / "multi-snapshot.json").read_text(encoding="utf8"))
checks = []
for transition in snapshot["initial_transitions"]:
    mapping = snapshot["physical_mapping"][transition["resource"]]
    kind = int(mapping["payload_tag"] in [2, 3])
    case = [1, kind, mapping["initial_access"], int(mapping["reused"]), 4, 0, 0, transition["to"], 0, 0, 1, 0]
    native, _ = oracle.execute(case)
    assert oracle.STATE["transitions"] == [[kind, transition["from"], transition["to"]]]
    checks.append({"invariant":"initial R2 snapshot", "case":case, "native":native})
for pass_index, transitions in enumerate(snapshot["pre_transitions"]):
    for transition in transitions:
        mapping = snapshot["physical_mapping"][transition["resource"]]
        kind = int(mapping["payload_tag"] in [2, 3])
        case = [0, kind, mapping["initial_access"], int(mapping["reused"]), transition["from"], 0, int(transition["from"] in [2, 3]), transition["to"], 0, int(transition["to"] in [2, 3]), 1, 0]
        native, _ = oracle.execute(case)
        assert oracle.STATE["transitions"] == [[kind, transition["from"], transition["to"]]]
        checks.append({"invariant":"pre R2 snapshot", "pass":pass_index, "case":case, "native":native})
for transition in snapshot["export_transitions"]:
    mapping = snapshot["physical_mapping"][transition["resource"]]
    kind = int(mapping["payload_tag"] in [2, 3])
    case = [2, kind, mapping["initial_access"], int(mapping["reused"]), transition["from"], 0, 0, transition["to"], 0, 0, 1, 0]
    native, _ = oracle.execute(case)
    assert oracle.STATE["transitions"] == [[kind, transition["from"], transition["to"]]]
    checks.append({"invariant":"export R2 snapshot", "case":case, "native":native})
empty = json.loads((R2 / "empty-snapshot.json").read_text(encoding="utf8"))
assert empty["batches"] == [{"start":0,"end":1,"export_barriers":True}]
assert empty["export_transitions"] == [] and empty["pre_transitions"] == [[]]
checks.append({"invariant":"R2 empty pass has final batch but no transitions", "scope":"snapshot assertion, not native compile entry"})
failure = json.loads((R2 / "backend-error-snapshot.json").read_text(encoding="utf8"))
population = runpy.run_path(str(oracle.OUT / "verify-population.py"))
native_failure = population["execute"]([(1, 0, 3, 1), (0, 0, 0, 1), (1, 0, 4, 1)], [0, 1, 2], False, failure["tag"])
assert native_failure["physical_count"] == failure["partial_backing_count"]
assert sum(tag == 3 for tag in native_failure["origin_tags"]) == failure["consumed_origins"]
checks.append({"invariant":"R2 backend error partial consumption", "native_origin_tags":native_failure["origin_tags"], "native_backing_count":native_failure["physical_count"]})
report = {"passed":True,"checks":checks,"r2_status_unchanged":"candidate_static","limits":["Each barrier extracted into single-resource fixture; not complete graph compiler differential","R2 files are read-only","Empty graph check uses R2 snapshot; no native compile invocation"]}
(oracle.OUT / "snapshot-contract-checks.json").write_text(json.dumps(report, indent=2), encoding="utf8")
print(json.dumps({"passed":True,"checks":len(checks)}))
