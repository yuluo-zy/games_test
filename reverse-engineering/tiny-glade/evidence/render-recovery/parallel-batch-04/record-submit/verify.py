from pathlib import Path
import hashlib
import itertools
import json
import struct
import subprocess
import sys
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[3]
sys.path.insert(0, str(ROOT / "evidence/render-recovery/parallel-batch-03/barrier-record"))
import verify as base

CPU = base.CPU
EXTRA = 0x50000000
CPU.mem_map(EXTRA, 0x200000)
PASSES = EXTRA
CALLBACK_DATA = EXTRA + 0x4000
VTABLE = EXTRA + 0x5000
EXECUTOR = EXTRA + 0x6000
FN_TABLE = EXTRA + 0x8000
QUEUE = EXTRA + 0x9000
SUBMITS = EXTRA + 0xA000
RECEIPT = EXTRA + 0xB000
ALLOCATIONS = EXTRA + 0x10000
CALLBACK = base.STOP + 0x100
END_COMMAND = base.STOP + 0x200
VK_SUBMIT = base.STOP + 0x300
STATE = {}

def read(address, fmt):
    return base.read(address, fmt)

def write(address, fmt, *values):
    base.write(address, fmt, *values)

def returns(value=None, second=None):
    base.ret(value)
    if second is not None:
        CPU.reg_write(UC_X86_REG_RDX, second)

def hook(cpu, address, size, user):
    STATE["visited"].add(hex(address))
    if address == STATE["cut"]:
        STATE["cut_reached"] = True
        cpu.emu_stop()
    elif address in (0x1428D9760, 0x1428D9430, 0x1428D9310, 0x1428D9390, 0x1428D9518, 0x1428D9850, 0x1428D9840):
        STATE["panic"] = True
        STATE["panic_address"] = hex(address)
        cpu.emu_stop()
    elif address == 0x140613C10:
        length = cpu.reg_read(UC_X86_REG_RCX)
        pointer = STATE["allocation"]
        STATE["allocation"] += ((length + 15) & ~15) + 16
        assert STATE["allocation"] < EXTRA + 0x200000
        cpu.mem_write(pointer, bytes(length))
        returns(pointer)
    elif address in (0x142923C00, 0x142923C10):
        dest = cpu.reg_read(UC_X86_REG_RCX)
        source = cpu.reg_read(UC_X86_REG_RDX)
        count = cpu.reg_read(UC_X86_REG_R8)
        cpu.mem_write(dest, bytes(cpu.mem_read(source, count)))
        returns(dest)
    elif address in (0x140613C20, 0x1405D12D0, 0x140BE2B10, 0x1402CA150, 0x140BE2880, 0x140BE2AD0, 0x1418C1380, 0x140BDF860, 0x140BE14B0, 0x140C81C70, 0x140C81E10, 0x1418C0A70, 0x140BF0CC0, 0x140C84010):
        returns()
    elif address in (0x140BFC750, 0x140BD4B40, 0x140C16A00):
        returns(0)
    elif address in (0x140D635E0, 0x140D63910):
        returns(0, 0)
    elif address == 0x140BE9C70:
        returns(0, 0)
    elif address == 0x140BEF4A0:
        output = cpu.reg_read(UC_X86_REG_RCX)
        cpu.mem_write(output, bytes(88))
        write(output, "<I", 1)
        write(output + 0x48, "<Q", 100)
        STATE["events"].append("create:100")
        returns(output)
    elif address == 0x140BED230:
        STATE["events"].append("begin:100")
        returns()
    elif address == 0x140BD6650:
        cpu.mem_write(cpu.reg_read(UC_X86_REG_RCX), bytes(144))
        returns()
    elif address == CALLBACK:
        index = cpu.reg_read(UC_X86_REG_RDX) - CALLBACK_DATA - 1
        STATE["events"].append(f"pass:{index}")
        STATE["consumed"] += 1
        write(cpu.reg_read(UC_X86_REG_RCX), "<Q", 0 if index == STATE["fail_at"] else 11)
        returns()
    elif address == 0x140BED3D0:
        command = cpu.reg_read(UC_X86_REG_RCX)
        handle = read(command + 0x48, "<Q")[0]
        STATE["events"].append(f"end:{handle}")
        write(command, "<Q", 2)
        returns()
    elif address == 0x140BF09B0:
        if cpu.reg_read(UC_X86_REG_RIP) == 0x1402C26DF:
            STATE["events"].append("exports")
    elif address == END_COMMAND:
        STATE["events"].append("end:last")
        STATE["end_handle"] = cpu.reg_read(UC_X86_REG_RCX)
        returns(int(STATE["end_error"]))
    elif address == 0x140C04E20 and STATE["mode"] == 1:
        payload = read(cpu.reg_read(UC_X86_REG_RSP) + 0x28, "<Q")[0]
        capacity, pointer, count = read(payload + 0x28, "<QQQ")
        STATE["events"].append(f"queue:{count}")
        STATE["submit_payload"] = {"capacity":capacity,"count":count,"states":[read(pointer + index * 88, "<I")[0] for index in range(count)],"handles":[read(pointer + index * 88 + 0x48, "<Q")[0] for index in range(count)],"queue_kind":cpu.reg_read(UC_X86_REG_R8),"opaque_d8_hex":bytes(cpu.mem_read(payload, 216)).hex()}
        write(cpu.reg_read(UC_X86_REG_RCX), "<QQ", 0, 41)
        returns(cpu.reg_read(UC_X86_REG_RCX))
    elif address == VK_SUBMIT:
        count = cpu.reg_read(UC_X86_REG_RDX) & 0xFFFFFFFF
        STATE["events"].append(f"vk-submit:{count}")
        STATE["vk_arguments"] = {"queue":cpu.reg_read(UC_X86_REG_RCX),"count":count,"submits":cpu.reg_read(UC_X86_REG_R8),"fence":cpu.reg_read(UC_X86_REG_R9)}
        returns(int(STATE["backend_error"]))
    elif address == 0x140BF0860:
        STATE["events"].append("convert-error")
        write(cpu.reg_read(UC_X86_REG_RCX), "<Q", 11 if STATE["converted_success"] else 9)
        returns()
    elif address == 0x140C0630D:
        STATE["events"].append("post-submit")

