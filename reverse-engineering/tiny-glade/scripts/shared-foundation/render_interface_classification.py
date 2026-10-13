"""Render generated-code reuse and per-declaration schema queues separately."""
import json
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
master = json.loads((ROOT/'tracking/shared-foundation-interfaces.json').read_text(encoding='utf-8'))
classification = json.loads((ROOT/'tracking/shared-foundation-interface-classification.json').read_text(encoding='utf-8'))
labels = {'generated_reuse':'自动实现 / 原依赖转发', 'schema_policy':'类型 / schema合同', 'unresolved':'归属待分类'}
schema_groups = defaultdict(list)
audits = {}
for item in master['inputs']:
    if item['path'].endswith('interface-coverage.json'):
        data = json.loads((ROOT/item['path']).read_text(encoding='utf-8'))
        audits[item['path']] = {x.get('interface_id',x.get('canonical_key',x.get('id'))):x for x in data['interfaces']}

for row in master['interfaces']:
    if row['classification']['classification'] != 'schema_policy':
        continue
    raw = audits[row['detailed_audit']][row['detailed_interface_key']]
    records = raw.get('pdb_provenance',raw.get('pdb_records',raw.get('PDB_records',[])))
    source = raw.get('original_source') or next(iter(raw.get('original_sources',[])),None)
    if not source:
        source = next((r['source'].removeprefix('skeleton/B/') for r in records if r.get('source')),row['source_module'])
    lines = set()
    for record in records:
        line = (record.get('skeleton_metadata') or {}).get('source_line') or record.get('original_source_line') or record.get('source_line')
        if line:
            lines.add(str(line))
    group = source+'@'+','.join(sorted(lines))
    schema_groups[group].append(row)

lines = ['# 宏、trait与类型合同分类', '',
         '2026-10-13 按用户指出的统计问题纠正。原609项库存保留，主业务接口表剔除自动实现/原库转发、schema生成helper和待分类项。没有删除原证据，也没有把剔除条目标成已经恢复。', '',
         '| 类别 | 原库存条目 | 处理 |', '|---|---:|---|',
         f'| 自有业务接口 | {master["classification_counts"]["custom_business"]} | 按实际算法/业务恢复与验证 |',
         f'| 自动实现/原库转发 | {master["classification_counts"]["generated_reuse"]} | derive或直接复用依赖，不逆向生成函数 |',
         f'| 类型/schema helper | {master["classification_counts"]["schema_policy"]} | 合并为字段、默认值、属性、错误模板等合同 |',
         f'| 待分类 | {master["classification_counts"]["unresolved"]} | 单独调查归属，不算已确认未实现业务函数 |', '',
         '分类根据PDB定义源行、同源派生族、原调用/指令和已恢复类型合同。只有方法名叫default/fmt/eq不是自动实现的证明。generated_reuse还包含已经确认的标准字段操作与依赖转发，不声称作者一定写了derive。', '',
         '自定义例外仍保留：Transform::Default的identity、Timer初始滤波状态、CurveU反序列化的new/clamp策略、纹理具体默认值和真正的继承/加载策略。', '',
         '## 合并后的类型合同队列', '',
         '以下按原定义源文件及声明行合并helper。声明位置分组用于去重，不冒充已经确认的作者类型名；缺字段/default/serde属性仍是工作。', '',
         '| 定义声明组 | helper条目数 | 待恢复的合同依据 |', '|---|---:|---|']
for group, rows in sorted(schema_groups.items()):
    reasons = list(dict.fromkeys(x['classification']['reason'] for x in rows))
    lines.append(f'| `{group}` | {len(rows)} | '+ '; '.join(reasons).replace('|','\\|').replace('\n',' ')+' |')
lines += ['', '另保留已确认自动生成的serde族对应的类型合同，避免移除生成代码时丢掉字段/格式工作：', '']
for group in classification['schema_policy_groups']:
    lines.append(f'- `{group["id"]}`：{group["reason"]}')
lines += ['', '## 完整分类清单', '']
for category in labels:
    lines += [f'### {labels[category]}', '', '| 原记录 | 分类依据 | 证据 |', '|---|---|---|']
    for row in master['interfaces']:
        decision = row['classification']
        if decision['classification'] == category:
            reason = decision['reason'].replace('|','\\|').replace('\n',' ')
            evidence = decision.get('classification_report',row['detailed_audit'])
            lines.append(f'| `{row["interface_id"]}` | {reason} | [逐项证据]({evidence}) |')
    lines.append('')
lines += ['[业务接口进度](INTERFACE_PROGRESS.md)；[真正无映射实现的业务接口](UNRECOVERED_INTERFACES.md)；[机器分类表](tracking/shared-foundation-interface-classification.json)。']
(ROOT/'INTERFACE_CLASSIFICATION.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(f'Rendered {len(schema_groups)} source-declaration schema groups plus {len(classification["schema_policy_groups"])} retained derived-schema obligations.')
