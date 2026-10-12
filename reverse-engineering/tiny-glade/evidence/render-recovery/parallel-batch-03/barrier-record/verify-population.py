import json
import struct
import verify as oracle
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *

CPU = oracle.CPU
ROOT = oracle.ROOT
OUT = oracle.OUT
MEMORY = oracle.MEMORY
REGISTRY = oracle.REGISTRY
GRAPH = MEMORY + 0xF000
ORIGINS = MEMORY + 0x10000
EVENTS = MEMORY + 0x11000
IMAGE_USAGE = MEMORY + 0x12000
BUFFER_USAGE = MEMORY + 0x13000
TLS = MEMORY + 0x14000
POP = {}

def hook(cpu, address, size, user):
    POP["visited"].add(hex(address))
    if address == 0x140D4D740:
        oracle.ret(TLS)
    elif address == 0x142923C20:
        cpu.mem_write(cpu.reg_read(UC_X86_REG_RCX), bytes([cpu.reg_read(UC_X86_REG_RDX) & 255]) * cpu.reg_read(UC_X86_REG_R8))
        oracle.ret(cpu.reg_read(UC_X86_REG_RCX))
    elif address in (0x1418BCC30, 0x1418BCE70):
        oracle.ret(0)
    elif address in (0x1417E7F20, 0x1417E7C30):
        target = cpu.reg_read(UC_X86_REG_RCX)
        cpu.mem_write(target, bytes(64))
        oracle.write(target + (0x20 if address == 0x1417E7F20 else 0x10), "<HB", 0 if POP["local_reuse"] else 0xFFFF, 0 if POP["local_reuse"] else 2)
        POP["local_map_remove"] += 1
        oracle.ret()
    elif address in (0x140BE6B90, 0x140BE71E0):
        target = cpu.reg_read(UC_X86_REG_RCX)
        cpu.mem_write(target, bytes(64))
        if POP["pool_hit"]:
            oracle.write(target + 0x10, "<Q", oracle.RECORDS)
            oracle.write(target + 0x18, "<IHBB", 7, 9, int(address == 0x140BE71E0), 0)
        else:
            oracle.write(target + (0x36 if address == 0x140BE6B90 else 0x26), "<B", 7 if address == 0x140BE6B90 else 5)
        POP["pool_acquire"] += 1
        oracle.ret()
    elif address in (0x140C07950, 0x140C03460):
        target = cpu.reg_read(UC_X86_REG_RCX)
        cpu.mem_write(target, bytes(72))
        oracle.write(target, "<Q", POP["error_tag"] if POP["error_tag"] is not None else 11)
        oracle.write(target + 8 + 0x10, "<Q", oracle.RECORDS)
        oracle.write(target + 8 + 0x18, "<IHBB", 7, 9, int(address == 0x140C03460), 0)
        POP["create"] += 1
        oracle.ret()
    elif address in (0x140BFE650, 0x140BEEC30):
        POP["names"] += 1
        oracle.ret()
    elif address in (0x140BDFC90, 0x1407CEA70, 0x14014A360, 0x140BDEBF0, 0x140613C20):
        POP["drops"].append(hex(address))
        oracle.ret()

CPU.hook_add(UC_HOOK_CODE, hook)