CPU.hook_add(UC_HOOK_CODE, hook)

def setup(case):
    base.setup([0, 0, 0, 0, 256, 0, 0, 4, 0, 0, 1, 0])
    CPU.mem_write(EXTRA, bytes(0x200000))
    base.STATE["cut"] = base.STOP
    STATE.clear()
    STATE.update(mode=case[0], events=[], panic=False, panic_address=None, consumed=0, cut=base.STOP, cut_reached=False, visited=set(), allocation=ALLOCATIONS, fail_at=None, end_error=False, backend_error=False, converted_success=False)
    write(0x14362C228, "<Q", EXTRA + 0xF000)
    write(0x14362C220, "<Q", EXTRA + 0xF008)
    if case[0] == 0:
        _, count, failure, final, _ = case
        STATE["fail_at"] = failure if failure != 999 else None
        write(base.COMPILED + 0x200, "<QQQ", 0, base.PENDING, 0)
        write(base.COMPILED + 0x90, "<QQQ", count, PASSES, count)
        write(base.REGISTRY + 0x60, "<QQQ", count, base.RANGES, count)
        CPU.mem_write(base.RANGES, bytes(count * 8))
        write(base.COMPILED + 0x20, "<QQ", base.EXPORTS, 0)
        write(base.BATCH, "<QQB", 0, count, final)
        write(base.COMMAND_VEC, "<QQQ", 16, base.COMMANDS, 1)
        write(base.COMMANDS, "<I", 1)
        write(base.COMMANDS + 0x48, "<Q", 99)
        write(VTABLE + 0x18, "<Q", CALLBACK)
        for index in range(count):
            write(PASSES + index * 64 + 0x28, "<QQ", CALLBACK_DATA + 1 + index, VTABLE)
        pointer = base.STACK + 0x10000
        write(pointer, "<Q", base.STOP)
        write(pointer + 0x28, "<QQQQ", 0, base.COMMAND_VEC, EXTRA + 0xC000, EXTRA + 0xD000)
        CPU.reg_write(UC_X86_REG_RSP, pointer)
        CPU.reg_write(UC_X86_REG_RCX, base.COMPILED)
        CPU.reg_write(UC_X86_REG_RDX, base.BATCH)
        CPU.reg_write(UC_X86_REG_R8, base.DEVICE)
        CPU.reg_write(UC_X86_REG_R9, 0)
        return 0x1402C1D40
    if case[0] == 1:
        _, count, last_state, error, wait = case
        STATE["end_error"] = bool(error)
        write(EXECUTOR + 0x18, "<QQ", wait, 55)
        write(EXECUTOR + 0x40, "<QQQ", count, base.COMMANDS, count)
        write(EXECUTOR + 0x58, "<Q", 0x8000000000000000)
        write(EXECUTOR + 0x178, "<Q", FN_TABLE)
        write(FN_TABLE + 0x2C8, "<Q", END_COMMAND)
        for index in range(count):
            write(base.COMMANDS + index * 88, "<I", last_state if index + 1 == count else 2)
            write(base.COMMANDS + index * 88 + 0x48, "<Q", 99 + index)
        pointer = base.STACK + 0x10000
        write(pointer, "<Q", base.STOP)
        CPU.reg_write(UC_X86_REG_RSP, pointer)
        CPU.reg_write(UC_X86_REG_RCX, EXECUTOR)
        CPU.reg_write(UC_X86_REG_RDX, base.DEVICE)
        return 0x140BEBDF0
    _, count, error, converted, _ = case
    STATE["backend_error"] = bool(error)
    STATE["converted_success"] = bool(converted)
    frame = base.STACK + 0x10000
    CPU.reg_write(UC_X86_REG_RSP, frame - 0x80)
    CPU.reg_write(UC_X86_REG_RBP, frame)
    CPU.reg_write(UC_X86_REG_RBX, base.DEVICE)
    CPU.reg_write(UC_X86_REG_R14, QUEUE)
    write(base.DEVICE + 0x88, "<Q", VK_SUBMIT)
    write(QUEUE + 0x78, "<Q", 0xABCD)
    write(frame + 0x258, "<QI", SUBMITS, count)
    write(frame + 0x418, "<Q", base.DEVICE)
    write(frame + 0x3D8, "<Q", 42)
    write(frame + 0x410, "<Q", 0)
    write(frame + 0x238, "<Q", RECEIPT)
    STATE["cut"] = 0x140C0634F
    return 0x140C06151

