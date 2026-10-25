"""Parallel-agent batch decompile runner for country-core systems/resources.

Usage: python scripts/run-decomp-batch.py <domain> <dbid> [--only-summary]

- Filters evidence/country-core-deep/sysres-procedures.json by DOMAINS[domain] substrings
- Builds a RE-MCP job (open/wait/decompile*/close, allow_error on decompiles)
- Invokes scripts/mcp-job.mjs, then extracts per-function text files to
  evidence/country-core-deep/<domain>/decomp/decomp-<va>.txt and writes summary.json
"""
import json, os, re, subprocess, sys, time

ROOT = r"D:/game/reverse-engineering/tiny-glade"
IDX = os.path.join(ROOT, "evidence/country-core-deep/sysres-procedures-v3.json")
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
LOCK = os.path.join(ROOT, "evidence/country-core-deep/.mcp-batch.lock")

def acquire_lock(no_lock, timeout=7200):
    """Serializes Ghidra MCP sessions across agents (machine RAM constraint)."""
    if no_lock:
        return None
    start = time.time()
    while True:
        try:
            fd = os.open(LOCK, os.O_CREAT | os.O_EXCL | os.O_WRONLY)
            os.write(fd, f"{os.getpid()} {time.time()}\n".encode())
            os.close(fd)
            return fd
        except FileExistsError:
            if time.time() - start > timeout:
                print("LOCK TIMEOUT", file=sys.stderr)
                sys.exit(1)
            time.sleep(10)

def release_lock():
    try:
        os.unlink(LOCK)
    except OSError:
        pass

DOMAINS = {
    "history": ["/history"],
    "render-top": ["render/"],
    "wall": ["resources/walls/", "systems/wall/", "stairs_walls", "shape_intersections"],
    "session-save": ["session.rs", "systems/save/", "systems/prefab", "resources/prefab/",
                     "loading_screen", "logo_animation"],
    "terrain-water": ["terrain", "water", "world_raster", "raster_stroke", "garden",
                      "in_progress_curve", "drawing_input_smoother", "brush_sizes",
                      "cursor_terrain_raycast", "mouse_terrain_raycast", "read_mouse_depth_value",
                      "mesh_follow_terrain", "rocky_terrain", "glade_border", "garden_mask",
                      "water_mask", "water_exclude_clutter_mask"],
    "ui-input": ["systems/ui/", "ui_", "mode_manager", "mod_key", "input/", "mouse_capture",
                 "toolbar", "photo_mode", "onboarding", "gui"],
    "camera-time": ["camera_rig", "main_camera", "look_at_camera", "ui_camera_mode_gizmos",
                    "time_of_day", "time_update", "weather", "daily", "glade_settings",
                    "timelapse_replay", "arrow_gizmo_animation", "sparkle", "clear_canvas",
                    "despawn"],
    "collision-eco": ["collision/", "stairs/", "critters", "paths/", "ivy/", "clutter_meshes",
                      "preplaced_autoclutter"],
    "audio-misc": ["audio", "steam", "debug", "shutdown", "feedback_report_resources", "gpu_lut",
                   "grading_lut", "color_icon", "color_id", "gothic_window_profile",
                   "glade_color_customization", "image_formation", "placement_data",
                   "texture_presets", "render_ssbo", "shader_refactoring_state", "window",
                   "editing_highlight", "app_loading_state", "frozen_map", "meta_commander",
                   "name_hash"],
    "render-loop": ["systems/render_loop"],
}

