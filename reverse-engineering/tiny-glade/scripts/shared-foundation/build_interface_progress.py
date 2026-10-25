"""Merge reviewed original B interface audits; never infer progress from code names."""
from collections import Counter
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
BASE = 'evidence/shared-foundation-2026-10-13/'
INPUTS = [BASE+x+'/interface-coverage.json' for x in
          ['small-interfaces','utils-base','geometry-curves','assets-small-first']]
rows = []
inputs = []
for path in INPUTS:
    data = json.loads((ROOT/path).read_text(encoding='utf-8'))
    inputs.append({'path':path,'sha256':hashlib.sha256((ROOT/path).read_bytes()).hexdigest()})
    for original in data['interfaces']:
        if '/small-interfaces/' in path:
            key = original['interface_id']; group = original['crate']
            analyzed = original['business_analysis_started']; implemented = original['has_recovered_implementation']
            native = original['direct_original_entry_differential']; static = original['static_business_compared']
            pending = False; restored = original['restoration']
            source_module = 'crates/'+original['crate']+'/src'
        elif '/utils-base/' in path:
            key = original['canonical_key']; group = 'utils 顶层'
            analyzed = original['analysis_status']=='analyzed_partial'
            implemented = original.get('candidate_implementation') is not None
            native = original['original_instruction_differential']; static = original['static_comparison']
            pending = False; restored = original.get('candidate_implementation')
            source_module = 'crates/utils/src'
        elif '/geometry-curves/' in path:
            key = original['interface_id']; group = 'utils 几何/曲线/Transform'
            analyzed = original['business_analysis_started']; implemented = original['has_recovered_implementation']
            native = original['has_original_comparison']; static = implemented and analyzed
            pending = original['ownership']=='private_or_generated_schema_trait_policy_requires_classification'
            restored = {'file':original.get('restored_file'),'declarations':original.get('restored_declarations')}
            source_module = '/'.join(original['original_source'].split('/')[:4])
        else:
            key = original['id']; group = 'asset / asset-pipe'
            analyzed = original['actual_business_analysis']; implemented = original['implemented']
            native = original['native_compared']; static = original['static_compared']
            pending = False; restored = original.get('recovered_api')
            source_module = 'crates/'+('asset-pipe' if key.startswith('asset_pipe::') else 'asset')+'/src'
        rows.append({'interface_id':key,'group':group,'source_module':source_module,'baseline':'B',
            'business_analysis_started':analyzed,'untouched':not analyzed,
            'candidate_implementation_present':implemented,'static_business_compared':static,
            'direct_original_entry_compared_partial':native,'ownership_triage_pending':pending,
            'full_interface_accepted':False,'original_game_consumer_chain_accepted':False,
            'restoration':restored,'gaps':original.get('gaps',[]),
            'detailed_audit':path,'detailed_interface_key':key})
assert len(rows)==len({x['interface_id'] for x in rows}), 'Cross-owner alias needs review, do not double count'

classification_path = 'tracking/shared-foundation-interface-classification.json'
classification = json.loads((ROOT/classification_path).read_text(encoding='utf-8'))
inputs.append({'path':classification_path, 'sha256':hashlib.sha256((ROOT/classification_path).read_bytes()).hexdigest()})
assert set(classification['entries']) <= {x['interface_id'] for x in rows}, 'Classifier references an absent raw interface'
trait_methods = {'default','fmt','eq','ne','clone','clone_from','partial_cmp','cmp','hash',
                 'clone_behavior','register_required_components','serialize','deserialize',
                 'visit_map','visit_seq','visit_str','visit_string','visit_bytes','visit_byte_buf',
                 'visit_f64','visit_u64','expecting'}
for row in rows:
    decision = classification['entries'].get(row['interface_id'])
    if decision is None:
        method = row['interface_id'].rsplit('::',1)[-1]
        unresolved = row['ownership_triage_pending'] or method in trait_methods
        decision = {'classification':'unresolved' if unresolved else 'custom_business',
                    'reason':'Unclassified trait/owner; separate queue until provenance reviewed.' if unresolved else 'Source-owned ordinary method retained from existing original business-interface audit.',
                    'evidence':[row['detailed_audit']]}
    row['classification'] = decision
    row['count_as_business_interface'] = decision['classification']=='custom_business'
    row['raw_audit_ownership_triage_pending'] = row['ownership_triage_pending']
    row['ownership_triage_pending'] = decision['classification']=='unresolved'
business_rows = [x for x in rows if x['count_as_business_interface']]

review_path = 'tracking/shared-foundation-code-review.json'
review = None
if (ROOT / review_path).exists():
    review = json.loads((ROOT / review_path).read_text(encoding='utf-8'))
    inputs.append({'path':review_path, 'sha256':hashlib.sha256((ROOT/review_path).read_bytes()).hexdigest()})
    for row in rows:
        restoration = row.get('restoration') or {}
        source = restoration.get('file', restoration.get('path'))
        if source in review['current_reviewed_sources']:
            row['code_review'] = {
                'source_file':source, 'source_sha256':review['current_reviewed_sources'][source],
                'review_ledger':review_path,
                'findings_in_same_restored_file':[x['id'] for x in review['findings'] if x['source_file']==source],
                'mapping_basis':'restored source file; findings may concern other interfaces or synthetic adapters in this file',
                'full_original_interface_accepted':False}