def execute(case, command_bytes=None):
    entry = setup(case)
    if command_bytes is not None:
        CPU.mem_write(base.COMMANDS, command_bytes)
    try:
        CPU.emu_start(entry, base.STOP, count=200000)
    except Exception as error:
        raise RuntimeError(f"case={case}, RIP={CPU.reg_read(UC_X86_REG_RIP):x}") from error
    if base.STATE["panic"] != "ok":
        STATE["panic"] = True
        STATE["panic_address"] = hex(CPU.reg_read(UC_X86_REG_RIP))
    mode, count = case[:2]
    if mode == 0:
        command_length = read(base.COMMAND_VEC + 16, "<Q")[0]
        states = [read(base.COMMANDS + index * 88, "<I")[0] for index in range(command_length)]
        receipt = None
        pending = read(base.COMPILED + 0x210, "<Q")[0]
        consumed = sum(read(PASSES + index * 64 + 0x28, "<Q")[0] == 0 for index in range(count))
    elif mode == 1:
        states = [read(base.COMMANDS + index * 88, "<I")[0] for index in range(count)]
        receipt = read(EXECUTOR + 0x108, "<QQ") if "submit_payload" in STATE else None
        pending = read(EXECUTOR + 0x50, "<Q")[0]
        consumed = 0
        assert read(EXECUTOR + 0x40, "<QQQ") == (0, 8, 0)
        assert read(EXECUTOR + 0x18, "<Q")[0] == 0
        assert read(EXECUTOR + 0x58, "<Q")[0] == 0x8000000000000000
    else:
        states = []
        receipt = read(RECEIPT, "<QQ") if STATE["cut_reached"] else None
        pending = 0
        consumed = 0
        assert read(QUEUE + 0x70, "<B")[0] == 0
        assert read(base.DEVICE + 0x800, "<Q")[0] == 0
    output = f'{int(STATE["panic"])}|{",".join(STATE["events"])}|{pending}|{",".join(map(str, states))}|{consumed}|{str(receipt[0]) + ":" + str(receipt[1]) if receipt else "none"}'
    observation = {"case":case,"entry":hex(entry),"cut":hex(STATE["cut"]),"native":output,"events":STATE["events"].copy(),"panic_address":STATE["panic_address"],"visited":sorted(STATE["visited"])}
    for field in ["submit_payload", "vk_arguments", "end_handle"]:
        if field in STATE:
            observation[field] = STATE[field]
    return output, observation

