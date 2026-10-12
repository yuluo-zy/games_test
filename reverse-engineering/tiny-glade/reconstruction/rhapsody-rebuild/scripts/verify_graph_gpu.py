"""运行真实整图 GPU 闭包并固化输入；不是原游戏GPU画面对照。"""
from pathlib import Path
import hashlib,json,subprocess,datetime
ROOT=Path(__file__).resolve().parents[1]
def hashes():
 paths=list((ROOT/'src').rglob('*.rs'))+[ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'examples/vulkan_smoke.rs',Path(__file__)]
 return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes()
command=['cargo','run','--offline','--locked','--example','vulkan_smoke','--manifest-path',str(ROOT/'Cargo.toml')]
result=subprocess.run(command,capture_output=True,text=True,check=True)
observation=json.loads(result.stdout.splitlines()[-1]);changed=before!=hashes()
report={'status':'candidate_live_graph_passed' if not changed and observation['status']=='passed' else 'failed_or_stale',
 'timestamp_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':command,'observation':observation,
 'input_hashes':before,'inputs_changed_during_run':changed,'original_complete_equivalence_verified':False,
 'limits':['真实GPU链仅buffer创建/填充/拷贝/屏障/时间线与读回；没有原作shader/画面对照',
 '访问码到Vulkan阶段/掩码的表显式传入，不声明原vk_sync_fork枚举已恢复',
 '没有窗口swapchain或raster绘制验收；资源等待/回收性能策略仍是候选']}
(ROOT/'evidence/graph-vulkan-probe.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'status':report['status'],'observation':observation,'inputs_changed_during_run':changed},ensure_ascii=False))
assert report['status']=='candidate_live_graph_passed'
