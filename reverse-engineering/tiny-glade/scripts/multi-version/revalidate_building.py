"""Reexecute frozen-source proofs; orchestration is not reconstruction progress."""
import argparse
import concurrent.futures
import json
import subprocess
import sys
import time
from verification_provenance import ROOT, candidate_provenance


VERIFIERS = [
    "verify_wall_collection", "verify_wall_terrain_versions", "verify_wall_collection_dirty",
    "verify_wall_spatial_index", "verify_hole_invalidation", "verify_wall_holes",
    "verify_gable_arch", "verify_nani", "verify_wall_cache", "verify_roof_direction",
    "verify_roof_customization", "verify_opening_rules", "verify_openings_history_runtime",
    "verify_runtime", "verify_history", "verify_parameters", "verify_replay",
    "verify_discrete_replay", "verify_history_chain", "verify_history_transaction",
    "verify_building_state", "verify_roof_creation", "verify_building_chain",
    "verify_building_cli", "verify_lifecycle", "verify_lifecycle_cycle",
    "verify_component_versions", "verify_decorator_placement", "verify_decorator_subtype",
    "verify_building_history", "verify_wall_height_authority", "verify_wall_height_command",
    "verify_wall_constructor", "verify_asset_parameter_history_runtime",
    "verify_wall_construction_sources", "verify_wall_production", "verify_wall_path_preparation",
    "verify_wall_freeform_cache", "verify_raw_wall_production_runtime",
    "verify_wall_construction_policy", "verify_wall_path_nonempty", "verify_wall_production_lifecycle",
    "verify_wall_arches", "verify_wall_arches_freehand", "verify_arches_wall_runtime",
    "verify_wall_history_timed", "verify_wall_history_groups", "verify_wall_history_transition", "verify_wall_bulldoze",
    "verify_wall_platform_stair_geometry", "verify_wall_platform_stair_chain", "verify_wall_platform_stair_rows", "verify_wall_platform_stair_stats", "verify_stair_wall_runtime", "verify_wall_platform_stairs_consumer",
    "verify_wall_segmentation", "verify_wall_segmentation_schema", "verify_wall_segmentation_material_stats", "verify_wall_platform_segmentation", "verify_wall_authoring", "verify_authored_wall_runtime", "verify_wall_structures", "verify_lintel_structures", "verify_lintel_wall_runtime", "verify_wall_history_authority", "verify_wall_platform_stair_segments",
]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--retry-failed",action="store_true",help="Reexecute only failed tasks when the entire Rust/Cargo provenance is identical to the saved run")
    args = parser.parse_args()
    frozen = candidate_provenance()
    if not args.skip_build and not args.retry_failed:
        subprocess.run([sys.executable, str(ROOT / "scripts/multi-version/verify_build.py")], cwd=ROOT, check=True)
    destination = ROOT / "evidence/building-integrate/revalidation"
    destination.mkdir(parents=True, exist_ok=True)
    tasks = [(name, [sys.executable, str(ROOT / "scripts/multi-version" / f"{name}.py")]
              + (["--all"] if name == "verify_wall_spatial_index" else ["--branches"] if name == "verify_wall_production" else ["--cases", "128"] if name == "verify_wall_path_nonempty" else [])) for name in VERIFIERS]
    tasks.extend([
        ("roof_state", [sys.executable, str(ROOT / "scripts/verify-roof-state.py")]),
        ("source_edits", [sys.executable, str(ROOT / "scripts/source-edits/verify.py")]),
    ])

    prior=None
    if args.retry_failed:
        prior=json.loads((destination/"summary.json").read_text())
        if any(prior.get(key)!=value for key,value in frozen.items()):
            raise RuntimeError("Retry cannot retain prior successes after any Rust/Cargo provenance change")
        failed_names={row["name"] for row in prior["results"] if row["exit"]}
        tasks=[task for task in tasks if task[0] in failed_names]

    def execute(task):
        name, command = task
        start = time.monotonic()
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True)
        log = destination / f"{name}.log"
        log.write_text(result.stdout + result.stderr)
        return {"name": name, "exit": result.returncode, "seconds": round(time.monotonic() - start, 1),
                "log": log.relative_to(ROOT).as_posix(), "tail": (result.stdout + result.stderr)[-900:]}

    results = [] if prior is None else [row for row in prior["results"] if not row["exit"]]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        for future in concurrent.futures.as_completed([pool.submit(execute, task) for task in tasks]):
            result = future.result()
            results.append(result)
            print(json.dumps({"name": result["name"], "exit": result["exit"], "seconds": result["seconds"]}), flush=True)
    unchanged = frozen == candidate_provenance()
    failed = [result["name"] for result in results if result["exit"]]
    summary = {"ok": unchanged and not failed, "sources_remained_frozen": unchanged, "results": results,
               "scope": "Current scoped native/CPU integration reexecution; not complete building, C migration or GPU acceptance.",
               **frozen}
    if prior is not None:
        summary["retry_history"]=prior.get("retry_history",[])+[{"failed_results_before_reexecution":[row for row in prior["results"]if row["exit"]],"successful_results_retained_only_with_identical_rust_cargo_inputs":True}]
    (destination / "summary.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
    print(json.dumps({"done": len(results), "failed": failed, "sources_remained_frozen": unchanged}))
    if not summary["ok"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
