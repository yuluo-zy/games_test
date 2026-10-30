"""Evidence-linked module review queue. Structural evidence never closes semantics."""
import collections
import json
from pathlib import Path
import re
from artifacts import RUNTIME, PROJECT, WORKSPACE, atomic, db, digest, load, module_for

def module_reviews():
    con=db(True)
    modules=collections.defaultdict(lambda:{'entries':set(),'records':set(),'sources':set(),'unknowns':set()})
    for row in con.execute("SELECT entry,record,name,sources FROM symbols WHERE version='B' AND record LIKE 'pdb:%'"):
        paths=json.loads(row['sources'])
        for path in paths:
            m=modules[module_for(path)]
            m['entries'].add(row['entry']);m['records'].add(row['record']);m['sources'].add(path)
    tracking=load(PROJECT/'tracking/module-coverage.json')
    for record in tracking['modules']:
        if record.get('source_counts',{}).get('B',0):
            module=record['path'].removesuffix('/src')
            modules[module]['tracking_status']=record['source_status']
            # Populate tracked business submodules with their real source-attributed entries.
            for row in con.execute("SELECT entry,record,sources FROM symbols WHERE version='B' AND record LIKE 'pdb:%'"):
                matches=[p for p in json.loads(row['sources']) if p.startswith(module+'/')]
                if matches:
                    modules[module]['entries'].add(row['entry']);modules[module]['sources'].update(matches)
                    modules[module]['records'].add(row['record'])
    chain=load(Path(__file__).with_name('chain.json'))
    reviewed_functions={e['source'] for e in chain['edges'] if e['kind']=='HAS_CONTRACT' and e['status']=='confirmed'}
    reports=[]
    for name,m in sorted(modules.items()):
        partial=[f for f in reviewed_functions if int(f.rsplit(':',1)[1],16) in m['entries']]
        excluded=m.get('tracking_status')=='dependency_boundary'
        reports.append({'module':name,'version':'B','identified_entries':len(m['entries']),
             'pdb_symbol_records':len(m['records']),'source_files':sorted(m['sources']),
             'source_mapping_evidence':'PDB source attribution; not exclusive runtime ownership',
             'prior_inventory_status':m.get('tracking_status','not_listed'),
             'semantic_status':'dependency_boundary_needs_policy_review' if excluded else 'partially_reviewed' if partial else 'unreviewed',
             'reviewed_chain_functions':sorted(partial),'responsibility':'unknown until evidence review',
             'required_review':['entry and exported contracts','producer -> authoritative state -> consumer',
                                'registration and scheduling','branch and failure contracts',
                                'lifecycle, persistence/rendering side effects','current implementation and validation binding'],
             'accepted':False})
    con.close()
    result={'snapshot_basis':load(RUNTIME/'inventory.json')['snapshot'],'modules':reports,
            'all_semantics_complete':False,'accepted_modules':0,
            'authority':str(WORKSPACE/'tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md'),
            'scope':'Review queue derived from source attribution; not automatic semantic decomposition'}
    atomic(RUNTIME/'module-reviews.json',result)
    return {'modules':len(reports),'accepted_modules':0,'all_semantics_complete':False,
            'path':str(RUNTIME/'module-reviews.json')}

def byte_coverage():
    from artifacts import baseline,pack_module
    con=db(True);b=baseline('B');pe=pack_module().PE(b['binary']['path'])
    intervals=[]
    for row in con.execute("SELECT start,end FROM assemblies WHERE version='B' AND status='ok' ORDER BY start,end"):
        a,z=row['start'],row['end']
        if intervals and a<=intervals[-1][1]:intervals[-1][1]=max(z,intervals[-1][1])
        else:intervals.append([a,z])
    total=covered=0;uncovered=[]
    for section in pe.sections:
        if not section['executable']:continue
        start=section['rva'];end=start+min(section['virtual_size'],section['raw_size']);cursor=start;total+=end-start
        for a,z in intervals:
            a,z=max(start,a),min(end,z)
            if z<=a:continue
            covered+=z-a
            if a>cursor:uncovered.append([cursor,a])
            cursor=max(cursor,z)
        if cursor<end:uncovered.append([cursor,end])
    con.close()
    result={'B_executable_file_backed_virtual_bytes':total,'B_unique_fully_decoded_range_bytes':covered,
            'B_uncovered_executable_bytes':total-covered,'uncovered_intervals':uncovered,
            'scope':'Undecoded regions can contain padding/data; not a function or semantic recovery percentage'}
    atomic(RUNTIME/'byte-coverage.json',result)
    return {k:v for k,v in result.items() if k!='uncovered_intervals'}