def main():
    domain, dbid = sys.argv[1], sys.argv[2]
    only_summary = "--only-summary" in sys.argv
    # 默认并发（内存足够时）；--lock 恢复全局串行
    no_lock = "--lock" not in sys.argv
    chunks = 1
    if "--chunks" in sys.argv:
        chunks = int(sys.argv[sys.argv.index("--chunks") + 1])
    if chunks > 1 and not no_lock:
        chunks = 1  # --lock 串行模式下分片无意义
    pats = DOMAINS[domain]
    idx = json.load(open(IDX, encoding="utf-8"))
    steps, seen = [], set()
    for f in sorted(idx):
        if not any(p in f for p in pats):
            continue
        for p in idx[f]["procedures"]:
            va = p["va"]
            if va in seen:
                continue
            seen.add(va)
            steps.append({"tool": "decompile_function",
                          "args": {"database": dbid, "address": va},
                          "timeout_ms": 240000, "allow_error": True,
                          "_note": f"{f} :: {p['name'][:90]}"})
    job = [
        {"tool": "open_database",
         "args": {"file_path": BIN, "run_auto_analysis": False, "database_id": dbid,
                  "keep_open": True}, "timeout_ms": 600000},
        {"tool": "wait_for_analysis", "args": {"database": dbid}, "timeout_ms": 600000},
    ] + steps + [
        {"tool": "close_database", "args": {"database": dbid, "save": False},
         "timeout_ms": 180000},
    ]
    outdir = os.path.join(ROOT, "evidence/country-core-deep", domain)
    os.makedirs(os.path.join(outdir, "decomp"), exist_ok=True)
    jobpath = os.path.join(outdir, "batch-job.json")
    outpath = os.path.join(outdir, "batch-report.json")
    if chunks > 1:
        # split decompile steps into N chunks, run concurrently with separate
        # MCP sessions (each chunk its own database id dbid_c0..cn), merge reports.
        import concurrent.futures as cf
        chunk_steps = [steps[i::chunks] for i in range(chunks)]
        print(f"[{domain}] {len(steps)} steps in {chunks} chunks")
        if only_summary:
            return

        def run_chunk(ci):
            csteps = chunk_steps[ci]
            cdbid = f"{dbid}_c{ci}"
            # rewrite each step's database to the chunk's own session id
            csteps = [{**s, "args": {**s["args"], "database": cdbid}} for s in csteps]
            cjob = ([
                {"tool": "open_database",
                 "args": {"file_path": BIN, "run_auto_analysis": False,
                          "database_id": cdbid, "keep_open": True},
                 "timeout_ms": 600000},
                {"tool": "wait_for_analysis", "args": {"database": cdbid},
                 "timeout_ms": 600000},
            ] + csteps + [
                {"tool": "close_database", "args": {"database": cdbid, "save": False},
                 "timeout_ms": 180000},
            ])
            cjpath = os.path.join(outdir, f"chunk-{ci}-job.json")
            copath = os.path.join(outdir, f"chunk-{ci}-report.json")
            json.dump(cjob, open(cjpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
            r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                                "re_mcp_ghidra", cjpath, copath], cwd=ROOT,
                               capture_output=True, text=True)
            return ci, copath, r.returncode, r.stdout[-500:], r.stderr[-500:]

        reports = {}
        with cf.ThreadPoolExecutor(max_workers=chunks) as ex:
            for ci, copath, rc, so, se in ex.map(run_chunk, range(chunks)):
                print(f"[chunk {ci}] exit={rc} {so[-200:]}")
                if rc != 0:
                    print("CHUNK FAILED", se[-800:])
                    sys.exit(1)
                reports[ci] = copath
        # merge
        merged = {"results": []}
        for ci in sorted(reports):
            rep = json.load(open(reports[ci], encoding="utf-8"))
            merged["results"].extend(rep["results"])
        json.dump(merged, open(outpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        # also keep per-chunk jobs' step notes in a merged job file for reference
        all_steps = [s for cs in chunk_steps for s in cs]
        json.dump([{"tool": "open_database",
                    "args": {"file_path": BIN, "run_auto_analysis": False,
                             "database_id": dbid, "keep_open": True}},
                   {"tool": "wait_for_analysis", "args": {"database": dbid}}]
                  + all_steps
                  + [{"tool": "close_database", "args": {"database": dbid, "save": False}}],
                  open(jobpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    else:
        json.dump(job, open(jobpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        print(f"[{domain}] {len(steps)} decompile steps -> {jobpath}")
        if only_summary:
            return
        acquire_lock(no_lock)
        print(f"[{domain}] MCP lock acquired, running batch...")
        try:
            r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                                "re_mcp_ghidra", jobpath, outpath], cwd=ROOT,
                               capture_output=True, text=True)
        finally:
            release_lock()
        print(r.stdout[-2000:])
        if r.returncode != 0:
            print("RUNNER FAILED:", r.stderr[-2000:])
            sys.exit(1)
    report = json.load(open(outpath, encoding="utf-8"))
    summary = []
    for res in report["results"]:
        if res["tool"] != "decompile_function":
            continue
        addr = res["args"]["address"]
        note = res.get("_note", "")
        content = res["result"].get("content")
        text, is_err = "", bool(res["result"].get("isError"))
        if content:
            text = content[0].get("text", "")
            try:
                j = json.loads(text)
                if "decompiled_code" in j:
                    text = j.get("decompiled_code", "")
                elif "disassembly" in j:
                    text = j.get("disassembly", "")
            except Exception:
                pass
        fn = os.path.join(outdir, "decomp", f"decomp-{addr}.txt")
        open(fn, "w", encoding="utf-8").write((note + "\n" if note else "") + text)
        summary.append({"address": addr, "bytes": len(text), "error": is_err, "note": note})
    json.dump(summary, open(os.path.join(outdir, "summary.json"), "w", encoding="utf-8"),
              ensure_ascii=False, indent=1)
    ok = sum(1 for s in summary if not s["error"])
    print(f"[{domain}] done: {ok}/{len(summary)} ok; decomp/ + summary.json written")
    for s in summary:
        if s["error"]:
            print("  ERR", s["address"], s["note"][:70])

if __name__ == "__main__":
    main()
