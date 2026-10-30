#!/usr/bin/env python3
"""Recount business definitions from the published filtered graph and current Rust."""
from __future__ import annotations
import argparse
from collections import Counter,defaultdict
import csv
from datetime import datetime,timezone
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[2]
CODEMAP=ROOT/'reverse-engineering/tiny-glade/tools/code-map'
sys.path.insert(0,str(CODEMAP))
from artifacts import RUNTIME,atomic,canonical,digest,load
from graph import Graph
from graph_filter import association_exclusion,normalized_name,REVISION
from source_index import scan,associate
from audit_b_module_inventory import module as legacy_module,NAMES

OUT=ROOT/'tiny-glade/reports/business/2026-10-23-module-recovery'
STEM='B-BUSINESS-CODE-MAP'
REMOVED_ROOTS={'crash_reporter_gui','feedback_reporter','geforcenow','tg_integrity_report'}
LABELS={f'M{i:02}':name for i,name in enumerate(NAMES,1)}
LABELS.update(SHARED='共享基础／渲染／宿主',M17='整游戏联合验收（无独立函数分母）')
STATES={'unique_source_candidate':'唯一源码候选','ambiguous_source_candidates':'多候选待核',
        'unresolved_source_association':'未定位待核'}
FINDING_LABELS={'confirmed_unrecovered_code':'已确认缺实现',
    'confirmed_algorithm_order_difference':'已确认原算法顺序差异（影响待核）',
    'source_wiring_gap':'已记录的直接生产接线缺口',
    'observed_acceptance_failure':'已观察验收失败（原因未定）',
    'original_contract_review':'原版合同待核','joint_validation_open':'联合验收未闭合',
    'registration_and_lifecycle_review':'注册与生命周期待核','partial_implementation_review':'局部实现边界待核',
    'scope_and_source_review':'范围与源码归属待核','dependency_contract_review':'依赖合同待核'}

def primary(path):
    # A wall-constructor callee is still M06 even if its leaf mentions stairs.
    if 'wall-constructor' in path and not any(p in path for p in ('construct_playermade_stairs','construct_platform_stairs')):
        return 'M06'
    if '/systems/wall/construction/' in path:return 'M06'
    return legacy_module(path)

def scope(name,path):
    if association_exclusion(name):return 'mechanical_excluded'
    if 'closure$' in name or 'closure_env$' in name:return 'callback_instance_not_independent_function'
    root=name.split('::',1)[0]
    if root in REMOVED_ROOTS or re.search(r'/(?:credits|third_party_licenses|report_bug|crash_pickup)(?:/|\.)',path):return 'user_removed'
    if root=='theme_fetcher' or re.search(r'/(?:daily|daily_theme)(?:/|\.)',path):return 'daily_deferred'
    if not path.startswith('crates/'):return 'definition_path_unresolved'
    if '/systems/debug/' in path or re.search(r'/gui/gui_(?:debug|error_log|link_to_official_game)\.rs$',path):return 'auxiliary_review'
    return 'business_candidate'