def export_maps():
    """Aggregate the same normalized call evidence without an alias Cartesian join."""
    from graph import Graph
    from graph_filter import association_exclusion, REVISION
    import xml.etree.ElementTree as ET
    graph=Graph();snapshot=graph.current();con=db(True)
    binding=load(RUNTIME/'published-catalog-binding.json')
    if binding['snapshot']!=snapshot or binding['catalog_sha256']!=digest(RUNTIME/'catalog.sqlite3'):
        raise RuntimeError('Catalog changed since graph publication; import it before exporting a combined map')
    nodes=graph.query("MATCH (n:Entity {snapshot:$s,kind:'Module'}) RETURN n.key AS key,n.name AS name,n.version AS version",{'s':snapshot})
    attribution=collections.defaultdict(set)
    tracked=load(PROJECT/'tracking/module-coverage.json')['modules']
    prefixes={v:[r['path'].removesuffix('/src') for r in tracked if r.get('source_counts',{}).get(v,0)] for v in 'ABC'}
    for row in con.execute('SELECT version,entry,sources,name FROM symbols'):
        if association_exclusion(row['name']):continue
        for path in json.loads(row['sources']):
            attribution[(row['version'],row['entry'])].add(f'module:{row["version"]}:{module_for(path)}')
            candidates=[prefix for prefix in prefixes[row['version']] if path.startswith(prefix+'/')]
            if candidates:
                attribution[(row['version'],row['entry'])].add(f'module:{row["version"]}:{max(candidates,key=len)}')
    owners=collections.defaultdict(set)
    for row in con.execute('SELECT version,entry,start FROM spans'):
        if (row['version'],row['entry']) in attribution:
            owners[(row['version'],row['start'])].add(row['entry'])
    counter=collections.Counter();seen=set()
    for row in con.execute("SELECT version,entry,site,target FROM refs WHERE kind='CALLS'"):
        v=row['version'];dest=attribution.get((v,row['target']),())
        if not dest:continue
        for caller in owners.get((v,row['entry']),()):
            ref=(v,caller,row['site'],row['target'])
            if ref in seen:continue
            seen.add(ref)
            for a in attribution[(v,caller)]:
                for b in dest:
                    if a!=b:counter[(a,b)]+=1
    con.close()
    edges=[{'source':a,'target':b,'source_attributed_unique_calls':n} for (a,b),n in sorted(counter.items())]
    doc={'snapshot':snapshot,'modules':nodes,'dependencies':edges,'semantic_complete':False,
         'filter_policy':REVISION,
         'projection':'Normalized catalog used by this published graph; source-attribution many-to-many preserved',
         'warning':'Source attribution is not exclusive business ownership; no automatic split acceptance'}
    atomic(RUNTIME/'module-map.json',doc)
    ns='http://graphml.graphdrawing.org/xmlns';root=ET.Element('graphml',xmlns=ns)
    ET.SubElement(root,'key',id='name',**{'for':'node','attr.name':'name','attr.type':'string'})
    ET.SubElement(root,'key',id='calls',**{'for':'edge','attr.name':'calls','attr.type':'int'})
    output=ET.SubElement(root,'graph',id=snapshot,edgedefault='directed')
    for n in nodes:
        node=ET.SubElement(output,'node',id=n['key']);ET.SubElement(node,'data',key='name').text=n['name']
    for i,e in enumerate(edges):
        edge=ET.SubElement(output,'edge',id=str(i),source=e['source'],target=e['target'])
        ET.SubElement(edge,'data',key='calls').text=str(e['source_attributed_unique_calls'])
    ET.ElementTree(root).write(RUNTIME/'module-map.graphml',encoding='utf-8',xml_declaration=True)
    return {'snapshot':snapshot,'modules':len(nodes),'dependencies':len(edges),'json':str(RUNTIME/'module-map.json'),'graphml':str(RUNTIME/'module-map.graphml')}
