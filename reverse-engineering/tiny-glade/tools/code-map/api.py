"""Public read API: bounded graph lookup plus evidence-aware assembly views."""
import hashlib
from pathlib import Path
from functools import lru_cache
import zipfile
from artifacts import RUNTIME, WORKSPACE, db, digest, load, read_blob
from graph import Graph, bounded, query_tool as graph_query
from validation import evidence_status

def compact_node(node):
    omitted={'id','snapshot','physical_key'}
    if node.get('kind')=='Function':omitted.add('sha')
    return {key:value for key,value in node.items() if key not in omitted}

def compact_relation(row):
    return {key:value for key,value in row.items() if key!='id'}

@lru_cache(maxsize=1)
def identities():
    return {v:{'version':v,'binary_sha256':b['binary']['sha256'],'pdb_sha256':b['pdb']['sha256']}
            for v,b in load(RUNTIME/'inventory.json')['identities'].items()}

def query_tool(tool,args):
    if tool=='assembly':
        graph=Graph();snapshot=args.get('snapshot') or graph.current()
        key=args['key'];limit=bounded(args.get('limit',200),1,200,'limit')
        offset=bounded(args.get('offset',0),0,1000000,'offset')
        # Select only blobs belonging to the requested published graph snapshot.
        selected=graph.read_snapshot('''MATCH (f:Entity {snapshot:$s,key:$key})-[:LINK {kind:'HAS_RANGE'}]->(r)
            -[:LINK {kind:'HAS_ASSEMBLY'}]->(a)
            RETURN DISTINCT a.version AS version,a.start AS start,a.end AS end,a.sha AS sha''',{'s':snapshot,'key':key})
        con=db(True);instructions={};evidence=[];undecoded=[]
        for span in selected:
            row=con.execute('SELECT * FROM assemblies WHERE version=? AND start=? AND end=? AND sha=?',
                            (span['version'],span['start'],span['end'],span['sha'])).fetchone()
            if not row:raise RuntimeError('Snapshot blob not available in local manifest')
            doc=read_blob(row)
            for ins in doc['instructions']:
                if isinstance(ins,dict):
                    at=ins['rva'];at=int(at,0) if isinstance(at,str) else at
                    raw,text=ins['bytes'],ins['assembly']
                else:at,raw,text=ins
                value=instructions.setdefault((at,raw),{'rva':at,'bytes':raw,'assembly':text,'ranges':[]})
                value['ranges'].append([span['start'],span['end']])
            undecoded.extend(doc.get('undecoded',[]));evidence.append(row['blob']+'#'+row['member'])
        con.close();rows=sorted(instructions.values(),key=lambda x:(x['rva'],x['bytes']))
        covered={at+offset for at,raw in instructions for offset in range(len(raw)//2)}
        remaining_gaps=sorted(set(undecoded)-covered)
        result={'snapshot':snapshot,'key':key,'rows':rows[offset:offset+limit],'total':len(rows),
                'truncated':len(rows)>offset+limit,'next_offset':offset+limit if len(rows)>offset+limit else None,
                'undecoded':remaining_gaps,'historical_range_gaps':sorted(set(undecoded)),'evidence':evidence,
                'unknowns':['Range ownership and complete CFG are distinct from byte verification']}
    else:
        result=graph_query(tool,args)
        if tool=='coverage':
            # Catalog extraction can advance while a graph snapshot is unpublished.
            # Keep local stage counts explicitly separate from published graph counts.
            result['local_catalog_coverage']=result.pop('versions')
            result['published_graph_counts']=Graph().query('''MATCH (n:Entity {snapshot:$s})
                WHERE n.kind IN ['Function','Assembly','Module','RangeDispute']
                RETURN n.version AS version,n.kind AS kind,count(n) AS count ORDER BY version,kind''',{'s':result['snapshot']})
        if tool=='evidence_pack':
            roots=Graph().query('MATCH (n:Entity {snapshot:$s,key:$key}) RETURN properties(n) AS node',
                                {'s':result['snapshot'],'key':args['key']})
            if roots:result['rows'].append({'nodes':[roots[0]['node']],'relations':[]})
            paths={}
            for row in result['rows']:
                for entity in row.get('nodes',[])+row.get('relations',[]):
                    evidence=entity.get('evidence')
                    if evidence and evidence.startswith(str(WORKSPACE)):
                        paths[evidence]=entity.get('name','')
            excerpts=[]
            for name,hint in sorted(paths.items())[:20]:
                path=Path(name).resolve()
                if not path.is_relative_to(WORKSPACE) or not path.is_file():continue
                if path.suffix not in ('.md','.rs','.json','.c','.asm'):continue
                if path.stat().st_size>2**20:continue
                text=path.read_text(errors='replace')
                # A bounded excerpt is navigation evidence, never an assertion of full coverage.
                anchor=text.find(hint) if hint else -1
                start=max(0,anchor-200) if anchor>=0 else 0
                excerpt=text[start:start+1600]
                excerpts.append({'path':str(path),'sha256':digest(path),
                                 'start_line':text.count('\n',0,start)+1,'excerpt':excerpt,
                                 'truncated':start>0 or start+1600<len(text)})
            result['file_excerpts']=excerpts
            if roots and roots[0]['node'].get('kind')=='EvidenceFile':
                root=roots[0]['node'];name,member=root['blob'].split('#',1)
                with zipfile.ZipFile(RUNTIME/name) as archive:raw=archive.read(member)
                if hashlib.sha256(raw).hexdigest()!=root['sha']:raise ValueError('Archived ASM integrity failure')
                result['archived_excerpt']={'sha256':root['sha'],'identity_status':root['identity_status'],
                                            'text':raw.decode('utf-8',errors='replace')[:1600],'truncated':len(raw)>1600}
            result['assembly_query']={'tool':'assembly','args':{'key':args['key'],'snapshot':result['snapshot']}}
            nodes={};relations={}
            for path in result['rows']:
                for node in path.get('nodes',[]):nodes[node['key']]=compact_node(node)
                for relation in path.get('relations',[]):
                    relations[relation['id']]=compact_relation(relation)
            result['rows']=list(nodes.values());result['relations']=list(relations.values())
            result['representation']='deduplicated subgraph from bounded path results'
        else:
            for row in result.get('rows',[]):
                if isinstance(row,dict):
                    if 'node' in row:row['node']=compact_node(row['node'])
                    if 'relation' in row:row['relation']=compact_relation(row['relation'])
                    if 'nodes' in row:row['nodes']=[compact_node(n) for n in row['nodes']]
                    if 'relations' in row:row['relations']=[compact_relation(r) for r in row['relations']]
    result['source_binding']=evidence_status()
    version=result.get('version',args.get('key','function:B:0').split(':')[1])
    result['binary_identity']=identities().get(version)
    return result
