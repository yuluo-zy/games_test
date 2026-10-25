"""以功能链和验收证据报告进度，不使用文件数量推算完成度。"""
from pathlib import Path
import datetime, hashlib, json
ROOT = Path(__file__).resolve().parents[1]
SCOPE = [
 ('设备筛选与初始化','高','device-policy-functional-validation.json','原Ron白名单、驱动/设备兼容判断、surface筛选与设备创建','原完整排序、未知驱动编码及implicit layer来源待核实'),
 ('基础资源与设备能力','高','raw-resource-implementation.json','真实buffer/image/view/sampler创建、映射、读回和释放；BDA与时间线提交','ray tracing完整路径未验收；硬件只有Intel UHD630'),
 ('分配器私有包装','高','allocator-functional-validation.json','分配/释放、峰值、失败采样与预算回滚；实际GPU资源回收','gpu-allocator0.26原本地分叉来源未确认，公共池为外部缺口'),
 ('shader与descriptor生产','极高','shader-production-validation.json','404原文件/638反射项布局、descriptor复用、管线缓存；原填充shader与raster读回','原资产/pass关联、淘汰及完整材质选择未知；638管线未全量运行'),
 ('Ref与managed绑定','高','pass-bindings-functional-validation.json','Ref访问声明、主图注册表解析、generation/device校验、descriptor持有与GPU消费','当前provider桥分两阶段；自动接入主图首个shader callback、原完整ABI与游戏producer待完成'),
 ('附件与渲染命令','高','attachment-production-validation.json','附件生成view/renderpass/framebuffer缓存；begin/clear/end和像素读回','多子通道、input/resolve及全量pass组合未验收'),
 ('渲染图与队列同步','极高','queue-graph-functional-validation.json','整图生成提交数组；主/辅助队列候选合同；GPU外部wait/signal与两批次读回','原并行调度与跨族ownership transfer未恢复；辅助队列无本机实测'),
 ('持久渲染session','高','persistent-session-validation.json','连续3帧复用chain/pool/timeline，resize、命令缓冲复用、失败停止与回收','串行候选；原并行调度与完整失败展开未知'),
 ('temporal跨帧','高','temporal-functional-validation.json','原状态机、重复请求、命中规则、pair/export；6帧GPU双缓冲与resize','原未使用key淘汰/reset触发、完整ABI与游戏consumer未验收'),
 ('managed生命周期','高','managed-implementation.json','generation、Arc、CPU退役、序号到时间线及多队列回收','原全域producer/序号桶与性能未整体对照；raw析构保守等待'),
 ('窗口交换链','高','swapchain-functional-validation.json','真实acquire→GPU submit→present；3帧、resize、vsync重建与Win32窗口释放','原winit分叉、焦点/全屏/完整WSI策略未恢复'),
 ('GPU诊断','高','crash-functional-validation.json','GPU marker、assertion/debug packet读回、错误门控；原局部入口对照','原shader诊断producer、vendor dump和实际device-lost未验收'),
 ('格式映射','中','format-functional-validation.json','3个原入口816案例对照；另提供官方249格式通用查询','原私有与Khronos语义分开，不以公共表代替原入口等价证据'),
]
def freshness(report):
 hashes = report.get('input_hashes', report.get('input_sha256', report.get('source_hashes', {})))
 if not isinstance(hashes,dict) or not hashes:
  return {'state':'not_recorded','changed_inputs':[]}
 changed=[]
 for name,digest in hashes.items():
  path=Path(name)
  if not path.is_absolute(): path=ROOT/path
  if not path.exists() and not Path(name).is_absolute(): path=ROOT.parents[1]/name
  if not path.exists() or hashlib.sha256(path.read_bytes()).hexdigest()!=digest: changed.append(name)
 return {'state':'stale' if changed else 'matched','changed_inputs':changed}
def main():
 rows=[]
 for name,difficulty,filename,functionality,remaining in SCOPE:
  path=ROOT/'evidence'/filename
  report=json.loads(path.read_text(encoding='utf-8')) if path.exists() else None
  rows.append({'subsystem':name,'difficulty':difficulty,'functional_scope':functionality,'remaining':remaining,
    'evidence':'evidence/'+filename if report else None,
    'evidence_status':report.get('status','scope_report_available') if report else 'in_progress_no_acceptance_report',
    'evidence_freshness':freshness(report) if report else None,'complete_original_equivalence':False})
 aggregate=ROOT/'evidence/functional-project-validation.json'
 project=json.loads(aggregate.read_text(encoding='utf-8')) if aggregate.exists() else None
 project_current=bool(project and project.get('status')=='passed_current_candidate_functional_scope' and freshness(project)['state']=='matched')
 for row in rows:
  row['candidate_current_project_revalidated']=project_current
 output={'timestamp_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),
  'conclusion':'候选已实现多条可运行功能链，原作全功能等价性尚未验收。验收仅限证据明确覆盖的行为。',
  'completion_metric':'functional_acceptance_only','full_original_equivalence_verified':False,'progress_percentage':None,
  'percentage_reason':'没有完整原功能验收分母；不从文件非空或函数数推算完成率。','subsystems':rows,
  'project_validation':{'evidence':'evidence/functional-project-validation.json','status':project.get('status') if project else 'not_run','freshness':freshness(project) if project else None},
  'limits':['历史证据保留输入hash；源码改变需重新验收，不自动延用通过结论。','缺少原依赖分叉、原游戏构图和画面对比，不能宣称完全逆向完成。']}
 for filename in ['module-progress.json','functional-coverage-audit.json']:
  (ROOT/'evidence'/filename).write_text(json.dumps(output,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
 lines=['# 功能进度与难度','',output['conclusion'],'','以可观察功能、原二进制对照范围和实际GPU验收作为进度依据。文件数量不计入完成度。','','| 子系统 | 功能验收范围 | 证据状态 / 输入 | 剩余缺口 | 难度 |','|---|---|---|---|---|']
 for row in rows:
  evidence=f"[{row['evidence_status']}]({row['evidence']})" if row['evidence'] else '正在实现，尚无验收报告'
  if row['evidence_freshness']:
   evidence+=' / '+{'matched':'记录输入一致','stale':'原报告输入已变化','not_recorded':'未记录统一输入'}[row['evidence_freshness']['state']]
  if project_current: evidence+='；候选本轮统一复验通过'
  lines.append(f"| {row['subsystem']} | {row['functional_scope']} | {evidence} | {row['remaining']} | {row['difficulty']} |")
 lines+=['','整体复验状态：'+str(output['project_validation']['status'])+'。见[统一验收记录](evidence/functional-project-validation.json)。','','原55个PDB路径只作为结构清单，见evidence/original-structure.json；新增候选接口、字段编码和性能策略需独立对照。']
 (ROOT/'MODULE_STATUS.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
 print(json.dumps({'subsystems':len(rows),'full_original_equivalence_verified':False,'project_validation':output['project_validation']['status']},ensure_ascii=False))
if __name__=='__main__': main()
