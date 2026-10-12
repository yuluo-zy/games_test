import json,hashlib,subprocess
from pathlib import Path
OUT=Path(__file__).resolve().parent
PROJECT=OUT.parent.parent
INDEX=PROJECT/'skeleton/task-index'
def digest(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def write(p,v): p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n')
def run(args):
 r=subprocess.run(args,capture_output=True,text=True)
 if r.returncode: raise RuntimeError(r.stdout+r.stderr)
 return r.stdout
m=json.loads((OUT/'mapping.json').read_text()); a=json.loads((OUT/'assembly.json').read_text())
for x in m:
 code=Path(x['candidate_path']).read_text(); marker='// ORIGINAL: '+x['original_symbol']+'\n'
 x['candidate_sha256']=digest(x['candidate_path']); x['source_marker_line']=code[:code.index(marker)].count('\n')+1
write(OUT/'mapping.json',m)
refs={}
for pattern in ['pack-*/*.json','pack-*/*.asm.txt','support-pack-*/*.json','support-pack-*/*.asm.txt']:
 for p in OUT.glob(pattern): refs[str(p)]=digest(p)
for n in ['selection.json','contracts-observations.json','contract-review.md','manifest-A.json','manifest-B.json','manifest-C.json']:
 refs[str(OUT/n)]=digest(OUT/n)
for n in ['tracking/dependency-exclusions.json','tracking/baselines.json','DEPENDENCY_FILTERING.md','skeleton/B/Cargo.lock','skeleton/C/Cargo.lock','reconstruction/Cargo.lock']:
 p=PROJECT/n; refs[str(p)]=digest(p)
for p,h in a['base_dependencies'].items():
 assert digest(p)==h, 'base changed: '+p
 refs[p]=h
closure={**refs,**{p:digest(p) for p in a['new_modules'].values()},a['root']:digest(a['root'])}
write(OUT/'dependency-closure.json',{'target':'C','scope':'candidate assembly and identity-bound evidence; no production integration','files_sha256':closure,'external_crates':'glam and std are reused; Cargo dependency versions/features must be bound by integrator before future build; no Cargo manifest created','unresolved':['Original full ABI and producer/consumer integration','Original locked dependency compatibility of candidate assembly','Runtime differential and actual World chain validation paused']})
receipts=[]
for task in dict.fromkeys(x['task_id'] for x in m):
 entries=[x for x in m if x['task_id']==task]; claim=json.loads((INDEX/'claims'/f'{task}.json').read_text()); assert claim['owner']=='abc-building-35'
 work=Path(claim['work_dir']); write(work/'mapping.json',entries); write(work/'evidence-refs.json',{'files_sha256':refs}); write(work/'reuse-review.json',{'entries':[{k:x[k] for k in ['original_symbol','existing_rust_name_candidates','dependency_reuse']} for x in entries],'note':'Name candidates alone are not acceptance. Previous 52 types/functions reused through assembly imports; public std/glam internals not rewritten.'})
 lines=['# 有界候选合同','', '目标C；A/B参照。原body同码仅支持本批观察范围。完整作者ABI、枚举和容器布局未知；语义Rust接口不宣称原内存表示。','', '独立恢复前合同见 '+str(OUT/'contract-review.md')+'；本批过滤/寄存器补证见support-pack-A/B/C。','', '依赖：std/Vec与glam使用现成API；已冻结上批类型通过assembly借用，禁止另写公共库内部。未接生产Cargo，锁与feature适配由后续整合负责。','', '## 每函数观察与边界','']
 obligations=['# 待验收义务','', '| 函数 | 原证据与Rust位置 | 消费者/版本 | 验收状态与缺口 |','|---|---|---|---|']
 for x in entries:
  lines+=['### '+x['original_symbol'],'',f"- observed / ABC：原指令见contracts-observations.json键`{x['original_symbol']}`及mapping中native_instances的evidence；具体VA见本文件源码注释。",f"- Rust：{x['candidate_path']}:{x['source_marker_line']}。输入/输出、数值顺序、失败边界以该函数中文注释与独立合同为范围。",'- inferred：字段语义名、借用视图/泛型接口是候选API，不是完整原类型的ABI证明。','- unknown：原始完整签名、合法输入producer、跨版本消费者接入；PDB调用者列表在mapping的indexed_callers/physical_reference_callers_rva，未恢复这些caller不能声称业务闭环。','- 副作用：仅保留该叶体的返回/字段赋值；没有原证据的通知、分配、清理或历史推进不自行增加。父owner的增改删、传播、回放另待接入。','']
  obligations.append(f"| `{x['original_symbol']}` | mapping与中文注释行{x['source_marker_line']}；原asm具体VA | C目标，ABC同码参照；caller见mapping | implemented_unverified；字段/ABI/依赖兼容及真实producer→consumer→history→removal待补/待验 |")
 obligations+=['','待运行反例：动态网格宽≠64及signed回绕；NaN/无穷/边界/负半径；LoopRange负值；三维矩形上下边同时平移；key互异高位输入；holes origin过滤及未过滤别名；空Vec复用。此列表未执行，不构成验证。']
 (work/'CONTRACT.md').write_text('\n'.join(lines)+'\n'); (work/'obligations.md').write_text('\n'.join(obligations)+'\n')
 files=['mapping.json','CONTRACT.md','obligations.md','evidence-refs.json','reuse-review.json',entries[0]['source_path']]
 if task=='TG-1d8b7844066af51d6b':
  write(work/'assembly/dependency-closure.json',json.loads((OUT/'dependency-closure.json').read_text())); files+=['assembly/lib.rs','base-dependencies.json','assembly/dependency-closure.json']
 manifest=work/'delivery.json'
 if manifest.exists(): manifest.rename(work/'delivery-before-final-closure.json')
 cmd=['python3',str(PROJECT/'skeleton/workflow-design/tiny-glade-recovery/scripts/workflow.py'),'freeze','--root',str(work),'--output',str(manifest)]
 for f in files: cmd+=['--file',f]
 run(cmd); run(['python3',str(PROJECT/'skeleton/workflow-design/tiny-glade-recovery/scripts/workflow.py'),'check','--manifest',str(manifest)])
 receipts.append({'task_id':task,'manifest':str(manifest),'manifest_sha256':digest(manifest),'status':'frozen_unverified'})
write(OUT/'deliveries.json',receipts)
print(json.dumps({'candidates':len(m),'frozen_file_groups':len(receipts),'closure_files':len(closure)}))
