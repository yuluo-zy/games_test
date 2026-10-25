from pathlib import Path
import importlib.util
import hashlib
import itertools
import json
import subprocess
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
spec = importlib.util.spec_from_file_location("record_r6", ROOT / "evidence/render-recovery/parallel-batch-04/record-submit/verify.py")
r6 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r6)
b = r6.base
CPU = b.CPU
REFS = r6.EXTRA + 0xE000
TRACE = {}

def hook(cpu, address, size, user):
    if not TRACE:
        return
    TRACE["visited"].add(hex(address))
    if address == 0x1402C1D40:
        TRACE["phase"] = "initial"
    elif address == 0x1402C3240:
        TRACE["phase"] = "pass"
    elif address == 0x1402C26A1:
        TRACE["phase"] = "export"
    elif address in (0x140BD65E0, 0x140BD6610):
        kind = int(address == 0x140BD6610)
        handle = cpu.reg_read(UC_X86_REG_RDX)
        resource = handle - (0xA000 if kind == 0 else 0xB000)
        TRACE["events"].append(f'barrier:{TRACE["phase"]}:{resource}:{cpu.reg_read(UC_X86_REG_R8) & 255}:{cpu.reg_read(UC_X86_REG_R9) & 255}')
    elif address == 0x140BEE030:
        command = cpu.reg_read(UC_X86_REG_RDX)
        handle = b.read(command + 0x48, "<Q")[0]
        TRACE["events"].append(f'emit:{TRACE["phase"]}:{handle}')
    elif address == 0x140BD6950:
        TRACE["events"].append(f'reset:{TRACE["phase"]}')
    elif address == 0x140BECFD0:
        # R5 treats tracker internals as an external fixture. Keep that boundary.
        resource = (cpu.reg_read(UC_X86_REG_RCX) - b.RECORDS) // 256
        TRACE["events"].append(f'track-barrier:{TRACE["phase"]}:{resource}')
    elif address == 0x140BD6A70:
        # Actual private helper executes; sequence mutation is read after return.
        payload = cpu.reg_read(UC_X86_REG_RCX)
        record = b.read(payload + 8, "<Q")[0]
        TRACE["events"].append(f'track-pass:{(record - b.RECORDS) // 256}')
    elif address == 0x140BEF4A0:
        command = cpu.reg_read(UC_X86_REG_RCX)
        b.write(command + 8, "<Q", 7)
        TRACE["create_pool_indices"].append(b.read(cpu.reg_read(UC_X86_REG_RSP) + 0x20, "<I")[0])
        TRACE["events"].append("create:100")
    elif address == 0x140BFC750:
        TRACE["pool_selectors"].append(cpu.reg_read(UC_X86_REG_RDX) & 0xFFFFFFFF)
    elif address == 0x140BED230:
        TRACE["begin_tags"].append(cpu.reg_read(UC_X86_REG_R8) & 0xFFFFFFFF)
        TRACE["events"].append("begin:100")
    elif address == r6.CALLBACK:
        index = cpu.reg_read(UC_X86_REG_RDX) - r6.CALLBACK_DATA - 1
        TRACE["events"].append(f"pass:{index}")
    elif address == 0x140BED3D0:
        TRACE["events"].append(f'end:{b.read(cpu.reg_read(UC_X86_REG_RCX) + 0x48, "<Q")[0]}')
    elif address == r6.END_COMMAND:
        TRACE["events"].append("end:last")
    elif address == 0x140C04E20:
        TRACE["events"].append("queue:2")

CPU.hook_add(UC_HOOK_CODE, hook)