def capture_graph():
    g=Graph();s=g.current()
    state=g.query('MATCH (n:Snapshot {id:$s,complete:true}) RETURN n.filter_policy AS policy',{'s':s})
    if not state or state[0]['policy']!=REVISION:raise ValueError('Published graph does not use the current mechanical filter')
    symbols=g.query("""MATCH (n:Entity {snapshot:$s,kind:'Symbol',version:'B'})
        WHERE n.scope_class='project_source_attribution'
        MATCH (n)-[:LINK {kind:'DENOTES'}]->(f:Entity {snapshot:$s,kind:'Function'})
        RETURN n.key AS key,n.name AS name,n.rva AS rva,n.source_paths AS source_paths,
        n.evidence AS evidence,n.scope_class AS scope_class,f.key AS function_key ORDER BY n.key""",{'s':s},timeout=60)
    roots=sorted({r['name'].split('::',1)[0] for r in symbols})
    unknown=g.query("""MATCH (n:Entity {snapshot:$s,kind:'Symbol',version:'B'})
        WHERE n.scope_class='unknown' AND n.defining_namespace IN $roots
        MATCH (n)-[:LINK {kind:'DENOTES'}]->(f)
        RETURN n.key AS key,n.name AS name,n.rva AS rva,n.evidence AS evidence,f.key AS function_key ORDER BY n.key""",{'s':s,'roots':roots},timeout=60)
    inline=g.query("""MATCH (n:Entity {snapshot:$s,kind:'InlineInstance',version:'B'})
        WHERE split(n.name,'::')[0] IN $roots RETURN n.key AS key,n.name AS name,
        n.host_rva AS host_rva,n.evidence AS evidence,n.source_line AS source_line ORDER BY n.key""",{'s':s,'roots':roots},timeout=60)
    modules=g.query("MATCH (n:Entity {snapshot:$s,kind:'Module',version:'B'}) RETURN n.key AS key,n.name AS name ORDER BY n.name",{'s':s},timeout=60)
    binary=g.query("MATCH (n:Entity {snapshot:$s,key:'binary:B'}) RETURN n.sha AS sha256",{'s':s})[0]
    counts=g.query("""MATCH (n:Entity {snapshot:$s,kind:'Symbol',version:'B'})
        RETURN n.scope_class AS scope,count(n) AS count ORDER BY scope""",{'s':s},timeout=60)
    structures=g.query("""MATCH (n:Entity {snapshot:$s}) WHERE n.kind IN ['Function','Symbol','Module']
        RETURN n.version AS version,n.kind AS kind,count(n) AS count ORDER BY version,kind""",{'s':s},timeout=60)
    if g.current()!=s:raise RuntimeError('Publication changed during export; retain export and rerun')
    report={'snapshot':s,'filter_policy':REVISION,'binary_identity':binary,'symbols':symbols,
            'modules':modules,'unknown_owned_symbols':unknown,'inline_instances':inline,
            'symbol_scope_counts':counts,'structure_counts':structures,
            'captured_at_utc':datetime.now(timezone.utc).isoformat()}
    atomic(RUNTIME/'business-audit-graph-export.json',report)
    return report

def inline_path(evidence):
    marker='/skeleton/B/'
    return evidence.split(marker,1)[1].removesuffix('.enriched.json.gz') if marker in evidence else ''

def group_definitions(graph):
    groups={};excluded=Counter();excluded_records=[]
    def add(name,path,row,kind):
        why=scope(name,path)
        if why!='business_candidate':
            excluded[why]+=1
            excluded_records.append({'key':row['key'],'original_path':path,'reason':why,
                'name':name[:512],'name_truncated':len(name)>512})
            return
        norm=normalized_name(name);key=(path,norm)
        item=groups.setdefault(key,{'id':hashlib.sha256(canonical(key)).hexdigest()[:20],
            'module':primary(path),'original_path':path,'name':norm,'symbol_keys':set(),
            'physical_functions':set(),'inline_instances':set(),'inline_hosts':set(),'evidence':set()})
        item['evidence'].add(row['evidence'])
        if kind=='symbol':
            item['symbol_keys'].add(row['key']);item['physical_functions'].add(row['function_key'])
        else:
            item['inline_instances'].add(row['key']);item['inline_hosts'].add(f'function:B:{row["host_rva"]:x}')
    for row in graph['symbols']:
        for path in row['source_paths']:add(row['name'],path,row,'symbol')
    for row in graph['inline_instances']:add(row['name'],inline_path(row['evidence']),row,'inline')
    return list(groups.values()),excluded,excluded_records

def matching_modules(path,modules):
    return [m for m in modules if path.startswith(m['name']+'/') or path==m['name']+'.rs']

def current_sources(groups,records):
    by_name=defaultdict(list)
    for record in records:
        if not association_exclusion(record['name']):by_name[record['name']].append(record)
    for row in groups:
        candidates,method=associate(row['name'],row['original_path'],by_name.get(row['name'].rsplit('::',1)[-1],[]))
        row['association_method']=method
        row['status']='unique_source_candidate' if len(candidates)==1 else 'ambiguous_source_candidates' if candidates else 'unresolved_source_association'
        row['candidates']=[{k:v for k,v in r.items() if k not in ('body_start','body_end')} for r in candidates]
        row['body_state']=candidates[0]['state'] if len(candidates)==1 else None
        row['semantic_review']='not_established_by_source_association'

