"""Compare the requested functional catalog against current seven-library code."""
from collections import Counter
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
REFERENCE = 'tracking/foundation-business-functions.json'
REFERENCE_MD = 'FOUNDATION_BUSINESS_FUNCTIONS.md'
BASE = 'evidence/foundation-business-comparison-2026-10-13/'
OWNERS = ('small-modules','utils-base','geometry-curves','assets')
LIBRARIES = ('utils','tabloid','paths','lang-lib','theme-defs','asset','asset-pipe')
STATES = {
    'implemented_scoped':'已有限定实现',
    'partial_missing_logic':'部分实现，仍缺库内逻辑',
    'not_implemented':'尚无对应业务实现',
    'integration_pending':'库外游戏状态接入未完成',
    'contract_only':'数据契约',
    'out_of_baseline':'C版范围，排除B主线',
}


def items(value):
    if value is None:
        return []
    return value if isinstance(value,list) else [value]


def hash_file(relative):
    return hashlib.sha256((ROOT/relative).read_bytes()).hexdigest()


catalog = json.loads((ROOT/REFERENCE).read_text(encoding='utf-8'))
reference = {x['id']:(lib,x) for lib in catalog['libraries'] if lib['name'] in LIBRARIES for x in lib['features']}
rows = []
inputs = [{'path':p,'sha256':hash_file(p)} for p in (REFERENCE_MD,REFERENCE)]
sources = {}


def record_hashes(value):
    if isinstance(value,list):
        for child in value:
            record_hashes(child)
    elif isinstance(value,dict):
        path = value.get('path',value.get('file',value.get('source')))
        if isinstance(path,str) and path.startswith('reconstruction/shared-foundation/') and value.get('sha256'):
            assert path not in sources or sources[path]==value['sha256'], f'Conflicting snapshot: {path}'
            sources[path]=value['sha256']
        for key,child in value.items():
            if key.startswith('reconstruction/shared-foundation/') and isinstance(child,str) and len(child)==64:
                assert key not in sources or sources[key]==child
                sources[key]=child
            elif isinstance(child,(dict,list)):
                record_hashes(child)


for owner in OWNERS:
    path=BASE+owner+'.json'
    data=json.loads((ROOT/path).read_text(encoding='utf-8'))
    inputs.append({'path':path,'sha256':hash_file(path)})
    for key in ('current_sources','source_hashes','source_sha256','source_hash_snapshot','source_inventory'):
        record_hashes(data.get(key))
    for entry in data['features']:
        key=entry.get('feature_id',entry.get('id'))
        lib,original=reference[key]
        state=entry.get('current_status',entry.get('comparison_status',entry.get('status')))
        assert state in STATES, (key,state)
        mapping=entry.get('source_mapping',entry.get('current_implementation',entry.get('existing_implementation',entry.get('implementation_files',[]))))
        record_hashes(mapping)
        rows.append({'feature_id':key,'library':lib['name'],'name':original['name'],
                     'baseline':entry.get('baseline','B Windows; A/C not promoted'),
                     'reference_status':original['status'],'reference_remaining':original['remaining'],
                     'current_status':state,
                     'implemented_scope':items(entry.get('implemented_behavior',entry.get('implemented_scope',entry.get('implemented_library_logic',entry.get('existing_logic'))))),
                     'source_mapping':mapping,
                     'missing_library_logic':items(entry.get('missing_logic',entry.get('missing_library_logic'))),
                     'unresolved_contracts':items(entry.get('unresolved_library_contracts')),
                     'validation_gaps':items(entry.get('validation_gaps',entry.get('verification_gaps'))),
                     'consumer_gaps':items(entry.get('consumer_gaps',entry.get('consumer_wiring_gaps'))),
                     'reference_corrections':items(entry.get('notes',entry.get('documentation_correction',entry.get('document_remaining_assessment')))),
                     'original_interfaces':entry.get('original_interfaces',[]),
                     'evidence':entry.get('evidence',[]),'detailed_comparison':path,
                     'full_original_feature_accepted':False})
assert len(rows)==len({x['feature_id'] for x in rows})==len(reference)==98
assert {x['feature_id'] for x in rows}==set(reference)
for path,expected in sources.items():
    assert hash_file(path)==expected, f'Source changed after comparison: {path}'
rows.sort(key=lambda x:(LIBRARIES.index(x['library']),int(x['feature_id'].split(':')[1])))
statistics=dict(Counter(x['current_status'] for x in rows))
modules={name:dict(Counter(x['current_status'] for x in rows if x['library']==name)) for name in LIBRARIES}
doc={'schema_version':1,'baseline':'B primary','reference':REFERENCE_MD,'scope':list(LIBRARIES),
     'statistics':statistics,'libraries':modules,'features':rows,'inputs':inputs,'current_sources':sources,
     'policy':'Functional closures, not names or macro/helper counts. Distinguish absent library logic from missing original comparison and external game wiring. Scoped implementation is not complete original acceptance; no completion percentage.',
     'source_modified':False,'tests_rerun':False,'raw_interface_count_is_not_functional_workload':True}