def execute(origins, events, pool_hit=False, error_tag=None, local_reuse=False):
    base = [0, 0, 0, 0, 256, 0, 0, 4, 0, 0, 1, 0]
    oracle.setup(base)
    oracle.STATE["cut"] = oracle.STOP
    POP.clear()
    POP.update(visited=set(), local_map_remove=0, pool_acquire=0, create=0, names=0, drops=[], pool_hit=pool_hit, error_tag=error_tag, local_reuse=local_reuse)
    count = len(origins)
    oracle.write(REGISTRY, "<QQQ", count + 8, oracle.REUSE, 0)
    oracle.write(REGISTRY + 0x18, "<QQQ", count + 8, oracle.INITIAL, 0)
    oracle.write(REGISTRY + 0x30, "<QQQ", count + 8, oracle.BACKING, 0)
    oracle.write(REGISTRY + 0x90, "<QQQ", count, IMAGE_USAGE, count)
    oracle.write(REGISTRY + 0xA8, "<QQQ", count, BUFFER_USAGE, count)
    oracle.write(REGISTRY + 0xC0, "<QQQ", len(events), EVENTS, len(events))
    oracle.write(GRAPH, "<QQQ", count, ORIGINS, count)
    oracle.write(TLS, "<QQQ", 1, 71, 83)
    for index, (tag, kind, initial, usage) in enumerate(origins):
        position = ORIGINS + index * 72
        oracle.write(position, "<Q", tag)
        if tag == 0:
            oracle.write(position + 0x36, "<B", 7 if kind == 1 else 0)
        else:
            oracle.write(position + 8, "<BB", kind, initial)
            oracle.write(position + 0x20, "<Q", oracle.RECORDS)
            oracle.write(position + 0x28, "<IHBB", 7, 9, kind, 0)
        oracle.write(IMAGE_USAGE + index * 4, "<I", usage)
        oracle.write(BUFFER_USAGE + index * 4, "<I", usage)
    for index, event in enumerate(events):
        oracle.write(EVENTS + index * 4, "<I", event)
    stack_pointer = oracle.STACK + 0x10000
    oracle.write(stack_pointer, "<Q", oracle.STOP)
    CPU.reg_write(UC_X86_REG_RSP, stack_pointer)
    CPU.reg_write(UC_X86_REG_RCX, REGISTRY)
    CPU.reg_write(UC_X86_REG_RDX, GRAPH)
    CPU.reg_write(UC_X86_REG_R8, MEMORY + 0x15000)
    CPU.reg_write(UC_X86_REG_R9, oracle.DEVICE)
    try:
        CPU.emu_start(0x140BE7BA0, oracle.STOP, count=60000)
    except Exception as error:
        raise RuntimeError(f"RIP={CPU.reg_read(UC_X86_REG_RIP):x}, origins={origins}, events={events}") from error
    physical_count = oracle.read(REGISTRY + 0x40, "<Q")[0]
    return {
        "input": {"origins": origins, "events": events, "pool_hit": pool_hit, "error_tag": error_tag, "synthetic_local_reuse": local_reuse},
        "panic": "result-tag" if oracle.STATE["panic"] == "command-empty" else oracle.STATE["panic"],
        "returned_failure": CPU.reg_read(UC_X86_REG_RAX) & 255 if oracle.STATE["panic"] == "ok" else None,
        "origin_length": oracle.read(GRAPH + 0x10, "<Q")[0],
        "origin_tags": [oracle.read(ORIGINS + index * 72, "<Q")[0] for index in range(count)],
        "reuse": [list(oracle.read(oracle.REUSE + index * 4, "<HB")) for index in range(count)],
        "initial": list(bytes(CPU.mem_read(oracle.INITIAL, count))),
        "physical_count": physical_count,
        "backing_tags": [oracle.read(oracle.BACKING + index * 64, "<Q")[0] for index in range(physical_count)],
        "boundaries": {key: POP[key] for key in ["local_map_remove", "pool_acquire", "create", "names", "drops"]},
        "visited": sorted(POP["visited"]),
    }

def main():
    observations = []
    for tag in [0, 1, 2]:
        for kind in [0, 1]:
            for initial in [0, 3, 255]:
                for usage in [0, 1]:
                    for hit in [False, True]:
                        observation = execute([(tag, kind, initial, usage)], [0], hit)
                        assert observation["panic"] == "ok", observation
                        assert observation["returned_failure"] == 0
                        assert observation["origin_length"] == 0
                        assert observation["origin_tags"] == [3]
                        assert observation["reuse"] == [[0, 0]]
                        assert observation["physical_count"] == 1
                        assert observation["initial"] == [initial if tag else 0]
                        assert observation["backing_tags"] == [5 if tag == 0 and usage == 0 else kind * 2 + int(tag == 1)]
                        observations.append(observation)
    for kind in [0, 1]:
        for tag in range(11):
            observation = execute([(1, kind, 3, 1), (0, kind, 0, 1), (1, kind, 4, 1)], [0, 1, 2], False, tag)
            assert observation["physical_count"] == 1
            assert observation["origin_tags"] == [3, 3, 1]
            assert observation["origin_length"] == 3
            assert observation["reuse"] == [[0, 0], [65535, 0], [65535, 0]]
            assert observation["returned_failure"] == (1 if tag <= 8 else None)
            assert observation["panic"] == ("ok" if tag <= 8 else "result-tag")
            observations.append(observation)
    release = execute([(1, 0, 3, 1), (1, 1, 4, 1)], [0x10000, 1])
    assert release["reuse"] == [[65535, 0], [0, 0]]
    assert release["origin_tags"] == [1, 3]
    observations.append(release)
    for kind in [0, 1]:
        alias = execute([(1, kind, 3, 1), (0, kind, 0, 1)], [0, 1], local_reuse=True)
        assert alias["reuse"] == [[0, 0], [0, 1]]
        assert alias["initial"] == [3, 0]
        assert alias["physical_count"] == 1
        assert alias["boundaries"]["pool_acquire"] == 0
        observations.append(alias)
    report = {"passed": True, "cases": len(observations), "sha256": oracle.EXPECTED_SHA, "oracle": "original entry 0x140be7ba0 to original return or captured panic", "observations": observations, "limits": ["std hash/lookup replaced with empty-map fixtures, not reimplemented", "Two synthetic local-reuse Some fixtures test consumers only, not actual map insertion or reachability", "Pool public fixture supports hit/miss; create/name/device boundary captured", "Drops counted, no destructor execution", "Initial vector allocator excluded by preallocation", "No local reuse-map insertion or whole-program producer proof"]}
    (OUT / "population-validation.json").write_text(json.dumps(report, indent=2), encoding="utf8")
    print(json.dumps({"passed": True, "cases": len(observations)}))

if __name__ == "__main__":
    main()