def stats(subset):
    return {'observed_entries':len(subset),'business_analyzed_partial':sum(x['business_analysis_started'] for x in subset),
        'untouched':sum(x['untouched'] for x in subset),'candidate_implementation_present':sum(x['candidate_implementation_present'] for x in subset),
        'direct_original_entry_compared_partial':sum(x['direct_original_entry_compared_partial'] for x in subset),
        'ownership_triage_pending':sum(x['ownership_triage_pending'] for x in subset),
        'candidate_present_original_analysis_missing':sum(x['candidate_implementation_present'] and x['untouched'] for x in subset)}

groups = [{'group':name,**stats([x for x in business_rows if x['group']==name])} for name in dict.fromkeys(x['group'] for x in rows)]
doc={'schema_version':1,'baseline':'B','denominator_complete':False,
    'policy':'Business counters include only classified source-owned business APIs. Generated/forwarding methods, schema-policy helpers and unresolved traits are separate. Raw PDB records and historical analysis flags remain available, never promoted merely by reclassification. No game completion percent.',
    'limitations':['PDB is not a complete author API denominator','Private closures are recorded in child audits, not counted as named interfaces',
        'Asset generated-schema rules are additionally tracked in child schema_rules; do not silently treat excluded derived library glue as zero schema work',
        'Ownership-pending geometry trait/schema items are retained separately','Synthetic validation tools are not restored game APIs',
        'Original consumer-chain acceptance is separate from candidate in-crate wiring'],
    'statistics':stats(business_rows),'groups':groups,'interfaces':rows,'inputs':inputs,
    'raw_statistics':stats(rows),
    'classification_counts':{c:sum(x['classification']['classification']==c for x in rows) for c in classification['categories']},
    'classified_queues':{c:[x['interface_id'] for x in rows if x['classification']['classification']==c] for c in classification['categories'] if c!='custom_business'},
    'modules':{m:stats([x for x in business_rows if x['source_module']==m]) for m in sorted({x['source_module'] for x in rows})},
    'private_or_source_owned_entries':stats(business_rows),
    'pending_ownership_entries':stats([x for x in rows if x['ownership_triage_pending']])}
if review:
    doc['code_review'] = {'source':review_path, 'statistics':review['statistics'],
                         'report':'evidence/shared-foundation-review-2026-10-13/REVIEW.md',
                         'changes_original_interface_coverage':False}
target=ROOT/'tracking/shared-foundation-interfaces.json'
target.write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
lines=['# 共享基础：业务函数接口进度','','基线：B。主表只统计已分类的业务接口；宏自动实现/库转发、类型/schema合同及待分类项另列。PDB不是作者API全集，不能从文件数、Rust声明数或测试数推导完成率。', '',
    '| 范围 | 已观察 | 业务已分析（部分） | 未触及 | 候选实现 | 原入口对照（局部） | 待归属 |',
    '|---|---:|---:|---:|---:|---:|---:|']
for g in groups:
    lines.append('| '+' | '.join(str(g[k]) for k in ['group','observed_entries','business_analyzed_partial','untouched','candidate_implementation_present','direct_original_entry_compared_partial','ownership_triage_pending'])+' |')
t=doc['statistics'];lines+=['| 合计 | '+' | '.join(str(t[k]) for k in ['observed_entries','business_analyzed_partial','untouched','candidate_implementation_present','direct_original_entry_compared_partial','ownership_triage_pending'])+' |','',
    '“未触及”指尚无接受的原接口业务分析；仅有PDB名字、伪代码或猜测适配器仍属未触及。候选实现与分析是独立维度。原入口对照不代表全部泛型/输入/调用链验收。', '',
    f"原始PDB库存仍保留{len(rows)}项。分类后业务接口{len(business_rows)}项；自动实现/库转发{doc['classification_counts']['generated_reuse']}项、类型/schema合同helper {doc['classification_counts']['schema_policy']}项、待分类{doc['classification_counts']['unresolved']}项均不计入未实现业务函数。数字变化是口径纠正，不是新增恢复。", '',
    '[宏、trait与类型合同分类清单](INTERFACE_CLASSIFICATION.md)。', '',
    '[完整机器台账](tracking/shared-foundation-interfaces.json)。原类型/参数未知时不使用void()占位推定ABI。', '', '## 接口审计来源','']
lines += [f'- [{p.split("/")[-2]}]({p[:-5]}.md)' for p in INPUTS]
if review:
    lines += ['', '## 代码 review 与修复', '',
              '[本轮独立 review、修复、性能实测与功能缺口](evidence/shared-foundation-review-2026-10-13/REVIEW.md)。',
              '逐接口机器台账附有当前恢复文件的 review SHA 与同文件发现；这是代码检查记录，不将未触及接口提升为完成。']
lines += ['', '## 本批状态与后续','',
    '- utils顶层：按手写业务接口补齐；Default/fmt等先查宏/自定义归属，不以方法名创建逆向任务。',
    '- curve：CurveUCircular九接口完成本批对照；真正CurveSlice状态对象正在进行原逻辑对照，未提前计入。',
    '- asset：默认值/继承宏的字段策略另记类型合同，原scanner/loader/pipeline业务仍按接口推进。',
    '- 小模块：theme-defs字段载荷和tabloid初始化/生命周期合同独立保留，派生或依赖转发不计业务缺口。', '', '## 全部未触及业务接口','', '| 范围 | 原接口 | 依据与后续 |','|---|---|---|']
for x in business_rows:
    if x['untouched']:
        lines.append(f"| {x['group']} | `{x['interface_id']}` | [逐接口证据]({x['detailed_audit']})；{'待归属' if x['ownership_triage_pending'] else '待原业务分析'} |")
(ROOT/'INTERFACE_PROGRESS.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps(doc['statistics'],ensure_ascii=False))
