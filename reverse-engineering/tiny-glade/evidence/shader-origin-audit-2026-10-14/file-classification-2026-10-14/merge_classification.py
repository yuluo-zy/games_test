import collections
import csv
import hashlib
import json
import posixpath
import sys
from pathlib import Path

sys.stdout.reconfigure(encoding='utf-8')
HERE = Path(__file__).resolve().parent
AUDIT = HERE.parent
RECOVERED = Path('D:/game/tiny-glade/shaders')
LABELS = {
    'confirmed_upstream':'已确认包含上游核心',
    'game_modified':'上游同源并经游戏改写',
    'likely_derived':'疑似上游派生',
    'game_specific':'游戏专属数据与规则',
    'game_adapter':'游戏接入或组合层',
    'generic_unresolved':'通用代码，具体来源未定',
    'unreviewed':'尚未审阅',
    'recovery_support_or_unmapped':'恢复支持文件或原路径未映射',
}
CONF = {'high':3,'medium':2,'low':1}
RANK = {'confirmed_upstream':7,'game_modified':6,'likely_derived':5,'game_specific':4,'game_adapter':3,'generic_unresolved':2,'unreviewed':0}

def confidence_rank(value):
    # Some analysts qualify confidence by the scope proved, rather than the
    # entire file. Preserve that text while comparing its stated level.
    if value == 'medium_high':
        return 2.5
    if isinstance(value,str):
        for level,rank in CONF.items():
            if value == level or value.startswith(level+'_'):
                return rank
    return 0

def canonical(source):
    return posixpath.normpath(source.replace('\\','/'))

def recovered_key(source):
    key = canonical(source)
    if key.endswith('.hlsl.inc'):
        key = key[:-4]
    return key

def string(value):
    if isinstance(value,str):
        return value
    return json.dumps(value,ensure_ascii=False,separators=(',',':')) if value is not None else ''

