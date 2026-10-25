"""Apply separately verified implementation reports; retain the pre-edit audit."""
from pathlib import Path
from collections import Counter
import json,hashlib
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'evidence/foundation-completion-2026-10-13'
MASTER=ROOT/'tracking/shared-foundation-business-comparison.json'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
snapshot=OUT/'pre-implementation-comparison.json'
if not snapshot.exists():snapshot.write_bytes(MASTER.read_bytes())
doc=json.loads(snapshot.read_text(encoding='utf-8'))
by_id={r['feature_id']:r for r in doc['features']}
reports=[];seen=set()
for group in ('curves','geometry','base-assets','wall-heightmap'):
    p=OUT/group/'report.json'
    report=json.loads(p.read_text(encoding='utf-8'))
    reports.append({'path':str(p.relative_to(ROOT)).replace('\\','/'),'sha256':sha(p)})
    for relative,expected in report.get('source_sha256',{}).items():
        assert sha(ROOT/relative)==expected,f'Changed owner source: {relative}'
    for entry in report['features']:
        key=entry['feature_id'];assert key not in seen,key;seen.add(key)
        row=by_id[key]
        row['pre_implementation_status']=row['current_status']
        for field in ('current_status','implemented_scope','missing_library_logic','unresolved_contracts','validation_gaps','source_mapping'):
            if field in entry:row[field]=entry[field]
        row['evidence']=list(dict.fromkeys(row['evidence']+entry.get('evidence',[])+[reports[-1]['path']]))
        row['implementation_report']=reports[-1]['path']
        row['full_original_feature_accepted']=False
        row['reference_corrections']=list(dict.fromkeys(row['reference_corrections']+entry.get('reference_corrections',[])))
doc['inputs']=[{'path':str(snapshot.relative_to(ROOT)).replace('\\','/'),'sha256':sha(snapshot)}]+reports
doc['current_sources']={str(p.relative_to(ROOT)).replace('\\','/'):sha(p) for lib in doc['scope'] for p in sorted((ROOT/'reconstruction/shared-foundation/crates'/lib/'src').rglob('*.rs'))}
doc['statistics']=dict(Counter(r['current_status'] for r in doc['features']))
doc['libraries']={lib:dict(Counter(r['current_status'] for r in doc['features'] if r['library']==lib)) for lib in doc['scope']}
doc.update(source_modified=True,tests_rerun=True,implementation_reports=reports,
           implementation_validation='evidence/foundation-completion-2026-10-13/integration-validation.json',
           historical_comparison=str(snapshot.relative_to(ROOT)).replace('\\','/'))
write(MASTER,doc)
states=('implemented_scoped','partial_missing_logic','not_implemented','integration_pending','contract_only','out_of_baseline')
labels=('已有限定实现','部分逻辑／契约待核','尚无业务实现','外部接线','数据契约','C版范围')
lines=['# 七个基础库：补全后的业务功能对照','',
       '以 B 版为主，恢复业务等价实现。当前源码和补全证据已更新；原始审查快照保留在 '+doc['historical_comparison']+'。编译、候选测试、原作局部对照与整个游戏验收分别记录。','',
       '| 库 | '+' | '.join(labels)+' |','|---|'+ '|'.join('---:' for _ in states)+'|']
for lib in doc['scope']:lines.append('| '+lib+' | '+' | '.join(str(doc['libraries'][lib].get(s,0))for s in states)+' |')
lines+=['| 合计 | '+' | '.join(str(doc['statistics'].get(s,0))for s in states)+' |','',
        '## 仍需处理的库内规则','']
for r in doc['features']:
    pending=r['missing_library_logic']+r['unresolved_contracts']
    if pending:lines.append('- `'+r['feature_id']+'` '+r['name']+'：'+'；'.join(pending))
lines+=['','## 全部功能','', '| ID／功能 | 当前状态 | 已实现范围 | 库内缺口／契约 | 验证边界／外部接线 | 证据 |','|---|---|---|---|---|---|']
def cell(v):
    if isinstance(v,list):v='；'.join(str(x)for x in v)
    return str(v).replace('|','\\|').replace('\n',' ')
names=dict(zip(states,labels))
for r in doc['features']:
    lines.append('| `'+r['feature_id']+'` '+cell(r['name'])+' | '+names.get(r['current_status'],r['current_status'])+' | '+cell(r['implemented_scope'])+' | '+cell(r['missing_library_logic']+r['unresolved_contracts'])+' | '+cell(r['validation_gaps']+r['consumer_gaps'])+' | '+cell(r['implementation_report']if 'implementation_report'in r else r['detailed_comparison'])+' |')
(ROOT/'FOUNDATION_BUSINESS_COMPARISON.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps({'updated_features':len(seen),'statistics':doc['statistics']},ensure_ascii=False))
