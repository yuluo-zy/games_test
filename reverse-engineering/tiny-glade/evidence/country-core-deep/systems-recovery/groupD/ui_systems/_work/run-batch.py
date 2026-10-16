"""D2 restorer batch decompiler (tg_d2).

Decompiles the 33 business VAs of system-decorator ui_systems (batch 2, group D).
Writes evidence to systems-recovery/groupD/ui_systems/decomp/decomp-<va>.txt
"""
import json, os, subprocess, sys
import concurrent.futures as cf

ROOT = r"D:/game/reverse-engineering/tiny-glade"
WORK = os.path.join(ROOT, "evidence/country-core-deep/systems-recovery/groupD/ui_systems/_work")
BIN = os.path.join(ROOT, "workspace/input/tiny-glade.exe")
DBID = "tg_d2"
CHUNKS = 2

TARGETS = {
    "0x14127c990": "calculate_decorator_dst::convert_raycast_hit_to_decorator_dst",
    "0x141277bb0": "calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid",
    "0x141278050": "calculate_decorator_grab_offset::raycast_with_ss_offset",
    "0x14127f0e0": "decorator_interaction_intent::snap_balcony_door",
    "0x14127cf70": "decorator_interaction_intent::calculate_decorator_interaction_intent",
    "0x14127a790": "handle_splitting_roof_decorators::try_display_roof_decorator_parts_and_get_hovered_part_id",
    "0x14127bab0": "handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part",
    "0x1412782b0": "handle_splitting_wall_decorators::display_decorator_wall_parts_and_get_hovered_part_id",
    "0x141279410": "handle_splitting_wall_decorators::handle_moving_out_wall_decorator_part",
    "0x14127fe10": "move_ops::show_decorator_remove_signifier",
    "0x1412801f0": "move_ops::calculate_move_dst",
    "0x1407ae2e0": "ui_change_chimney_height::ui_change_chimney_height",
    "0x1407ae910": "ui_change_chimney_height::show_signifiers",
    "0x1407adea0": "ui_change_stair_height::ui_change_stair_height",
    "0x1407af330": "ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher::clear",
    "0x1407af340": "ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance",
    "0x1412823c0": "ui_decorator_affordance_dispatch::create_decorator_widget<&str>",
    "0x141282300": "ui_decorator_affordance_dispatch::create_decorator_widget<DecoratorHandleName>",
    "0x14079c6d0": "ui_focus_stairs::ui_focus_stairs",
    "0x140791400": "ui_inactive::ui_decorator_inactive",
    "0x1407a4850": "ui_move_decorator::ui_move_decorator",
    "0x1407a84d0": "ui_move_decorator::init_state",
    "0x1407a7e80": "ui_move_decorator::find_stair_placement_candidate",
    "0x1407a9c10": "ui_place_decorator::ui_place_decorator",
    "0x1407adb20": "ui_place_decorator::show_decorator_icon",
    "0x1407adcb0": "ui_place_decorator::decorator_cursor_icon",
    "0x1407a8580": "ui_rotate_lantern::ui_rotate_decorator",
    "0x14079c350": "ui_stairs_misc::stairs_reset_for_new_frame",
    "0x14079c3a0": "ui_stairs_misc::continue_stairs",
    "0x140791510": "ui_stairs_viz::visualize_ui_stairs",
    "0x14079bc20": "ui_stairs_viz::insert_point_ui",
    # SystemParam glue (shared VA)
    "0x1407a8550": "ui_move_decorator::SystemParam glue (apply/new_archetype/queue)",
    "0x1403f8dd0": "ui_move_decorator::get_param",
    "0x1403f8c50": "ui_move_decorator::validate_param",
}

def main():
    steps = [{"tool": "decompile_function",
              "args": {"database": DBID, "address": va},
              "timeout_ms": 240000, "allow_error": True,
              "_note": name} for va, name in sorted(TARGETS.items())]
    chunk_steps = [steps[i::CHUNKS] for i in range(CHUNKS)]
    outdir = os.path.join(ROOT, "evidence/country-core-deep/systems-recovery/groupD/ui_systems/decomp")
    os.makedirs(outdir, exist_ok=True)

    def run_chunk(ci):
        csteps = chunk_steps[ci]
        cdbid = f"{DBID}_c{ci}"
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
        cjpath = os.path.join(WORK, f"chunk-{ci}-job.json")
        copath = os.path.join(WORK, f"chunk-{ci}-report.json")
        json.dump(cjob, open(cjpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
        r = subprocess.run(["node", os.path.join(ROOT, "scripts/mcp-job.mjs"),
                            "re_mcp_ghidra", cjpath, copath], cwd=ROOT,
                           capture_output=True, text=True, timeout=7200)
        return ci, copath, r.returncode, r.stdout[-300:], r.stderr[-300:]

    reports = {}
    with cf.ThreadPoolExecutor(max_workers=CHUNKS) as ex:
        for ci, copath, rc, so, se in ex.map(run_chunk, range(CHUNKS)):
            print(f"[chunk {ci}] exit={rc} {so[-200:]}")
            if rc != 0:
                print("CHUNK FAILED", se[-600:])
                sys.exit(1)
            reports[ci] = copath
    merged = {"results": []}
    for ci in sorted(reports):
        rep = json.load(open(reports[ci], encoding="utf-8"))
        merged["results"].extend(rep["results"])
    outpath = os.path.join(WORK, "batch-report.json")
    json.dump(merged, open(outpath, "w", encoding="utf-8"), ensure_ascii=False, indent=1)

    summary = []
    for res in merged["results"]:
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
                    text = j["decompiled_code"]
            except Exception:
                pass
        status = "ERR" if is_err else "OK"
        if not is_err:
            dp = os.path.join(outdir, f"decomp-{addr}.txt")
            with open(dp, "w", encoding="utf-8") as f:
                f.write(f"// {note}\n")
                f.write(text)
        summary.append({"va": addr, "note": note, "status": status, "chars": len(text)})
    with open(os.path.join(WORK, "batch-summary.json"), "w", encoding="utf-8") as f:
        json.dump(summary, f, ensure_ascii=False, indent=1)
    for s in summary:
        print(f"{s['va']} {s['status']} {s['chars']} {s['note']}")

if __name__ == "__main__":
    main()
