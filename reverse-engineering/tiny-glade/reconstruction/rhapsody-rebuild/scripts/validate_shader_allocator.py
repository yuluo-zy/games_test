"""重新生成 shader/allocator 子系统证据；原公共依赖只通过明确 fixture 边界供给。"""
import datetime
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parents[1]
PYTHON_SUPPORT = ROOT / "scripts/native_support"
sys.path.insert(0, str(PYTHON_SUPPORT))
import pe_fixture as native
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RAX, UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_R8, UC_X86_REG_RSP


def run_example(name, *args):
    return json.loads(subprocess.check_output([str(ROOT / "target/debug/examples" / (name + ".exe")), *args], text=True))


def original(entry, rcx, rdx=0, r8=0):
    native.STATE.clear()
    native.STATE.update(visited=set(), cut=native.STOP, cut_reached=False, panic="ok")
    native.write(native.STACK + 0x10008, "<Q", native.STOP)
    for register, value in [(UC_X86_REG_RSP, native.STACK + 0x10008), (UC_X86_REG_RCX, rcx), (UC_X86_REG_RDX, rdx), (UC_X86_REG_R8, r8)]:
        native.CPU.reg_write(register, value)
    native.CPU.emu_start(entry, native.STOP, count=100000)
    return native.STATE["panic"] == "ok", native.CPU.reg_read(UC_X86_REG_RAX)


def input_hashes(paths):
    return {str(path.relative_to(ROOT)).replace("\\", "/"): hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}