def main():
    originals = json.loads((AUDIT/'original-source-dependencies.json').read_text('utf-8'))
    modules = json.loads((AUDIT/'original-shader-inventory.json').read_text('utf-8'))
    by_source = collections.defaultdict(list)
    observations = collections.defaultdict(list)
    for row in originals:
        by_source[canonical(row['source'])].append(row)
    for owner in ['common','game','render','remaining']:
        path = HERE/owner/'classification.json'
        if not path.exists():
            continue
        payload = json.loads(path.read_text('utf-8-sig'))
        items = payload['files'] if isinstance(payload,dict) else payload
        for item in items:
            item = dict(item)
            item['owner'] = owner
            item['classification_file'] = str(path.relative_to(HERE)).replace('\\','/')
            observations[canonical(item['source'])].append(item)
    known_cores = {
        'shaders/inc/fidelityfx/ffx_a.h':'AMD FidelityFX SPD portability header',
        'shaders/inc/fidelityfx/ffx_spd.h':'AMD FidelityFX SPD downsampler',
        'shaders/ssgi/XeGTAO.hlsl.inc':'Intel XeGTAO algorithm include',
        'shaders/inc/reservoir.hlsl':'Kajiya Reservoir helper',
    }
    special_rt_modules = {
        'compiled-shaders/_gi_trace_gi_candidates.cs.hlsl_cs_main_0.bin',
        'compiled-shaders/_primary_rt_debug.cs.hlsl_cs_main_0.bin',
        'compiled-shaders/_water_deferred_rt.cs.hlsl_cs_main_0.bin',
        'compiled-shaders/_water_ice_deferred_rt.cs.hlsl_cs_main_0.bin',
    }
    row_index = []
    for source,raw_rows in sorted(by_source.items()):
        candidates = observations[source]
        selected = max(candidates,key=lambda x:(confidence_rank(x.get('confidence')),RANK.get(x['status'],0))) if candidates else {
            'source':source,'status':'unreviewed','origin':'尚未审阅，不能默认为原创','confidence':'low',
            'scope':'源路径存在，尚未对来源或游戏专属规则作分类','proof':None,'limitations':[], 'owner':None}
        record = dict(selected)
        record.update({'source':source,'status_label':LABELS.get(record['status'],record['status']),
                       'raw_source_aliases':[r['source'] for r in raw_rows],
                       'all_classification_observations':candidates,
                       'consumer_pipelines':sorted(set(f for r in raw_rows for f in r['files']))})
        record['consumer_pipeline_count'] = len(record['consumer_pipelines'])
        if source in known_cores:
            record['verified_fragment'] = known_cores[source]
        relevant = [m for m in modules if m['file'] in record['consumer_pipelines']]
        own_sites = [(m,s,site) for m in relevant for s in m['stages'] for site in s['debug_line_files'] if site['file'] and canonical(site['file'])==source]
        record['source_emission'] = {'stage_count':len(own_sites), 'line_site_count':sum(site['line_site_count'] for _,_,site in own_sites),
                                   'source_lines':sorted(set(n for _,_,site in own_sites for n in site['unique_lines']))}
        record['verified_upstream_regions_in_consumer_modules'] = []
        # These are module-context regions, not a claim that a helper file itself calls all of them.
        if source.endswith(('.cs.hlsl','.raster.hlsl')):
            groups = collections.defaultdict(set)
            for m in relevant:
                for s in m['stages']:
                    for site in s['debug_line_files']:
                        key = canonical(site['file']) if site['file'] else ''
                        if key in known_cores:
                            groups[known_cores[key]].add(m['file'])
                if m['file'] in special_rt_modules:
                    groups['GPU-Raytracer BVH8 traversal fragment'].add(m['file'])
            record['verified_upstream_regions_in_consumer_modules'] = [{'component':name,'original_bins':sorted(files)} for name,files in sorted(groups.items())]
        path = RECOVERED/recovered_key(source).removeprefix('shaders/')
        record['recovered_files'] = [str(path)] if path.exists() else []
        row_index.append(record)
    keys = {recovered_key(r['source']):r for r in row_index}
    recovered_rows = []
    for path in sorted(RECOVERED.rglob('*.hlsl')):
        key = 'shaders/'+path.relative_to(RECOVERED).as_posix()
        linked = keys.get(key)
        if not linked:
            # Reconstructed support paths may omit .inc before .hlsl.
            candidates = [r for r in row_index if recovered_key(r['source']).replace('.inc.hlsl','.hlsl')==key]
            linked = candidates[0] if len(candidates)==1 else None
        recovered_rows.append({'recovered_file':str(path),'relative_file':path.relative_to(RECOVERED).as_posix(),
                               'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
                               'original_source':linked['source'] if linked else None,
                               'status':linked['status'] if linked else 'recovery_support_or_unmapped',
                               'status_label':linked['status_label'] if linked else LABELS['recovery_support_or_unmapped'],
                               'confidence':linked['confidence'] if linked else 'low',
                               'origin':linked['origin'] if linked else '当前文件名没有唯一映射到发行包 source_dependencies；不推定源码归属',
                               'verified_upstream_regions_in_consumer_modules':linked['verified_upstream_regions_in_consumer_modules'] if linked else []})
    summary = {'original_recorded_paths':len(originals),'canonical_original_paths':len(row_index),
               'recovered_hlsl_files':len(recovered_rows),
               'status_counts':dict(collections.Counter(r['status'] for r in row_index)),
               'unreviewed_sources':[r['source'] for r in row_index if r['status']=='unreviewed'],
               'unmapped_recovered_files':[r['relative_file'] for r in recovered_rows if not r['original_source']],
               'classification_scope':'source regions/contracts/adaptation, not a certification of every line from scratch',
               'known_core_fragment_map':known_cores}
    for name,payload in [('original-file-classification.json',row_index),('recovered-file-classification.json',recovered_rows),('classification-summary.json',summary)]:
        (HERE/name).write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n','utf-8')
    with (HERE/'file-index.csv').open('w',encoding='utf-8-sig',newline='') as f:
        writer=csv.DictWriter(f,fieldnames=['source','recovered_files','status_label','confidence','origin','scope','verified_fragment','consumer_pipeline_count','actual_source_stage_count','upstream_regions','classification_file'])
        writer.writeheader()
        for r in row_index:
            writer.writerow({k:string(r.get(k)) for k in ['source','recovered_files','status_label','confidence','origin','scope','verified_fragment','consumer_pipeline_count','classification_file']} |
                            {'actual_source_stage_count':r['source_emission']['stage_count'],'upstream_regions':'; '.join(p['component'] for p in r['verified_upstream_regions_in_consumer_modules'])})
    lines=['# Shader 逐文件来源清单','',
           '这里的“游戏专属”指原作游戏的数据、规则、参数和接入实现；不是每行代码/数学算法都从零原创的证明。确认上游的条目也只对说明的核心段负责，不把整个消费者 shader 判成上游源码。','',
           f'原作 {len(originals)} 个记录路径合并为 {len(row_index)} 个规范化路径（去除 eval/../ 等别名）；当前恢复目录 {len(recovered_rows)} 个 HLSL。原始证据来自发行包，完整性验证和详细分析见 [README.md](README.md)。','',
           '分类是来源证据等级与实现范围，不是开源代码行数占比。','']
    for status,label in LABELS.items():
        group=[r for r in row_index if r['status']==status]
        if not group: continue
        lines.extend([f'## {label}（{len(group)} 个路径）','','| 原作源路径 | 来源/范围 | 置信度 |','|---|---|---|'])
        for r in group:
            context='；消费者另含 '+', '.join(p['component'] for p in r['verified_upstream_regions_in_consumer_modules']) if r['verified_upstream_regions_in_consumer_modules'] else ''
            origin=string(r['origin']).replace('|','\\|')
            lines.append(f"| `{r['source']}` | {origin}{context} | {r['confidence']} |")
        lines.append('')
    (HERE/'FILE_INDEX.md').write_text('\n'.join(lines)+'\n','utf-8')
    print(json.dumps(summary,ensure_ascii=False,indent=2))

if __name__=='__main__':
    main()
