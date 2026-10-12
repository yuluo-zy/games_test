"""完整图产生的命令与同步 Batch 输入原 queue_submit 入口；不是整游戏 oracle。"""
from pathlib import Path
import copy
import hashlib
import importlib.util
import json
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("queue_native", ROOT / "scripts/native_support/queue_entry.py")
q = importlib.util.module_from_spec(spec)
spec.loader.exec_module(q)


def vector_input(pointer, offset, width, values):
    data = q.alloc(width * len(values)) if values else width
    if values:
        q.b.write(data, "<" + ({4: "I", 8: "Q"}[width]) * len(values), *values)
    q.b.write(pointer + offset, "<QQQ", len(values), data, len(values))


def original(call, sequences, queue, start, error):
    tag, index = queue
    handles = [handle for batch in call["submits"] for handle in batch["commands"]]
    synchronized = len(call["submits"]) == 2
    q.setup(count=0 if synchronized else len(handles), kind=tag, index=index,
            prior=len(handles) if synchronized else 0, start=start, error=error)
    commands = q.EXTRA + 0x9000 if synchronized else q.COMMANDS
    for ordinal, handle in enumerate(handles):
        pointer = commands + ordinal * 88
        q.b.write(pointer, "<QQII", 2, sequences[handle], tag, index)
        q.b.write(pointer + 0x48, "<Q", handle)
    if synchronized:
        # 只输入原 Batch 的 Vec 字段；VkSubmitInfo、pNext、队列 timeline 由原入口生成。
        source = call["submits"][0]
        pointer = q.PAYLOAD + 0x40
        count = len(source["commands"])
        vector_input(pointer, 0, 8, source["wait_semaphores"])
        vector_input(pointer, 24, 4, source["wait_stages"])
        vector_input(pointer, 48, 8, source["timeline"]["wait_values"])
        # 最后 count 个内部 timeline 信号不能注入；原入口必须从命令独立产生。
        vector_input(pointer, 72, 8, source["signal_semaphores"][:-count])
        vector_input(pointer, 96, 8, source["timeline"]["signal_values"][:-count])
    q.CPU.emu_start(0x140C04E20, q.b.STOP, count=1000000)
    returned = q.CPU.reg_read(q.UC_X86_REG_RIP) == q.b.STOP
    queue_pointer = q.b.DEVICE + 0x730 if tag == 0 else q.QUEUES + index * 0xB0
    capacity, pointer, head, length = q.b.read(queue_pointer + 0x20, "<QQQQ")
    pending = [{"state": q.b.read(pointer + ((head+i) % capacity)*96, "<Q")[0],
                "sequence": q.b.read(pointer + ((head+i) % capacity)*96 + 8, "<Q")[0],
                "timeline": q.b.read(pointer + ((head+i) % capacity)*96 + 88, "<Q")[0]}
               for i in range(length)]
    return {"backend": copy.deepcopy(q.STATE["backend"]),
            "receipt": list(q.b.read(q.RECEIPT, "<QQ")) if returned else None,
            "next_timeline": q.b.read(q.b.DEVICE+0x7F8, "<Q")[0],
            "pending_commands": pending, "panic": q.STATE["panic"],
            "original_full_entry_visited": "0x140c04e20" in q.STATE["visited"]}


