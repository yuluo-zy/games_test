"""编译并在真实 Vulkan 驱动运行整个基础资源闭包，记录其明确边界。"""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[1]
MODULES = ['instance', 'physical_device', 'init', 'allocator', 'buffer', 'texture',
           'sampler', 'framebuffer', 'render_pass', 'swapchain', 'format', 'error', 'bytes']


def hashes():
    paths = list((ROOT / 'src').rglob('*.rs')) + [ROOT / 'Cargo.toml', ROOT / 'Cargo.lock',
        ROOT / 'examples/raw_resource_smoke.rs', Path(__file__)]
    return {str(path.relative_to(ROOT)).replace('\\', '/'): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in paths}


def main():
    before = hashes()
    build_command = ['cargo', 'build', '--offline', '--locked', '--manifest-path',
                     str(ROOT / 'Cargo.toml'), '--example', 'raw_resource_smoke']
    subprocess.run(build_command, check=True)
    process = subprocess.run([str(ROOT / 'target/debug/examples/raw_resource_smoke.exe')],
                             text=True, capture_output=True, check=True)
    result = json.loads(process.stdout)
    after = hashes()
    changed = [key for key in set(before) | set(after) if before.get(key) != after.get(key)]
    report = {'status': 'passed_live_resource_smoke' if not changed else 'stale_inputs_changed',
              'module_scope': MODULES, 'build_command': build_command,
              'live_driver_observation': result, 'input_hashes': before,
              'changed_during_validation': changed, 'original_binary_equivalence_verified': False,
              'capabilities': [
                  'Vulkan实例/物理设备枚举/逻辑设备与时间线能力查询',
                  '专用内存分配、内存类型选择、buffer绑定及映射读写、非一致内存刷新/失效',
                  'image分配与绑定、图片视图、mip范围、render pass/framebuffer/sampler创建和析构',
                  'surface包装和Windows创建入口、swapchain创建/调整/获取/呈现及持有租约的完整候选接口',
              ],
              'limits': [
                  '真实驱动只验收基础资源创建/映射读写/销毁；没有执行绘制或验证原作GPU画面。',
                  '没有打开窗口，surface/swapchain真实驱动路径未运行，仅通过编译检查。',
                  '专用vkAllocateMemory是明确候选适配：原gpu-allocator 0.26本地分叉来源未取回；没有重写其通用次分配或原池策略。',
                  '这些公开签名由PDB函数名/项目层次约束构建，不声明原Rust类型/ABI或兼容名单已全部恢复。',
                  'format块信息表有限；未知格式返回None，未补造完整格式表。',
                  '此路径未启用bufferDeviceAddress，不接受SHADER_DEVICE_ADDRESS usage；专用BDA/稀疏/外部内存还未恢复。',
                  '资源析构和映射采用保守device_wait_idle，保证这一路径可运行，尚未替换成原作细粒度延迟回收性能策略。',
              ]}
    (ROOT / 'evidence/raw-resource-implementation.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'status': report['status'], 'live_driver_observation': result,
                      'changed_during_validation': changed}, ensure_ascii=False))
    if changed:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
