"""对照 Rust 命令输出和原机器码入口；不使用 Python 行为镜像代替 Rust。"""
import hashlib
import json
import re
import struct
import subprocess
from pathlib import Path
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *

ROOT = Path(__file__).resolve().parents[1]
ORIGINAL = Path("D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe")
EXPECTED = "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
raw = ORIGINAL.read_bytes()
assert hashlib.sha256(raw).hexdigest() == EXPECTED
pe = struct.unpack_from("<I", raw, 60)[0]
opt = pe + 24
base = struct.unpack_from("<Q", raw, opt + 24)[0]
size = struct.unpack_from("<I", raw, opt + 56)[0]
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, (size + 4095) & ~4095)
for i in range(struct.unpack_from("<H", raw, pe + 6)[0]):
    s = opt + struct.unpack_from("<H", raw, pe + 20)[0] + i * 40
    _, va, length, off = struct.unpack_from("<IIII", raw, s + 8)
    cpu.mem_write(base + va, raw[off:off + length])
STACK, STOP, MEM = 0x20000000, 0x30000000, 0x40000000
for address, length in [(STACK, 0x20000), (STOP, 0x10000), (MEM, 0x20000)]:
    cpu.mem_map(address, length)
API, CB, DEVICE, INNER = MEM, MEM+0x1000, MEM+0x2000, MEM+0x4000
HANDLE1, HANDLE2, OBJECT1, OBJECT2 = MEM+0x6000, MEM+0x6100, MEM+0x7000, MEM+0x7100
PIPELINE, BYTES, REGIONS, RANGE = MEM+0x8000, MEM+0x8100, MEM+0x8200, MEM+0x8300

def put(a, fmt, *values):
    cpu.mem_write(a, struct.pack(fmt, *values))

def get(a, fmt):
    return struct.unpack(fmt, bytes(cpu.mem_read(a, struct.calcsize(fmt))))

def reg(r):
    return cpu.reg_read(r)

def ret(v=None):
    sp = reg(UC_X86_REG_RSP)
    cpu.reg_write(UC_X86_REG_RIP, get(sp, "<Q")[0])
    cpu.reg_write(UC_X86_REG_RSP, sp+8)
    if v is not None:
        cpu.reg_write(UC_X86_REG_RAX, v)

put(API, "<QQ", CB, DEVICE)
put(CB+0x48, "<Q", 100)
put(CB+0x56, "<B", 0)
put(DEVICE+0x648, "<Q", INNER)
put(INNER+0xa50, "<H", 7)
put(DEVICE+0xad9, "<B", 1)
for h, o, token in [(HANDLE1, OBJECT1, 20), (HANDLE2, OBJECT2, 21)]:
    put(h+8, "<Q", o)
    put(h+0x10, "<IHB", 12, 7, 1)
    put(o+0xc, "<I", 12)
    put(o+0x20, "<Q", token)
    put(o+0x30, "<Q", token)
calls, panics, resolver_calls = [], [], []
slots = {0x428:"end", 0x2e0:"viewport", 0x2e8:"scissor", 0x320:"stencil", 0x2f0:"line_width", 0x338:"vertex", 0x330:"index", 0x410:"push", 0x370:"copy", 0x3a0:"fill", 0x3a8:"clear"}
hooks = {}
for i, (offset, name) in enumerate(slots.items()):
    address = STOP+0x1000+i*16
    put(DEVICE+offset, "<Q", address)
    hooks[address] = name

def hook(uc, address, instruction_size, data):
    if address == 0x1418aed30:
        # 原 managed get/track 外供边界；直接返回已配置对象，不声称恢复该入口。
        h = reg(UC_X86_REG_RDX)
        resolver_calls.append(h)
        ret(get(h+8, "<Q")[0])
    elif address in (0x1428d95d4, 0x1428d9310, 0x1428ca816, 0x1428ca902, 0x1428d9430):
        panics.append(hex(address))
        uc.emu_stop()
    elif address in hooks:
        name = hooks[address]
        c, d, r8, r9, sp = [reg(r) for r in (UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_R9, UC_X86_REG_RSP)]
        if name == "end": value = ["end"]
        elif name == "viewport": value = ["viewport", *get(r9, "<6f")]
        elif name == "scissor": value = ["scissor", *get(r9, "<iiII")]
        elif name == "stencil": value = ["stencil", d, r8]
        elif name == "line_width": value = ["line_width", struct.unpack("<f", struct.pack("<I", reg(UC_X86_REG_XMM1)&0xffffffff))[0]]
        elif name == "vertex": value = ["vertex", d, get(r9, "<Q")[0], get(get(sp+0x28, "<Q")[0], "<Q")[0]]
        elif name == "index": value = ["index", d, r8, r9]
        elif name == "push": value = ["push", d, r8, r9, list(cpu.mem_read(get(sp+0x30, "<Q")[0], get(sp+0x28, "<I")[0]))]
        elif name == "copy":
            p = get(sp+0x28, "<Q")[0]
            value = ["copy", d, r8, [list(get(p+i*24, "<QQQ")) for i in range(r9)]]
        elif name == "fill": value = ["fill", d, r8, r9, get(sp+0x28, "<I")[0]]
        elif name == "clear": value = ["clear", d, r8, list(get(r9, "<4f")), list(get(get(sp+0x30, "<Q")[0], "<5I"))]
        calls.append({"cb":c, "call":value})
        ret()