def original_exceptions(groups):
    exceptions={'country_core::resources::walls::decorator_storage::DecoratorStorage::verify_integrity':
                    'B 原体 ret，合法空操作；不列未恢复算法',
                'system_wall_constructor::debug_force_update_arches':
                    'B 14003bb80 原体 ret；不列未恢复算法',
                'country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl':
                    '非0/1模式的 not yet implemented 是原失败合同，不能据 panic 判断待恢复分支'}
    for row in groups:
        if row['name'] in exceptions:
            row['original_contract_exception']={'note':exceptions[row['name']],
                'evidence':'tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md',
                'anchor':'清单3项空／部分占位经原证复核'}

def module_statistics(groups,graph):
    summary=[]
    for key in [f'M{i:02}' for i in range(1,17)]+['SHARED','M17']:
        rows=[r for r in groups if r['module']==key]
        summary.append({'module':key,'name':LABELS[key],'groups':len(rows),
            'physical_entries':len({k for r in rows for k in r['physical_functions']}),
            'inline_only_groups':sum(not r['physical_functions'] for r in rows),
            'association_counts':dict(Counter(r['status'] for r in rows)),
            'body_review_groups':sum(r['body_state'] in ('empty_body_review','explicit_stub','partial_stub') and not r.get('original_contract_exception') for r in rows),
            'original_contract_exceptions':sum(bool(r.get('original_contract_exception')) for r in rows)})
    structural=[]
    for m in graph['modules']:
        rows=[r for r in groups if m in matching_modules(r['original_path'],[m])]
        primary_rows=[r for r in rows if max(matching_modules(r['original_path'],graph['modules']),key=lambda m:len(m['name']))['key']==m['key']]
        structural.append({**m,'inclusive_groups':len(rows),'exclusive_groups':len(primary_rows),
            'physical_entries':len({k for r in rows for k in r['physical_functions']}),
            'association_counts':dict(Counter(r['status'] for r in rows)),
            'scope':('自有定义候选' if rows else
                '外部来源库存' if m['name'].startswith('_references/') else
                '用户删除项' if any(part in m['name'] for part in ('crash-reporter-gui','feedback-reporter','geforcenow','tg-integrity-report')) else
                'Daily暂停' if m['name']=='crates/theme-fetcher' else
                '原框架/依赖边界，当前架构复用' if m['name'].startswith(('crates/rhapsody','crates/gpu-allocator')) else
                '仅来源库存，定义归属未闭合'),
            'acceptance':'not_proven_by_inventory'})
    return summary,structural

def serializable(rows):
    return [{k:sorted(v) if isinstance(v,set) else v for k,v in r.items()} for r in rows]

def evidence(path,anchor=None):
    p=ROOT/path
    row={'path':path,'available':p.is_file()}
    if p.is_file():
        raw=p.read_bytes();row['sha256']=hashlib.sha256(raw).hexdigest()
        if anchor:
            text=raw.decode('utf-8',errors='replace');at=text.find(anchor)
            row.update(anchor=anchor,line=text.count('\n',0,at)+1 if at>=0 else None,anchor_found=at>=0)
    return row

def review_findings(groups,records,graph):
    specification=load(Path(__file__).with_name('business_audit_findings.json'))
    report=[]
    native_keys=set()
    for row in groups:
        if any(c['state'] in ('explicit_stub','partial_stub','empty_body_review') for c in row['candidates']):
            native_keys.update(row['physical_functions'])
    anchors=graph.get('native_body_anchors')
    if anchors is None:
        anchors=Graph().query('''MATCH (f:Entity {snapshot:$s,kind:'Function'})
            WHERE f.key IN $keys OPTIONAL MATCH (f)-[:LINK {kind:'HAS_RANGE'}]->(r)
                -[:LINK {kind:'HAS_ASSEMBLY'}]->(a)
            RETURN f.key AS key,count(DISTINCT a) AS extracted_ranges,
                max(a.instructions) AS largest_range_instructions''',
                {'s':graph['snapshot'],'keys':sorted(native_keys)},timeout=60)
        graph['native_body_anchors']=anchors
        atomic(RUNTIME/'business-audit-native-anchors.json',{'snapshot':graph['snapshot'],'rows':anchors})
    by_key={r['key']:r for r in anchors}
    for item in specification['findings']:
        bound=[]
        for selector in item.get('functions',[]):
            matching=[r for r in records if r['path']==selector['path'] and r['name']==selector['name']]
            bound.extend([{k:v for k,v in r.items() if k not in ('body_start','body_end')} for r in matching])
        linked=[r['id'] for r in groups if any(c['path']==f['path'] and c['name']==f['name'] for c in r['candidates'] for f in bound)]
        refs=[evidence(r['path'],r.get('anchor')) for r in item.get('references',[])]
        state=item['classification']
        if item.get('requires_empty_or_stub') and (not bound or not any(r['state'] in ('empty_body_review','explicit_stub','partial_stub') for r in bound)):
            state='requires_recheck_source_changed'
        if any(not r['available'] or r.get('anchor_found') is False for r in refs):state='requires_recheck_evidence_missing'
        native=[by_key[key] for r in groups if r['id'] in linked for key in r['physical_functions'] if key in by_key]
        if state=='confirmed_unrecovered_code' and not any((r.get('largest_range_instructions') or 0)>1 for r in native):
            state='requires_recheck_original_body'
        report.append({**item,'classification':state,'current_functions':bound,'definition_groups':linked,
                       'evidence_bindings':refs,'native_body_anchors':native,'semantic_acceptance':False})
    return report,specification['module_review_boundaries']