def setup(case):
    # resources, passes, initial, reused, pattern, group, write, failure,
    # final, invalid-resource, invalid-kind, chain, end-error, pending
    n, passes, initial, reused, pattern, group, is_write, failure, final, invalid, validity, chain, end_error, pending = case
    r6.setup([0, passes, failure, final, 0])
    TRACE.clear()
    TRACE.update(events=[], visited=set(), phase="initial", begin_tags=[], pool_selectors=[], create_pool_indices=[])
    b.write(b.REGISTRY, "<QQQ", n, b.REUSE, n)
    b.write(b.REGISTRY + 0x18, "<QQQ", n, b.INITIAL, n)
    b.write(b.REGISTRY + 0x30, "<QQQ", n, b.BACKING, n)
    b.write(b.REGISTRY + 0x48, "<QQQ", n * passes, b.ACCESSES, n * passes)
    b.write(b.REGISTRY + 0x60, "<QQQ", passes, b.RANGES, passes)
    b.write(b.REGISTRY + 0x78, "<QQQ", n, b.LIFETIMES, n)
    b.write(b.COMPILED + 0x200, "<QQQ", n, b.PENDING, n if pending else 0)
    b.write(b.COMPILED + 0x20, "<QQ", b.EXPORTS, n)
    for resource in range(n):
        kind = resource % 2
        record = b.RECORDS + resource * 256
        payload = b.BACKING + resource * 64
        bad = validity if resource == invalid else 0
        b.write(b.REUSE + resource * 4, "<HB", resource, reused)
        b.write(b.INITIAL + resource, "<B", initial)
        b.write(payload, "<Q", kind * 2)
        b.write(payload + 16, "<Q", record)
        b.write(payload + 24, "<IHBB", 8 if bad == 3 else 7, 10 if bad == 1 else 9, kind ^ (bad == 2), 0)
        b.write(record, "<QII", 2, 1, 7)
        b.write(record + 0x18, "<I", 37)
        b.write(record + 0x30, "<Q", 0xB000 + resource)
        b.write(record + 0x40, "<Q", 0xA000 + resource)
        b.write(b.LIFETIMES + resource * 24 + 16, "<II", resource if passes else 0xFFFFFFFF, (passes - 1) * n + resource if passes else 0xFFFFFFFF)
        b.write(b.PENDING + resource * 4, "<I", resource)
        b.write(b.EXPORTS + resource * 48, "<II", kind, resource)
        b.write(b.EXPORTS + resource * 48 + 0x10, "<I", 91)
        b.write(b.EXPORTS + resource * 48 + 0x18, "<I", resource)
        b.write(b.EXPORTS + resource * 48 + 0x28, "<B", 11)
    for p in range(passes):
        b.write(b.RANGES + p * 8, "<II", p * n, (p + 1) * n)
        refs = REFS + p * 0x100
        b.write(r6.PASSES + p * 64, "<QQQ", n, refs, n)
        for resource in range(n):
            code = 4 if pattern == 0 or p % 2 == 0 else 23
            if is_write:
                b.STATE["write_codes"].add(code)
            index = p * n + resource
            b.write(b.ACCESSES + index * 20, "<IIIIB", resource, (p - 1) * n + resource if p else 0xFFFFFFFF, (p + 1) * n + resource if p + 1 < passes else 0xFFFFFFFF, group, code)
            b.write(refs + resource * 12, "<III", resource, 0, 0)
    if failure == 998:
        b.write(r6.PASSES + 0x28, "<Q", 0)
    return 0x1402C1D40

def execute(case):
    entry = setup(case)
    try:
        CPU.emu_start(entry, b.STOP, count=300000)
    except Exception as error:
        raise RuntimeError(f"case={case}, RIP={CPU.reg_read(UC_X86_REG_RIP):x}") from error
    panic = r6.STATE["panic"] or b.STATE["panic"] != "ok"
    n, passes = case[:2]
    length = b.read(b.COMMAND_VEC + 16, "<Q")[0]
    states = [b.read(b.COMMANDS + i * 88, "<I")[0] for i in range(length)]
    consumed = r6.STATE["consumed"]
    sequence = [b.read(b.RECORDS + i * 256, "<Q")[0] for i in range(n)]
    events = TRACE["events"].copy()
    visited = TRACE["visited"].copy()
    constructors = b.STATE["constructors"].copy()
    callback_presence = [b.read(r6.PASSES + i * 64 + 0x28, "<Q")[0] != 0 for i in range(passes)]
    pending = b.read(b.COMPILED + 0x210, "<Q")[0]
    receipt = "none"
    command_hash = None
    if case[11] and not panic:
        commands = bytes(CPU.mem_read(b.COMMANDS, length * 88))
        command_hash = hashlib.sha256(commands).hexdigest()
        r6.setup([1, length, 1, case[12], 0])
        CPU.mem_write(b.COMMANDS, commands)
        CPU.emu_start(0x140BEBDF0, b.STOP, count=300000)
        events = TRACE["events"].copy()
        visited.update(TRACE["visited"])
        panic = r6.STATE["panic"] or b.STATE["panic"] != "ok"
        states = [b.read(b.COMMANDS + i * 88, "<I")[0] for i in range(length)]
        if "submit_payload" in r6.STATE:
            receipt = "0:41"
    native = f'{int(panic)}|{",".join(events)}|{pending}|{",".join(map(str, states))}|{consumed}|{",".join(map(str, sequence))}|{receipt}'
    return native, {"case":case,"native":native,"events":events,"visited":sorted(visited),"barrier_constructors":constructors,"callback_presence_after_record":callback_presence,"command_transfer_sha256":command_hash,"begin_tags":TRACE["begin_tags"].copy(),"pool_selectors":TRACE["pool_selectors"].copy(),"create_pool_indices":TRACE["create_pool_indices"].copy()}