def main():
    cargo = ["cargo", "build", "--locked", "--offline", "--manifest-path", str(ROOT/"Cargo.toml"), "--example", "queue_graph_gpu"]
    subprocess.run(cargo, check=True)
    result = subprocess.run([str(ROOT/"target/debug/examples/queue_graph_gpu.exe"), "--cpu"], capture_output=True, text=True, check=True)
    cases = json.loads(result.stdout)
    failures = []
    compared = 0
    observations = []
    for case_index, case in enumerate(cases):
        sequences = {}
        for event in case["events"]:
            if event.startswith("begin:"):
                _, handle, sequence, tag, index = event.split(":")
                assert [int(tag), int(index)] == case["queue"]
                sequences[int(handle)] = int(sequence)
        cursor = 41
        native = []
        for call_index, call in enumerate(case["calls"]):
            reference = original(call, sequences, case["queue"], cursor, case["backend_error"])
            assert reference["original_full_entry_visited"]
            if reference["backend"] != call:
                failures.append({"case":case_index,"call":call_index,"field":"backend arrays","candidate":call,"original":reference["backend"]})
            cursor += sum(len(s["commands"]) for s in call["submits"])
            expected = None if case["status"] == "error" else [case["queue"][0] | (case["queue"][1] << 32),case["receipts"][call_index][2]]
            if reference["receipt"] != expected or reference["next_timeline"] != cursor:
                failures.append({"case":case_index,"call":call_index,"field":"receipt/timeline","original":reference,"expected_receipt":expected,"expected_timeline":cursor})
            if case["status"] == "error":
                selected = 0 if case["queue"][0] == 0 else case["queue"][1] + 1
                if reference["pending_commands"] != case["queues"][selected]["pending_commands"]:
                    failures.append({"case":case_index,"field":"failed pending commands","original":reference["pending_commands"],"candidate":case["queues"][selected]["pending_commands"]})
            compared += 1
            native.append(reference)
        assert case["next_timeline"] == cursor
        observations.append({"candidate":case,"original_calls":native})
    paths = [ROOT/"src/rg/executor.rs", ROOT/"src/rg/vulkan.rs", ROOT/"tests/chain.rs", ROOT/"examples/queue_graph_gpu.rs", Path(__file__).resolve(), ROOT/"scripts/native_support/queue_entry.py",ROOT/"scripts/native_support/pe_fixture.py",ROOT/"src/raw/device.rs",ROOT/"src/raw/submit.rs",ROOT/"src/raw/command_buffer.rs",ROOT/"src/raw/managed/lifecycle.rs",ROOT/"Cargo.toml",ROOT/"Cargo.lock"]
    report = {"status":"passed_known_queue_graph_scope" if not failures else "failed",
              "target_sha256":q.b.EXPECTED_SHA,"original_entry":"0x140c04e20",
              "original_entry_from_beginning":True,"submit_arrays_injected":False,
              "scenario_count":len(cases),"original_calls":compared,"mismatches":len(failures),"failures":failures,
              "observations":observations,
              "input_sha256":{str(p.relative_to(ROOT)).replace("\\","/"):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},
              "limits":["原侧只执行完整queue_submit，不执行整graph compile/record",
                        "外部同步输入GraphBatch映射到原prior槽，current为空；不证明原游戏prior生产策略",
                        "每次原调用独立CPU device fixture，公共allocator/CRT/Vulkan与错误转换外供",
                        "辅助queue是原CPU合同/software图验证，不伪造真实硬件aux",
                        "同一ExecutionChain切换queue域因原全局u64资源bucket冲突风险而明确拒绝",
                        "跨队列族独占资源没有release/acquire计划时拒绝，未恢复原游戏队列归属调度"]}
    destination = ROOT/"evidence/queue-graph-functional-validation.json"
    if destination.exists():
        previous = json.loads(destination.read_text(encoding="utf-8"))
        if previous.get("input_sha256") == report["input_sha256"]:
            # 输入未变时保留已经验收的实际 GPU/测试记录；不把原侧重放抹成仅 CPU 报告。
            for key in ["functional_implementation", "candidate_and_gpu_checks", "oracle_check", "gpu_validation", "validation_classes"]:
                if key in previous: report[key] = previous[key]
        elif "gpu_validation" in previous:
            report["previous_gpu_validation"] = {"status":"stale_after_input_change", "output":previous["gpu_validation"]}
    destination.write_text(json.dumps(report,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
    print(json.dumps({k:report[k] for k in ["status","scenario_count","original_calls","mismatches","failures"]}))
    if failures: raise SystemExit(1)


if __name__ == "__main__": main()
