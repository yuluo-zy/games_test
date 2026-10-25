"""合并独立研究的 PDB 范围，不以调用次数或符号别名冒充函数覆盖。"""
from pathlib import Path
import json
root = Path('D:/game/reverse-engineering/tiny-glade')
index = {}
sources = [root/'evidence/pdb-structure/procedures.json']+sorted((root/'evidence').glob('roof-*/procedures.json'))+[root/'evidence/scope-priorities/procedures.json']
for file in sources:
    data = json.loads(file.read_text(encoding='utf-8'))
    rows = data if isinstance(data,list) else data['procedures']
    for p in rows:
        if not p.get('va'):
            continue
        entry = index.setdefault(p['va'],dict(va=p['va'],names=[],ranges=[],pdb_sources=[],verification=[]))
        if p['name'] not in entry['names']:
            entry['names'].append(p['name'])
        item = {'code_size':p['code_size'],'end_exclusive':p['end_exclusive']}
        if item not in entry['ranges']:
            entry['ranges'].append(item)
        rel = file.relative_to(root).as_posix()
        if rel not in entry['pdb_sources']:
            entry['pdb_sources'].append(rel)
verification = [
 ('evidence/native/window-aspect-verification.json',[('0x140949c20','full_local_function','cases')]),
 ('evidence/native/roof-state-verification.json',[('0x1408e31c0','state_writes','constructor_cases'),('0x1408e3250','boolean_return','predicate_pair_cases'),('0x1408e33b0','boolean_return','predicate_pair_cases')]),
 ('evidence/roof-surface/verification.json',[('0x1408e4920','section_output_current_host_crt','section'),('0x1408e3260','non_panic_height','height'),('0x1408e3c30','eave_scalar','eave')]),
 ('evidence/roof-ridge/verification.json',[('0x1408e2700','calc_numeric','calc'),('0x1408e30f0','non_panic_inverse','inverse'),('0x1408e2690','from_numeric','from'),('0x1408e3780','tip_offset_numeric','tip'),('0x1408e30b0','tip_center_numeric','center'),('0x1408e2750','world_point_and_segment','world')]),
 ('evidence/scope-priorities/wall-rules-verification.json',[('0x140ac7f30','resolved_numeric_suffix_only','clamp'),('0x140b2a620','cached_max_value_and_missing_panic','max'),('0x140b2a680','cached_gable_max_and_missing_panic','gable'),('0x140b2a6e0','flat_roof_cached_height_and_missing_panic','flat')]),
 ('evidence/source-edits/verification.json',[('0x1421ab7c0','resolved_roof_event_suffix_and_numeric_notifications','cases')]),
]
for path, funcs in verification:
    file=root/path
    if not file.exists():
        continue
    report=json.loads(file.read_text(encoding='utf-8'))
    if not report.get('ok'):
        continue
    assert report['source_sha256']=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
    for va,scope,count_key in funcs:
        assert va in index,f'Verified entry has no PDB range: {va}'
        count=report.get(count_key,report.get('counts',{}).get(count_key))
        index[va]['verification'].append(dict(report=path,scope=scope,cases=count,limitations=report.get('limitations',[])))
rows=sorted(index.values(),key=lambda p:int(p['va'],16))
report=dict(source_sha256='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
  unique_entries_with_pdb_range=len(rows),entries_with_verified_local_scope=sum(bool(x['verification']) for x in rows),
  whole_game_function_denominator=None,functions=rows,
  policy='每个入口只计一次，别名保留。局部已验证的限定范围不等于该系统或完整游戏完成；尚无全游戏函数分母。')
(root/'tracking/function-index.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({k:report[k] for k in ['unique_entries_with_pdb_range','entries_with_verified_local_scope']}))
