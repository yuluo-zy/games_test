"""List every unmapped original interface and its evidence-backed reason."""
from collections import Counter
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MASTER = 'tracking/shared-foundation-interfaces.json'
master = json.loads((ROOT / MASTER).read_text(encoding='utf-8'))
audits = {}
for entry in master['inputs']:
    if entry['path'].endswith('interface-coverage.json'):
        data = json.loads((ROOT / entry['path']).read_text(encoding='utf-8'))
        audits[entry['path']] = {
            x.get('interface_id', x.get('canonical_key', x.get('id'))): x
            for x in data['interfaces']}

rows = []
for entry in master['interfaces']:
    if not entry['count_as_business_interface'] or entry['candidate_implementation_present']:
        continue
    original = audits[entry['detailed_audit']][entry['detailed_interface_key']]
    gaps = original.get('gaps', [])
    if entry['ownership_triage_pending']:
        reason = 'ownership_pending'
        explanation = '该 trait/schema 条目尚未确认是游戏规则、派生字段映射还是依赖胶水；先确认归属，再决定恢复或直接复用。'
    elif entry['business_analysis_started']:
        reason = 'analyzed_not_implemented'
        explanation = '已有部分原业务分析，但尚未完成可接受的映射实现；具体剩余规则见下方原审计缺口。'
    elif original.get('decompiled_evidence_present'):
        reason = 'decompiled_not_business_analyzed'
        explanation = '已有反编译材料，但尚未把分支、参数及引用方组合解释成业务合同；不是反编译工具失败。'
    else:
        reason = 'not_yet_business_analyzed'
        explanation = '尚未进入已验收的业务分析/实现批次；只有接口库存不能证明算法与调用策略。'
    source = original.get('original_source')
    if not source and original.get('original_sources'):
        source = ', '.join(original['original_sources'])
    if not source:
        records = original.get('pdb_provenance', [])
        source = next((x.get('source', '').removeprefix('skeleton/B/') for x in records if x.get('source')), entry['source_module'])
    rows.append({'interface_id': entry['interface_id'], 'source_module':entry['source_module'],
                 'original_source':source, 'reason':reason, 'explanation':explanation,
                 'original_audit_gaps':gaps, 'detailed_audit':entry['detailed_audit'],
                 'business_analysis_started':entry['business_analysis_started'],
                 'ownership_triage_pending':entry['ownership_triage_pending'],
                 'known_attempt_failed':False})

groups = Counter(x['source_module'] for x in rows)
reasons = Counter(x['reason'] for x in rows)
doc = {'baseline':'B', 'denominator_complete':False, 'count':len(rows),
       'definition':'Classified business interfaces without a reviewed mapped implementation. Generated reuse, schema helper records and unresolved traits are excluded and separately retained.',
       'modules':dict(groups), 'reasons':dict(reasons), 'interfaces':rows,
       'excluded_queues':master['classified_queues'],
       'candidate_without_business_analysis':[x['interface_id'] for x in master['interfaces'] if x['count_as_business_interface'] and x['candidate_implementation_present'] and x['untouched']],
       'inventory_input':{'path':MASTER,'sha256':hashlib.sha256((ROOT/MASTER).read_bytes()).hexdigest()}}
(ROOT/'tracking/unrecovered-shared-foundation-interfaces.json').write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
labels = {'ownership_pending':'归属待确认', 'analyzed_not_implemented':'已有分析，未实现',
          'decompiled_not_business_analyzed':'已有伪码，未解释业务', 'not_yet_business_analyzed':'尚未进行业务分析'}
lines = ['# 未恢复接口与原因（共享基础，B 版）', '',
         f'当前业务接口台账有 {len(rows)} 项没有已审查的映射实现。其中 {reasons["analyzed_not_implemented"]} 项已有部分分析，另 {len(rows)-reasons["analyzed_not_implemented"]} 项尚无接受的业务分析。', '',
         f'原先314项混入了自动实现/依赖转发、schema helper和待分类条目，已纠正。完整609项原库存不删除；自动实现/库转发{master["classification_counts"]["generated_reuse"]}项、schema helper {master["classification_counts"]["schema_policy"]}项和待分类{master["classification_counts"]["unresolved"]}项见[分类表](INTERFACE_CLASSIFICATION.md)，不再当作尚未实现业务函数。此变化不表示新增功能恢复。', '',
         '这不是恢复失败次数。未进入分析批次及材料未解释保留为业务缺口，归属未知则另列待分类；不得编造尝试失败记录。PDB 库存也不是作者全部 API，未发现的接口不在这个分母内。', '',
         '| 源模块 | 无映射实现 |', '|---|---:|']
lines += [f'| `{module}` | {count} |' for module,count in sorted(groups.items())]
lines += ['', '| 原因 | 数量 |', '|---|---:|']
lines += [f'| {labels[reason]} | {count} |' for reason,count in reasons.items()]
lines += ['', f'## 已分析但尚未实现的{reasons["analyzed_not_implemented"]}个接口', '']
for row in rows:
    if row['business_analysis_started']:
        lines += [f'- `{row["interface_id"]}`：'+'；'.join(row['original_audit_gaps'])]
lines += ['', '## 为什么有候选代码也不能算恢复完成', '',
          f'以下{len(doc["candidate_without_business_analysis"])}项虽有候选代码，但原业务分析仍未闭合，独立保留为未触及；不混入上述无映射实现的{len(rows)}项：', '']
lines += [f'- `{key}`' for key in doc['candidate_without_business_analysis']]
lines += ['', '已有实现还需区分泛型策略、边界输入及原游戏消费者链。例如 CurveSlice 的新状态对象只是未验收候选，不从文件或声明存在自动提升原接口状态。完整图片源资源缺失影响纹理 producer 的对照，不是全部资源接口缺失的统一原因。', '',
          '## 完整逐接口清单', '']
for module in sorted(groups):
    lines += [f'### {module}', '', '| 原接口 | 原源码 | 当前原因 | 原审计缺口 |', '|---|---|---|---|']
    for row in rows:
        if row['source_module'] == module:
            gap = '; '.join(row['original_audit_gaps']).replace('|','\\|').replace('\n',' ')
            lines.append(f'| `{row["interface_id"]}` | `{row["original_source"]}` | {labels[row["reason"]]}：{row["explanation"]} | {gap or "见原接口审计；尚未形成接受的业务合同。"} |')
    lines.append('')
lines += ['[完整机器台账](tracking/unrecovered-shared-foundation-interfaces.json)；[接口进度表](INTERFACE_PROGRESS.md)；[本轮代码 review](evidence/shared-foundation-review-2026-10-13/REVIEW.md)。']
(ROOT/'UNRECOVERED_INTERFACES.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps({'unmapped':len(rows),'modules':dict(groups),'reasons':dict(reasons)},ensure_ascii=False))