def write_reports(payload):
    OUT.mkdir(parents=True,exist_ok=True);atomic(OUT/(STEM+'.json'),payload)
    with (OUT/(STEM+'.csv')).open('w',encoding='utf-8-sig',newline='') as handle:
        writer=csv.writer(handle,lineterminator='\n');writer.writerow(['module','definition_id','name','original_path','status','method','body_state','physical_functions','inline_instances','candidates','semantic_review'])
        for r in payload['functions']:
            writer.writerow([r['module'],r['id'],r['name'],r['original_path'],r['status'],r['association_method'],r['body_state'],
                ';'.join(r['physical_functions']),len(r['inline_instances']),';'.join(f"{c['path']}:{c['line']} [{c['owner'] or 'free'}; {c['state']}]" for c in r['candidates']),r['semantic_review']])
    total=payload['association_totals']
    lines=['# 清理后业务函数、源码关联与未闭合链路','',f"图快照：`{payload['snapshot']}`；B 原二进制 SHA：`{payload['binary_identity']['sha256']}`。",
        f"本次扫描 {len(payload['source_files'])} 个生产 Rust 文件；生成时间（UTC）：{payload['generated_at_utc']}。",'',
        f"本轮有定义来源的业务候选共 **{payload['business_definition_groups']} 组**，覆盖 **{payload['business_physical_entries']} 个物理入口**；唯一位置候选 {total.get('unique_source_candidate',0)}，多候选 {total.get('ambiguous_source_candidates',0)}，未定位 {total.get('unresolved_source_association',0)}。这些是定位和审查数量，不是已恢复算法数量。",'',
        '## 统计口径','',
        'B 为统计主线，A/C 保持辅助证据，不把三版相同业务相加。机械名称及 new/get/insert/remove 按图谱统一规则排除；闭包实例不当独立算法。用户删除项、Daily暂停项、调试辅助另列排除口径。',
        '函数组按原源码路径＋去泛型后的符号合并；物理入口和仅有内联证据的定义分别列出。一个符号组不代表一个独立算法。源码关联只是位置候选，不能作为恢复完成率。没有同名实现不直接判缺失；原版 ret／原 panic 不当待补业务。',
        'M01为台账的toolbar（当前交流常称M0），M02为地形（常称M1）；其余保留台账编号。M17没有独立函数分母。共享函数不强行按调用者重复摊入业务模块。',
        '图结构基数（含依赖／运行时／未知，不能当业务函数数）：'+', '.join(f"{r['version']} {r['kind']}={r['count']}" for r in payload['graph_structure_counts'])+'。',
        '', '## 各业务域','', '| 模块 | 业务定义组 | 物理入口 | 仅内联定义 | 唯一源码候选 | 多候选 | 未定位 | 空/占位待核 |', '|---|---:|---:|---:|---:|---:|---:|---:|']
    for r in payload['module_summary']:
        c=r['association_counts'];lines.append(f"| {r['module']} {r['name']} | {r['groups']} | {r['physical_entries']} | {r['inline_only_groups']} | {c.get('unique_source_candidate',0)} | {c.get('ambiguous_source_candidates',0)} | {c.get('unresolved_source_association',0)} | {r['body_review_groups']} |")
    lines+=['','物理入口可由多个定义／模块共享，各行物理数不能直接相加；97个结构模块的父子包含数同样不可相加。','',
        '## 有证据的业务缺口与待核对链路','',
        '确认缺代码、已观察验收失败、原合同待核和联合验收未覆盖分别记录。失败记录不自动等于缺算法；下列结论不取代运行原版或正式生产验收。']
    for f in payload['chain_findings']:
        lines+=['',f"### {f['id']} · {', '.join(f['modules'])} · {f['title']}",'',
                f"**分类：{FINDING_LABELS.get(f['classification'],f['classification'])}**。{f['finding']}",f"链路：{f['chain']}",f"下一步：{f['next_step']}"]
        for r in f['current_functions']:lines.append(f"- 当前源码：`{r['path']}:{r['line']}` · `{r['name']}` · {r['state']}")
        for e in f['evidence_bindings']:lines.append(f"- 依据：`{e['path']}"+(f":{e['line']}" if e.get('line') else '')+f"`（{'存在，已绑定SHA' if e['available'] else '当前缺失，待重取'}）")
        if f['native_body_anchors']:lines.append('- 原体图证据：'+', '.join(f"`{r['key']}`（最大已验证区间{r['largest_range_instructions']}条指令）" for r in f['native_body_anchors']))
    lines+=['','## 全部模块的审查边界','', '| 业务域 | 真实链与本轮未闭合边界 |', '|---|---|']
    for r in payload['module_review_boundaries']:lines.append(f"| {r['module']} | {r['boundary']} |")
    lines+=['','## 源码关联待核对清单','',
        '全部逐项记录见同名 CSV/JSON，含原路径、图符号/物理函数/内联证据、当前类型owner、源码SHA和关联理由。下列为每域前8项入口；完整结果不受此展示上限影响。']
    for m in payload['module_summary']:
        pending=[r for r in payload['functions'] if r['module']==m['module'] and (r['status']!='unique_source_candidate' or r['body_state']!='body_candidate') and not r.get('original_contract_exception')]
        if not pending:continue
        lines+=['',f"### {m['module']} · {len(pending)} 项定位／函数体待核",'']
        for r in pending[:8]:
            c='; '.join(f"{x['path']}:{x['line']}" for x in r['candidates'][:3]) or '无可靠位置候选'
            lines.append(f"- `{r['name']}` · `{r['original_path']}` · {r['status']} · {c}")
    lines+=['','## B版97个结构模块','', '| 模块 | 包含定义组 | 独占归属组 | 物理入口 | 唯一候选 | 多候选 | 未定位 | 范围 |', '|---|---:|---:|---:|---:|---:|---:|---|']
    for r in payload['structural_modules']:
        c=r['association_counts'];lines.append(f"| `{r['name']}` | {r['inclusive_groups']} | {r['exclusive_groups']} | {r['physical_entries']} | {c.get('unique_source_candidate',0)} | {c.get('ambiguous_source_candidates',0)} | {c.get('unresolved_source_association',0)} | {r['scope']} |")
    lines+=['','## 归属未知与排除','',f"排除记录口径：`{json.dumps(payload['excluded_counts'],ensure_ascii=False)}`。这里计的是符号/内联来源记录，不与去重定义组混算。",
        f"自有命名空间无源码归属符号共 {payload['unknown_source_summary']['symbols']} 条，其中 {payload['unknown_source_summary']['without_known_definition_name']} 条无法用已有定义名定位，{payload['unknown_source_summary']['without_known_physical_entry']} 条也没有已归属业务定义的物理入口。完整图key/RVA见JSON，不计作确认缺业务。",
        f"上述无已知定义名的记录去重后为 {payload['unknown_source_summary']['unrepresented_normalized_names']} 个名称；没有已归属业务定义的物理入口去重为 {payload['unknown_source_summary']['unrepresented_physical_entries']} 个。两项有交叉，不相加作缺失总数。",
        '图中的公共依赖、运行时及其余未知归属不纳入自有业务分母。结构模块0项表示当前图未给出可用的自有定义来源，不能解释为已恢复或无业务。',
        '',f"当前源码扫描期间变化：{len(payload['source_changes_during_scan'])} 文件。清单只绑定本次文件SHA；后续修改需重跑受影响统计与证据检查。",
        '', '本次执行只读图与生产源码，更新统计和审查清单；未修改生产Rust、运行游戏或把已有测试receipt提升为当前验收。']
    (OUT/(STEM+'.md')).write_text('\n'.join(lines)+'\n',encoding='utf-8')

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--graph-export',type=Path);args=p.parse_args()
    graph=load(args.graph_export) if args.graph_export else capture_graph()
    if graph.get('filter_policy')!=REVISION:raise ValueError('Graph export filter revision mismatch')
    baseline=load(ROOT/'reverse-engineering/tiny-glade/tracking/baselines.json')['baselines']['B']
    if graph.get('binary_identity',{}).get('sha256')!=baseline['binary']['sha256']:
        raise ValueError('Graph export does not bind the current B binary')
    records,files=scan(ROOT);groups,excluded,excluded_records=group_definitions(graph)
    current_sources(groups,records);original_exceptions(groups)
    groups.sort(key=lambda r:(r['module'],r['original_path'],r['name']))
    summary,structural=module_statistics(groups,graph)
    findings,boundaries=review_findings(groups,records,graph)
    if not args.graph_export:atomic(RUNTIME/'business-audit-graph-export.json',graph)
    named={r['name'] for r in groups};physical={key for r in groups for key in r['physical_functions']}
    unknown=[{**r,'normalized_name':normalized_name(r['name']),
              'has_known_definition_name':normalized_name(r['name']) in named,
              'has_known_physical_entry':r['function_key'] in physical} for r in graph['unknown_owned_symbols']
              if not association_exclusion(r['name']) and r['name'].split('::',1)[0] not in REMOVED_ROOTS]
    changes=[r['path'] for r in files if not (ROOT/r['path']).is_file() or digest(ROOT/r['path'])!=r['sha256']]
    payload={'schema':1,'snapshot':graph['snapshot'],'filter_policy':REVISION,
        'binary_identity':graph['binary_identity'],'pdb_sha256':baseline['pdb']['sha256'],
        'graph_structure_counts':graph.get('structure_counts',[]),
        'graph_symbol_scope_counts':graph.get('symbol_scope_counts',[]),
        'generated_at_utc':datetime.now(timezone.utc).isoformat(),
        'repo_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
        'graph_export_sha256':digest(args.graph_export or RUNTIME/'business-audit-graph-export.json'),
        'implementation_sha256':{str(path.relative_to(ROOT)):digest(path) for path in (
            Path(__file__),CODEMAP/'source_index.py',CODEMAP/'graph_filter.py',
            ROOT/'tiny-glade/tools/business_audit_findings.json')},
        'source_index_sha256':hashlib.sha256(canonical(files)).hexdigest(),
        'business_definition_groups':len(groups),'business_physical_entries':len(physical),
        'module_summary':summary,'structural_modules':structural,'excluded_counts':dict(excluded),
        'excluded_records':excluded_records,'association_totals':dict(Counter(r['status'] for r in groups)),
        'functions':serializable(groups),'source_files':files,'source_changes_during_scan':changes,
        'unknown_source_symbols':unknown,'unknown_source_summary':{'symbols':len(unknown),
            'without_known_definition_name':sum(not r['has_known_definition_name'] for r in unknown),
            'without_known_physical_entry':sum(not r['has_known_physical_entry'] for r in unknown),
            'unrepresented_normalized_names':len({r['normalized_name'] for r in unknown if not r['has_known_definition_name']}),
            'unrepresented_physical_entries':len({r['function_key'] for r in unknown if not r['has_known_physical_entry']})},
        'chain_findings':findings,'module_review_boundaries':boundaries,'semantic_complete':False,
        'source_association_is_behavioral_acceptance':False}
    write_reports(payload)
    print(json.dumps({'ok':not changes,'snapshot':graph['snapshot'],'definition_groups':len(groups),
        'physical_entries':len(physical),'source_files':len(files),'structural_modules':len(structural),
        'findings':dict(Counter(f['classification'] for f in findings)),
        'report':str(OUT/(STEM+'.md'))},ensure_ascii=False))
    if changes:raise SystemExit(2)

if __name__=='__main__':main()