def shader_validation():
    mapping = run_example("shader_production_gpu", "--mapping")
    blends, vertices = [], []
    for flags in range(256):
        native.CPU.mem_write(native.MEMORY, bytes(64))
        native.CPU.mem_write(native.MEMORY + 0x1000, bytes(64))
        native.write(native.MEMORY + 0x101B, "<B", flags)
        returned, _ = original(0x140BE67F0, native.MEMORY, native.MEMORY + 0x1000)
        result = list(native.read(native.MEMORY, "<8I"))
        assert returned and result == mapping["blend"][flags]
        blends.append(result)
    for case in mapping["vertex"]:
        native.write(native.MEMORY, "<I", case["tag"])
        returned, value = original(0x140BE6850, native.MEMORY, case["bytes"])
        result = value & 0xFFFFFFFF if returned else None
        assert result == case["format"]
        vertices.append(dict(case, returned=returned))
    catalog_path = PROJECT / "evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/catalog.json"
    catalog = json.loads(catalog_path.read_text(encoding="utf-8"))
    count = 0
    for file in catalog["files"]:
        data = (Path("D:/game/ljxsj_92385/Tiny Glade/compiled-shaders") / file["file"]).read_bytes()
        assert hashlib.sha256(data).hexdigest() == file["sha256"]
        for module in file["modules"]:
            payload = data[module["offset"]:module["offset"] + 4 * module["word_count"]]
            assert hashlib.sha256(payload).hexdigest() == module["sha256"]
            count += 1
    inputs = [ROOT / name for name in ["src/raw/shader.rs", "src/raw/descriptor_set.rs", "src/rg/binding.rs", "tests/shader_production.rs", "examples/shader_production_gpu.rs", "examples/raster_gpu.rs", "src/raw/device.rs", "src/raw/init.rs", "src/raw/buffer.rs", "src/raw/allocator.rs", "src/raw/command_buffer.rs", "src/rg/attachments.rs", "src/rg/pass_api.rs", "Cargo.toml", "Cargo.lock", "scripts/validate_shader_allocator.py"]]
    evidence = {
        "timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "status": "shader_production_partial_validated",
        "live_gpu": run_example("shader_production_gpu"),
        "live_raster": run_example("raster_gpu"),
        "original_mapping": {"binary_sha256": native.EXPECTED_SHA, "functions": ["0x140be67f0", "0x140be6850"], "blend_cases": 256, "vertex_cases": len(vertices), "mismatches": 0, "blend_outputs": blends, "vertex_outputs": vertices,
                             "boundaries": ["完整原映射入口运行；格式化panic在公共panic helper处拦截", "Rust编译产物--mapping提供候选输出，原映射函数本体未替换"]},
        "reflection_catalog": {"original_files": len(catalog["files"]), "original_modules": count, "container_and_payload_hashes_matched": True, "original_spirq_source": catalog["reflection"], "catalog_sha256": hashlib.sha256(catalog_path.read_bytes()).hexdigest(), "full_catalog_gpu_run": False},
        "implemented_chain": ["原catalog→稀疏set/binding布局合并与stage IO校验", "immutable sampler/设备限制/push范围/runtime最高binding验证", "布局→状态→分配/复用→写入/拥有/绑定", "asset键选择compute/raster→正确入口名→自动布局", "精确公开状态cache key→Arc持有→显式失效", "pass访问声明→命名绑定/数组element→实际descriptor"],
        "limitations": ["原country-core资产键与pass关联未恢复", "原DescriptorSetState序号桶/淘汰频率未知；当前为安全内容缓存候选", "原pipeline资产层缓存策略与完整特殊化/材质状态未全量对齐", "runtime scalar的原Vulkan property精确名称未知，仍显式输入", "638反射布局通过CPU验证，不代表638管线都已运行", "原全帧/游戏画面不在本报告验收范围"],
        "input_hashes": input_hashes(inputs),
    }
    (ROOT / "evidence/shader-production-validation.json").write_text(json.dumps(evidence, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return {"gpu_descriptor_result": evidence["live_gpu"]["arithmetic_result"], "original_fill_words": evidence["live_gpu"]["original_fill_words"], "raster_pixels": evidence["live_raster"]["pixels_verified"], "mapping_mismatches": 0}


def allocator_validation():
    cases = run_example("allocator_gpu", "--rules")["cases"]
    current = {}
    calls = {"allocate": 0, "report": 0, "free": 0}

    def boundary(cpu, address, size, user):
        rcx = cpu.reg_read(UC_X86_REG_RCX)
        if address == 0x140C12DC0:
            # 原公共 gpu-allocator 分叉返回的 Result/Allocation 72字节仅做边界fixture；游戏包装本体未替换。
            native.CPU.mem_write(rcx, bytes(72))
            native.write(rcx + 68, "<B", 2)
            calls["allocate"] += 1
            native.ret(rcx)
        elif address == 0x140C15150:
            # 80字节report：+48 total、+64 device、+72 system；首字仅是快照身份标记。
            native.CPU.mem_write(rcx, bytes(80))
            native.write(rcx, "<Q", current["marker"])
            native.write(rcx + 8, "<Q", 8)
            total, device, system = current["report"]
            native.write(rcx + 48, "<Q", total)
            native.write(rcx + 64, "<QQ", device, system)
            calls["report"] += 1
            native.ret(rcx)
        elif address == 0x140BDFE90:
            native.ret()
        elif address == 0x140C16A00:
            native.ret(0)
        elif address == 0x140C14680:
            native.CPU.mem_write(rcx, bytes(72))
            calls["free"] += 1
            native.ret(rcx)
        elif address == 0x140C120D0:
            native.CPU.mem_write(rcx, bytes(0x620))
            native.ret(rcx)
        elif address == 0x142923C00:
            rdx = cpu.reg_read(UC_X86_REG_RDX)
            length = cpu.reg_read(UC_X86_REG_R8)
            native.CPU.mem_write(rcx, bytes(native.CPU.mem_read(rdx, length)))
            native.ret(rcx)

    hook = native.CPU.hook_add(UC_HOOK_CODE, boundary)
    native.write(0x14362C3A8, "<Q", native.MEMORY + 0x1F000)
    native.write(native.MEMORY + 0x1F000, "<Q", 0)
    receiver, output = native.MEMORY + 0x5000, native.MEMORY + 0x7000
    results = []
    for tracking in [False, True]:
        native.CPU.mem_write(receiver, bytes(0x690))
        returned, _ = original(0x140C0D980, receiver, native.MEMORY + 0x6000, int(tracking))
        assert returned
        assert native.read(receiver + 0x620, "<Q")[0] == 1 << 63
        assert native.read(receiver + 0x670, "<QQQ") == (0, 0, 0)
        assert native.read(receiver + 0x688, "<B")[0] == tracking
        for item in [item for item in cases if item["tracking"] == tracking]:
            current.update(item)
            returned, _ = original(0x140C0DA40, output, receiver, native.MEMORY + 0x6000)
            assert returned
            peaks = [original(entry, receiver)[1] for entry in [0x140C0E600, 0x140C0E630, 0x140C0E640]]
            _, snapshot = original(0x140C0E610, receiver)
            marker = native.read(snapshot, "<Q")[0] if snapshot else None
            assert peaks == item["peaks"] and marker == item["peak_snapshot_marker"]
            results.append(dict(item, native_peaks=peaks, native_snapshot_marker=marker))
        before = bytes(native.CPU.mem_read(receiver + 0x620, 0x68))
        returned, _ = original(0x140C0E150, output, receiver, native.MEMORY + 0x6000)
        assert returned and bytes(native.CPU.mem_read(receiver + 0x620, 0x68)) == before
    native.CPU.hook_del(hook)
    evidence = {"timestamp_utc": datetime.datetime.now(datetime.timezone.utc).isoformat(), "status": "private_wrapper_and_direct_adapter_partial_validated", "live_gpu": run_example("allocator_gpu"),
                "original_wrapper": {"binary_sha256": native.EXPECTED_SHA, "constructor_cases": 2, "allocate_cases": len(results), "free_preserves_peaks_cases": 2, "mismatches": 0, "boundary_calls": calls, "cases": results,
                                     "fields": {"peak_report": "0x620, None首字i64::MIN", "peak_device_local": "0x670", "peak_system": "0x678", "peak_total": "0x680", "track_peaks": "0x688"},
                                     "boundaries": ["原私有new/allocate/free及四个peak getter完整入口运行", "公共gpu-allocator new/allocate/free/report、报表drop、日志/CRT memcpy边界为明确fixture", "报表实际池实现、allocation payload语义及格式化日志未逆向"]},
                "implemented": ["MemoryBlock成功分配/drop自动设备级统计", "预算与溢出失败无统计污染", "独立device/system峰值与严格total新高快照", "原track flag及allocate返回后错误路径采样规则", "内存位置属性选择/DEVICE_ADDRESS flags保留", "GpuAllocator外层包装和DirectMemoryBackend通用依赖边界"],
                "limitations": ["原gpu-allocator0.26本地分叉源码/来源未取回，次分配/池预留/碎片策略未实现", "DirectMemoryBackend报告专用VkDeviceMemory量，不能等同原池AllocatorReport的预留/已用量", "原inner地址存在别名/日志metadata碰撞；公开inner签名仍是候选", "预算为候选显式控制，未证实原游戏采用此策略", "候选AllocatorReport为安全Rust表示，未宣称原80字节report ABI完整恢复"],
                "input_hashes": input_hashes([ROOT / name for name in ["src/raw/allocator.rs", "src/raw/device.rs", "src/raw/buffer.rs", "src/raw/texture.rs", "tests/allocator.rs", "examples/allocator_gpu.rs", "scripts/validate_shader_allocator.py", "Cargo.toml", "Cargo.lock"]])}
    (ROOT / "evidence/allocator-functional-validation.json").write_text(json.dumps(evidence, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return {"gpu_final_live": evidence["live_gpu"]["live_final"], "wrapper_original_mismatches": 0, "private_wrapper_cases": len(results)}


if __name__ == "__main__":
    if "--no-build" not in sys.argv:
        subprocess.run(["cargo", "build", "--offline", "--locked", "--manifest-path", str(ROOT / "Cargo.toml"), "--example", "shader_production_gpu", "--example", "raster_gpu", "--example", "allocator_gpu"], check=True)
    subprocess.run(["cargo", "test", "--offline", "--locked", "--manifest-path", str(ROOT / "Cargo.toml"), "--test", "shader_production", "--test", "allocator"], check=True)
    print(json.dumps({"shader": shader_validation(), "allocator": allocator_validation()}, ensure_ascii=False))
