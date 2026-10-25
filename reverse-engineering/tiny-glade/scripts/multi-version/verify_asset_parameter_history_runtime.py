"""Actual Bevy CPU owner integration; native leaf proofs are separate reports."""
import hashlib
import json
import subprocess
from verification_provenance import ROOT, candidate_provenance


def main():
    fixture = ROOT / "reconstruction/crates/tg-building-runtime/fixtures/asset-parameter-history-cycle.json"
    executable = ROOT / "reconstruction/target/release/building-run"
    execution = subprocess.run([str(executable), str(fixture)], check=True, capture_output=True, text=True)
    frames = [json.loads(line) for line in execution.stdout.splitlines()]
    assert len(frames) == 14
    assert all(not frame["faults"] for frame in frames)
    assert frames[0]["created"] == 1 and frames[0]["hole_geometry"][0]["final_records"] == 1
    assert frames[1]["hole_geometry_rebuilt"] == [] and frames[1]["nani_requests"] == {"create": 0, "update": 0, "remove": 0}
    assert frames[2]["history"]["committed"] == [0]
    heights = [frame["roofs"][0]["height"] for frame in frames[:12]]
    assert heights[2] > heights[3] > heights[4] > heights[5] > heights[6]
    assert heights[6] == .5 and heights[7] == heights[2]
    assert [frame["history"]["replay_buckets"] for frame in frames[3:7]] == [1, 1, 1, 0]
    assert frames[6]["history"]["completed_buckets"] == [0]
    assert frames[8]["roofs"][0]["cpu_tile_records"] == 0
    assert frames[9]["roofs"][0]["cpu_tile_records"] > 0
    assert frames[10]["hole_geometry"][0]["final_records"] == 3
    assert frames[11]["openings"] == [] and frames[11]["hole_geometry"][0]["final_records"] == 0
    assert frames[12]["walls"] == [] and frames[12]["roofs"] == [] and frames[12]["nani_staged_entities"] == 0
    assert frames[13]["nani_requests"] == {"create": 0, "update": 0, "remove": 0}
    # Asset identities come from the actual B native loader selection report.
    bindings_path = ROOT / "evidence/building-placement/B-plain-wall-asset-bindings.json"
    report = {
        "ok": True,
        "scope": "Actual Bevy raw wall + shipped collision assets + wall-space placement + opening cache + Roof edit/type CPU consumers + Roof owner recording/timed replay + delete cleanup; adapter scheduling, not original fork graph.",
        "fixture_sha256": hashlib.sha256(fixture.read_bytes()).hexdigest(),
        "candidate_binary_sha256": hashlib.sha256(executable.read_bytes()).hexdigest(),
        "native_asset_bindings_sha256": hashlib.sha256(bindings_path.read_bytes()).hexdigest(),
        "frames": frames,
        "limitations": [
            "Wall row-to-proposal production, structural output families, all history auxiliary owners and full snapshots are not proved by this integration.",
            "Roof native type switch GPU/VAO/transient mesh commands remain backend boundaries.",
            "World intent persistence/reprojection and cross-source history system order are adapter policies.",
            "B CPU integration does not accept C migration or the unresolved ARM rectangular tile 1ULP failure.",
        ],
        **candidate_provenance(),
    }
    output = ROOT / "evidence/building-integrate/B-asset-parameter-history-runtime-verification.json"
    output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"ok": True, "frames": len(frames)}))


if __name__ == "__main__":
    main()