(ROOT/'tracking/shared-foundation-business-comparison.json').write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')


def cell(value):
    return '；'.join(str(x.get('function',x)) if isinstance(x,dict) else str(x) for x in items(value)).replace('|','\\|').replace('\n',' ')


lines=['# 当前七个基础库：业务功能与源码对照','',
       '[原功能清单](FOUNDATION_BUSINESS_FUNCTIONS.md)。本次逐项核对范围内98行，4行属于C版、2行仅为数据契约；不使用函数名、宏或getter个数推导工作量。主基线为B Windows。','',
       '“已有限定实现”表示该库内已有可执行策略，并未宣称全部原泛型、异常、原图或游戏调用链已经验收。具体缺代码、规则未确定、缺原侧验证和库外接线分列。','',
       '| 库 | 已有限定实现 | 部分逻辑缺失 | 无业务实现 | 接入缺口 | 数据契约 | C版范围 |',
       '|---|---:|---:|---:|---:|---:|---:|']
order=list(STATES)
for name in LIBRARIES:
    lines.append('| '+name+' | '+' | '.join(str(modules[name].get(s,0)) for s in order)+' |')
lines.append('| 合计 | '+' | '.join(str(statistics.get(s,0)) for s in order)+' |')
lines += ['', '## 明确没有实现的业务闭包','']
for row in rows:
    if row['current_status']=='not_implemented':
        lines.append(f'- `{row["feature_id"]}` **{row["name"]}**：{cell(row["missing_library_logic"])}')
lines += ['', '## 已有代码但仍缺库内逻辑','', '| 功能 | 明确缺失 |','|---|---|']
for row in rows:
    if row['current_status']=='partial_missing_logic':
        lines.append(f'| `{row["feature_id"]}` {row["name"]} | {cell(row["missing_library_logic"])} |')
lines += ['', 'asset:01与asset:03共享RT/audio/trigger深层默认/最终化规则缺口，应作为一个类型/默认策略任务，不按两行或每个生成helper重复计工作量。','',
          '## 需要纠正的旧描述','',
          '- CurveSlice的视图、端点迭代、测长、扩展与remap已有候选代码；剩余原具体策略/调用方验证不等于整个功能没写。',
          '- prefab递归扫描、读取文件→RON→错误上下文及依赖登记已有等价组合代码；没有同名do_load包装不能算未实现。',
          '- 周期归一化、反向LoopRange迭代、样条求解、连续区间裁剪和墙曲线nearest-u/外推不能因旧remaining文字重复列为未写。',
          '- 全部九项B纹理生产策略都有代码；缺原始源图是生产链差分验证限制，不能算九项功能没实现。',
          '- 原接口194条“无已审查映射”是符号/对应关系库存，不能解释成194个业务功能缺代码；后续按本功能闭包表推进。','',
          '## 完整逐功能对照','']
for name in LIBRARIES:
    lines += [f'### {name}','', '| ID / 功能 | 当前状态与已有逻辑 | 具体缺失库内逻辑 | 验证缺口 | 库外接线 | 代码 / 依据 |','|---|---|---|---|---|---|']
    for row in rows:
        if row['library']!=name:
            continue
        mapping=[]
        for m in row['source_mapping']:
            if isinstance(m,dict):
                p=m.get('path',m.get('file'));fn=m.get('function',m.get('functions',m.get('key_functions',[])))
                if p:
                    mapping.append(f'[{Path(p).name}]({p}) `{cell(fn)}`')
        evidence=f'[逐项证据]({row["detailed_comparison"]})'
        absent=cell(row['missing_library_logic']) or ('C版另立基线' if row['current_status']=='out_of_baseline' else '本次未发现新的确定库内缺代码；类型/边界验证不因此完成。')
        lines.append(f'| `{row["feature_id"]}` {row["name"]} | {STATES[row["current_status"]]}：{cell(row["implemented_scope"])} | {absent} | {cell(row["validation_gaps"])} | {cell(row["consumer_gaps"])} | {"；".join(dict.fromkeys(mapping))} {evidence} |')
    lines.append('')
lines += ['只读对照：当前源码SHA已核对，未修改运行代码或重新执行既有测试。原85测试/局部原码证据保持其原范围，不据此提升全功能验收。','',
          '[机器功能对照表](tracking/shared-foundation-business-comparison.json)；[宏与类型分类](INTERFACE_CLASSIFICATION.md)。']
(ROOT/'FOUNDATION_BUSINESS_COMPARISON.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps({'feature_rows':len(rows),'states':statistics,'verified_source_files':len(sources)},ensure_ascii=False))