cpu.hook_add(UC_HOOK_CODE, hook)

def run(address, d=0, r8=0, r9=0, fifth=0, xmm1=0):
    sp = STACK+0x10000-8
    put(sp, "<Q", STOP)
    put(sp+0x28, "<Q", fifth)
    for r, v in [(UC_X86_REG_RSP,sp),(UC_X86_REG_RCX,API),(UC_X86_REG_RDX,d),(UC_X86_REG_R8,r8),(UC_X86_REG_R9,r9),(UC_X86_REG_XMM1,xmm1)]:
        cpu.reg_write(r,v)
    cpu.emu_start(address, STOP, count=10000)

# 从原入口执行命令，输入对象为明确 fixture；成功路径的内联 managed 校验仍执行。
run(0x1416f7870, 1920|(1080<<32))
for value in (0,255,255):
    run(0x1416f7920, value)
run(0x1416f7960, xmm1=struct.unpack("<I",struct.pack("<f",2.5))[0])
run(0x1416f7980, 3, HANDLE1)
run(0x1416f79f0, HANDLE2, 1)
for layout, kind, data in [(30,0,b"\x01\x02\x03\x04"),(31,1,b"\x05\x06\x07\x08")]:
    put(PIPELINE+0x20,"<Q",layout)
    put(PIPELINE+0x30,"<I",kind)
    cpu.mem_write(BYTES,data)
    run(0x1416f7a50, PIPELINE, BYTES,4)
run(0x1416f7850)
put(REGIONS,"<QQQ",4,8,16)
run(0x1416f7ac0,HANDLE1,HANDLE2,REGIONS,1)
put(RANGE,"<I",0)
run(0x1416f7c30,HANDLE2,RANGE,0xdeadbeef)
put(RANGE,"<I",1)
put(RANGE+8,"<QQ",4,16)
run(0x1416f7c30,HANDLE2,RANGE,42)
put(HANDLE1+0x16,"<B",0)
put(OBJECT1+0x40,"<Q",40)
put(OBJECT1+0x28,"<IB",6,4)
run(0x1416f7d80,HANDLE1)
assert not panics, panics
result = subprocess.run(["cargo","test","--offline","--locked","--manifest-path",str(ROOT/"Cargo.toml"),"--test","pass_api","command_sequence_binary_fixture","--","--nocapture"],capture_output=True,text=True,check=True)
candidate = json.loads(re.search(r"PASS_API_FIXTURE=(\[.*\])",result.stdout).group(1))
assert candidate[0]["call"][0] == "begin"
# 原 begin 包含未闭合缓存生产者，该项只作候选测试，排除出完整入口差分。
mismatches = [{"index":i,"original":a,"candidate":b} for i,(a,b) in enumerate(zip(calls,candidate[1:])) if a != b]
assert len(calls)==len(candidate)-1
# 独立检查明确失败与门控边界，Rust 对应测试单独验收。
before = len(calls)
put(DEVICE+0xad9,"<B",0)
run(0x1416f7960,xmm1=0x41100000)
assert len(calls)==before
for off, length in [(1,16),(4,17)]:
    put(RANGE+8,"<QQ",off,length)
    run(0x1416f7c30,HANDLE2,RANGE,0)
assert len(panics)==2
report = {
 "status":"verified_partial_boundary" if not mismatches else "mismatch",
 "binary_sha256":EXPECTED,
 "rust_tests":3,
 "native_complete_entries":10,
 "matched_backend_calls":len(calls),
 "mismatches":mismatches,
 "original_calls":calls,
 "candidate_calls":candidate[1:],
 "alignment_original_panic_boundaries":panics,
 "managed_resolver_hook_calls":len(resolver_calls),
 "limits":["begin_render_pass附件/视图及缓存生产者未比较","vertex/index使用外供managed get/track返回对象","copy/fill/clear内联校验成功路径使用fixture对象；校验失败边界未全部覆盖","Vulkan函数指针以参数捕获hook代替，无GPU执行","Rust填充对齐错误返回Result，原对应路径panic；已显式记录接口差异"],
 "input_hashes":{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [ROOT/"src/rg/pass_api.rs",ROOT/"tests/pass_api.rs",Path(__file__),ROOT/"Cargo.lock"]}
}
(ROOT/"evidence/pass-api-comparison.json").write_text(json.dumps(report,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
print(json.dumps({k:report[k] for k in ["status","native_complete_entries","matched_backend_calls","mismatches"]},ensure_ascii=False))
assert not mismatches