def chain(case):
    _, count, error, final, _ = case
    _, recorded = execute([0, count, 999, final, 0])
    command_bytes = bytes(CPU.mem_read(base.COMMANDS, 176))
    _, submitted = execute([1, 2, 1, error, 0], command_bytes)
    events = recorded["events"] + submitted["events"]
    receipt = read(EXECUTOR + 0x108, "<QQ") if "submit_payload" in STATE else None
    states = [read(base.COMMANDS + index * 88, "<I")[0] for index in range(2)]
    native = f'{int(STATE["panic"])}|{",".join(events)}|0|{",".join(map(str, states))}|{count}|{str(receipt[0]) + ":" + str(receipt[1]) if receipt else "none"}'
    observation = {"case":case,"native":native,"events":events,"record":recorded,"submit":submitted,"transfer":"same recorded 176 semantic/padding bytes passed to submit fixture; both entries original","command_sha256":hashlib.sha256(command_bytes).hexdigest()}
    return native, observation

def main():
    cases = [[0, count, failure, final, 0] for count in [0, 1, 2, 4, 8] for failure in [999, 0, 1, 3] for final in [0, 1] if failure == 999 or failure < count]
    cases += [[1, count, state, error, wait] for count, state, error, wait in itertools.product([0, 1, 2, 5], [0, 1, 2], [0, 1], [0, 1])]
    cases += [[2, count, error, converted, 0] for count, error, converted in itertools.product([0, 1, 3, 255, 65535], [0, 1], [0, 1])]
    cases += [[3, count, error, final, 0] for count, error, final in itertools.product([0, 1, 4], [0, 1], [0, 1])]
    expected, observations = [], []
    for case in cases:
        native, observation = chain(case) if case[0] == 3 else execute(case)
        expected.append(native)
        observations.append(observation)
    source = ROOT / "reconstruction/render/work-packages/record-submit-r6"
    subprocess.run(["cargo", "build", "--offline", "--release", "--manifest-path", str(source / "Cargo.toml")], check=True)
    runner = source / "target/release/tiny-glade-record-submit-r6.exe"
    actual = subprocess.run([str(runner)], input="".join(" ".join(map(str, case)) + "\n" for case in cases), text=True, capture_output=True, check=True).stdout.splitlines()
    failures = [{"case":case,"native":native,"candidate":candidate} for case, native, candidate in zip(cases, expected, actual) if native != candidate]
    report = {"passed":not failures and len(actual) == len(expected),"cases":len(cases),"by_mode":{str(mode):sum(case[0] == mode for case in cases) for mode in range(4)},"failure_count":len(failures),"failures":failures[:20],"sha256":base.EXPECTED_SHA,"observations":observations,"limits":["record original sequential path with empty resource-access lists; public command/allocator/callback boundaries stubbed","submit original entry delegates queue_submit to structured capture fixture","queue Vulkan-call tail starts with injected prebuilt submit/frame; earlier assembly not validated","chain copies exact original-recorded command bytes to original submit entry but queue assembly remains fixture","no Vulkan calls or live GPU execution","panic stops without unwinding"]}
    (OUT / "native-validation.json").write_text(json.dumps(report, indent=2), encoding="utf8")
    for label, start, end in [("record",0x1402C1D40,0x1402C2F82),("pass-record",0x1402C3240,0x1402C3A9A),("submit",0x140BEBDF0,0x140BEC2C9),("queue",0x140C04E20,0x140C0687D)]:
        text = subprocess.run(["llvm-objdump","-d",f"--start-address={hex(start)}",f"--stop-address={hex(end)}",str(base.TARGET)],text=True,capture_output=True,check=True).stdout
        (OUT / (label + ".objdump.txt")).write_text(text,encoding="utf8")
    print(json.dumps({"passed":report["passed"],"cases":len(cases),"failure_count":len(failures),"failures":failures[:3]}))
    assert report["passed"]

if __name__ == "__main__":
    main()