def cases():
    result = []
    for n, passes, reused, pattern, group, wr, final, chain in itertools.product([1, 2, 4], [1, 2, 4], [0, 1], [0, 1], [0, 7], [0, 1], [0, 1], [0, 1]):
        result.append([n, passes, 0 if reused else 3, reused, pattern, group, wr, 999, final, 999, 0, chain, 0, 1])
    for n, passes, failure, final in itertools.product([1, 2, 4], [1, 2, 4], [0, 1, 3], [0, 1]):
        if failure < passes:
            result.append([n, passes, 3, 0, 1, 0, 1, failure, final, 999, 0, 0, 0, 1])
    for n, invalid, validity, pending, reused in itertools.product([1, 2, 4], [0, 1, 3], [1, 2, 3], [0, 1], [0, 1]):
        if invalid < n:
            result.append([n, 2, 0 if reused else 3, reused, 1, 0, 1, 999, 1, invalid, validity, 0, 0, pending])
    for n, passes, final in itertools.product([1, 2, 4], [1, 2, 4], [0, 1]):
        result.append([n, passes, 3, 0, 1, 0, 1, 999, final, 999, 0, 1, 1, 1])
    for n, initial, reused, final, chain in itertools.product([1, 2, 4], [0, 255], [0, 1], [0, 1], [0, 1]):
        result.append([n, 0, initial, reused, 0, 0, 0, 999, final, 999, 0, chain, 0, 1])
    for n, passes, final in itertools.product([1, 2, 4], [1, 2], [0, 1]):
        result.append([n, passes, 3, 0, 1, 0, 1, 998, final, 999, 0, 0, 0, 1])
        result.append([n, passes, 3, 1, 1, 0, 1, 999, final, 999, 0, 0, 0, 1])
    return result

def main():
    fixtures = cases()
    observations = [execute(case)[1] for case in fixtures]
    source = ROOT / "reconstruction/render/work-packages/record-resources-r7"
    subprocess.run(["cargo", "build", "--offline", "--locked", "--release", "--manifest-path", str(source / "Cargo.toml")], check=True)
    actual = subprocess.run([str(source / "target/release/tiny-glade-record-resources-r7.exe")], input="".join(" ".join(map(str,c)) + "\n" for c in fixtures), text=True, capture_output=True, check=True).stdout.splitlines()
    failures = [{"case":o["case"],"native":o["native"],"candidate":a} for o,a in zip(observations, actual) if o["native"] != a]
    assert all(tag == 0 for o in observations for tag in o["begin_tags"])
    assert all(tag == 0 for o in observations for tag in o["pool_selectors"])
    report = {"passed":not failures and len(actual) == len(fixtures),"cases":len(fixtures),"failure_count":len(failures),"failures":failures[:20],"original_sha256":b.EXPECTED_SHA,"nonempty_resource_cases":len(fixtures),"nonempty_pass_access_cases":sum(c[1] != 0 for c in fixtures),"no_access_export_fallback_cases":sum(c[1] == 0 for c in fixtures),"record_to_submit_cases":sum(o["command_transfer_sha256"] is not None for o in observations),"begin_tags_observed":sorted({tag for o in observations for tag in o["begin_tags"]}),"pool_selectors_observed":sorted({tag for o in observations for tag in o["pool_selectors"]}),"create_pool_indices_fixture":sorted({tag for o in observations for tag in o["create_pool_indices"]}),"fixture_command_sequence":7,"sequence_is_original_initialization":False,"original_entries":["0x1402c1d40","0x1402c3240","0x1402c3d00","0x140bd6a70","0x140bebdf0"],"observations":observations,"old_108_cases_recounted":False}
    (OUT / "native-validation.json").write_text(json.dumps(report,indent=2),encoding="utf8")
    print(json.dumps({k:report[k] for k in ["passed","cases","failure_count","failures"]}))
    assert report["passed"]

if __name__ == "__main__":
    main()
