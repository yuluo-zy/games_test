"""Dedicated Neo4j backend. No SQL fallback; SQLite is only the extraction manifest."""
from __future__ import annotations
import collections
import hashlib
import inspect
import json
import os
from pathlib import Path
import secrets
import subprocess
import time
import xml.etree.ElementTree as ET
from transport import configuration,request

from artifacts import (RUNTIME, WORKSPACE, PROJECT, SCHEMA, atomic, baseline, canonical,
                       coverage, db, digest, load, module_for, read_blob, space)

NEO = RUNTIME / 'neo4j-community-5.26.32'
JAVA = Path('/Users/yuluo/Library/Java/JavaVirtualMachines/corretto-21.0.5/Contents/Home')
ARCHIVE_SHA = 'd279ae6a9b38034280681a558b0cd222dce89f87cc3d9867f1f7bd28ef5ebc30'

def runtime(action):
    env = dict(os.environ, JAVA_HOME=str(JAVA), NEO4J_HOME=str(NEO))
    if action == 'configure':
        if digest(RUNTIME/'downloads/neo4j-community-5.26.32-unix.tar.gz') != ARCHIVE_SHA:
            raise ValueError('Neo4j archive SHA mismatch')
        cfg = RUNTIME/'connection.json'
        if cfg.exists():
            raise RuntimeError('Already configured; credentials are preserved')
        password=secrets.token_urlsafe(32)
        fd=os.open(cfg,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600)
        with os.fdopen(fd,'w') as f:
            json.dump({'url':'http://127.0.0.1:17474','user':'neo4j','password':password},f)
        conf='''server.default_listen_address=127.0.0.1
server.default_advertised_address=127.0.0.1
server.bolt.enabled=true
server.bolt.listen_address=127.0.0.1:17687
server.bolt.advertised_address=127.0.0.1:17687
server.http.enabled=true
server.http.listen_address=127.0.0.1:17474
server.https.enabled=false
dbms.security.auth_enabled=true
server.memory.heap.initial_size=256m
server.memory.heap.max_size=1g
server.memory.pagecache.size=512m
db.transaction.timeout=10s
db.tx_log.rotation.retention_policy=512M size
db.tx_log.rotation.size=64M
'''
        (NEO/'conf/neo4j.conf').write_text(conf)
        p=subprocess.run([str(NEO/'bin/neo4j-admin'),'dbms','set-initial-password',password],env=env,capture_output=True,text=True)
        if p.returncode:raise RuntimeError('Initial password setup failed: '+p.stderr.replace(password,'[redacted]'))
        atomic(RUNTIME/'runtime-lock.json',{'neo4j':'5.26.32','archive_sha256':ARCHIVE_SHA,
                'download':'https://dist.neo4j.org/neo4j-community-5.26.32-unix.tar.gz',
                'java_home':str(JAVA),'java_version':subprocess.check_output([str(JAVA/'bin/java'),'-version'],stderr=subprocess.STDOUT,text=True)})
        return {'configured':True,'url':'http://127.0.0.1:17474','credentials_file':str(cfg)}
    if action not in ('start','stop','status'):raise ValueError('Invalid lifecycle action')
    p=subprocess.run([str(NEO/'bin/neo4j'),action],env=env,capture_output=True,text=True,timeout=60)
    return {'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr}

class Graph:
    def __init__(self):
        self.transport_config=configuration(RUNTIME/'connection.json')
        self.cfg=self.transport_config[0]

    def _execute(self,statements,timeout=10):
        payload=canonical({'statements':statements})
        result=json.loads(request(self.transport_config,payload,timeout+2))
        if result.get('errors'):raise RuntimeError(str(result['errors']))
        return result['results']

    @staticmethod
    def _rows(result):
        return [dict(zip(result['columns'],x['row'])) for x in result['data']]

    def query(self,statement,params=None,timeout=10):
        result=self._execute([{'statement':statement,'parameters':params or {},'resultDataContents':['row']}],timeout)
        return self._rows(result[0])

    def read_snapshot(self,statement,params,timeout=10):
        """Fresh completion check and guarded read in one HTTP transaction.

        This API is used only by fixed read queries. No cached completion flag.
        The read itself cannot produce rows for an unpublished snapshot.
        """
        guard="MATCH (_codeMapSnapshot:Snapshot {id:$s,complete:true}) WITH _codeMapSnapshot "
        statements=[
            {'statement':'MATCH (s:Snapshot {id:$s}) RETURN s.complete AS complete',
             'parameters':{'s':params['s']},'resultDataContents':['row']},
            {'statement':guard+statement,'parameters':params,'resultDataContents':['row']}]
        results=self._execute(statements,timeout)
        status=self._rows(results[0])
        if not status or status[0].get('complete') is not True:
            raise ValueError('Snapshot is absent or not complete; unpublished batches cannot be queried')
        return self._rows(results[1])

    def current(self):
        rows=self.query("MATCH (p:Publication {id:'current'}) RETURN p.snapshot AS snapshot")
        if not rows:raise RuntimeError('No complete snapshot published')
        return rows[0]['snapshot']

    def schema(self):
        for text in [
            'CREATE CONSTRAINT entity_id IF NOT EXISTS FOR (n:Entity) REQUIRE n.id IS UNIQUE',
            'CREATE CONSTRAINT snapshot_id IF NOT EXISTS FOR (n:Snapshot) REQUIRE n.id IS UNIQUE',
            'CREATE CONSTRAINT publication_id IF NOT EXISTS FOR (n:Publication) REQUIRE n.id IS UNIQUE',
            'CREATE INDEX entity_kind IF NOT EXISTS FOR (n:Entity) ON (n.snapshot,n.kind)',
            'CREATE INDEX entity_address IF NOT EXISTS FOR (n:Entity) ON (n.snapshot,n.version,n.rva)',
            'CREATE INDEX entity_key IF NOT EXISTS FOR (n:Entity) ON (n.snapshot,n.key)',
            'CREATE FULLTEXT INDEX entity_names IF NOT EXISTS FOR (n:Entity) ON EACH [n.name]']:
            self.query(text)

def nid(snapshot,logical):return snapshot+':'+logical

def scope_class(name,paths,policy):
    root=name.split('::',1)[0]
    if root in policy['definition_namespaces_always_excluded']:return 'runtime'
    if root in policy['external_package_definition_roots']:return 'dependency_reuse_boundary'
    if any('/generated/' in p or '_generated/' in p or p.startswith('generated/') for p in paths):return 'generated'
    if any(p.startswith('_references/') for p in paths):return 'dependency_reference'
    if paths:return 'project_source_attribution'
    return 'unknown'

def entities(con,snapshot):
    inv=load(RUNTIME/'inventory.json')
    policy=load(PROJECT/'tracking/dependency-exclusions.json')
    for v in 'ABC':
        b=inv['identities'][v]
        yield {'key':f'binary:{v}','kind':'Binary','name':v,'version':v,'sha':b['binary']['sha256'],
               'evidence':str(PROJECT/'tracking/baselines.json')}
    for row in con.execute('SELECT * FROM functions ORDER BY version,rva'):
        v,rva=row['version'],row['rva']
        yield {'key':f'function:{v}:{rva:x}','kind':'Function','version':v,'rva':rva,
               'name':f'{v}:0x{rva:x}','sha':row['sha'],'physical_key':row['sha']+f':{rva:x}'}
    modules=set();sources=set()
    for row in con.execute('SELECT * FROM symbols ORDER BY version,record'):
        v=row['version'];paths=json.loads(row['sources'])
        yield {'key':f'symbol:{v}:{row["record"]}','kind':'Symbol','name':row['name'],
               'version':v,'rva':row['entry'],'evidence':row['evidence'],'status':'observed',
               'ownership':'source_mapped' if paths else 'unknown','source_paths':paths,
               'scope_class':scope_class(row['name'],paths,policy),
               'defining_namespace':row['name'].split('::',1)[0],
               'scope_policy_evidence':str(PROJECT/'tracking/dependency-exclusions.json')}
        for path in paths:
            if (v,path) not in sources:
                sources.add((v,path))
                yield {'key':f'source:{v}:{path}','kind':'SourceFile','name':path,'version':v,'evidence':row['evidence']}
            module=module_for(path)
            if (v,module) not in modules:
                modules.add((v,module))
                yield {'key':f'module:{v}:{module}','kind':'Module','name':module,'version':v,
                       'semantic_status':'unreviewed','evidence':row['evidence']}
    for row in con.execute('SELECT * FROM spans ORDER BY version,entry,start,end,origin'):
        v=row['version'];rva=row['entry']
        key=f'range:{v}:{rva:x}:{row["start"]:x}:{row["end"]:x}:{row["origin"]}'
        yield {'key':key,'kind':'Range','name':key,'version':v,'rva':rva,
               'start':row['start'],'end':row['end'],'origin':row['origin'],
               'evidence':row['evidence'],'status':'candidate' if 'candidate' in row['origin'] else 'observed'}
    for row in con.execute('SELECT * FROM assemblies ORDER BY version,start,end'):
        yield {'key':f'assembly:{row["version"]}:{row["start"]:x}:{row["end"]:x}',
               'kind':'Assembly','name':f'0x{row["start"]:x}..0x{row["end"]:x}',
               'version':row['version'],'start':row['start'],'end':row['end'],
               'evidence':row['blob']+'#'+row['member'],'sha':row['sha'],
               'status':row['status'],'instructions':row['instructions'],'undecoded_bytes':row['gaps']}
    seen=set()
    for row in con.execute('SELECT * FROM refs ORDER BY version,site'):
        v=row['version'];site=row['site'];key=f'callsite:{v}:{site:x}'
        if key in seen:continue
        seen.add(key)
        yield {'key':key,'kind':'Callsite','name':f'{v}:0x{site:x}','version':v,'rva':site,
               'relation_kind':row['kind'],'target_rva':row['target'], 'evidence':row['evidence'],
               'status':'observed' if row['kind']=='CALLS' else 'candidate'}
    for row in con.execute('SELECT payload FROM extra_nodes ORDER BY key'):
        yield json.loads(row['payload'])
    semantic=Path(__file__).with_name('chain.json')
    if semantic.exists():
        for node in load(semantic)['nodes']:yield node

def links(con):
    def link(a,b,kind,evidence,status='observed',**kw):
        return dict(source=a,target=b,kind=kind,evidence=evidence,status=status,**kw)
    for row in con.execute('SELECT * FROM functions'):
        v=row['version'];yield link(f'binary:{v}',f'function:{v}:{row["rva"]:x}','CONTAINS','inventory.json')
    for row in con.execute('SELECT * FROM symbols'):
        v=row['version'];s=f'symbol:{v}:{row["record"]}';f=f'function:{v}:{row["entry"]:x}'
        yield link(s,f,'DENOTES',row['evidence'])
        for path in json.loads(row['sources']):
            yield link(s,f'source:{v}:{path}','SOURCE_ATTRIBUTION',row['evidence'])
            yield link(f'source:{v}:{path}',f'module:{v}:{module_for(path)}','IN_MODULE',row['evidence'])
    available={(x['version'],x['start'],x['end']) for x in con.execute('SELECT version,start,end FROM assemblies')}
    entries={(x['version'],x['rva']) for x in con.execute('SELECT version,rva FROM functions')}
    for row in con.execute('SELECT * FROM spans'):
        v=row['version'];rva=row['entry'];start,end=row['start'],row['end']
        key=f'range:{v}:{rva:x}:{start:x}:{end:x}:{row["origin"]}'
        if (v,rva) in entries:yield link(f'function:{v}:{rva:x}',key,'HAS_RANGE',row['evidence'])
        else:yield link(f'binary:{v}',key,'UNWIND_RANGE',row['evidence'],'candidate')
        if (v,start,end) in available:yield link(key,f'assembly:{v}:{start:x}:{end:x}','HAS_ASSEMBLY',row['evidence'])
    # Only exact known physical entries produce function-level direct calls.
    # A range start is not necessarily its owning function's entry.
    for row in con.execute('SELECT * FROM refs'):
        v=row['version'];c=f'callsite:{v}:{row["site"]:x}'
        owners=con.execute('SELECT DISTINCT entry FROM spans WHERE version=? AND start=? AND end>?',
                           (v,row['entry'],row['site'])).fetchall()
        for owner in owners:
            if (v,owner['entry']) not in entries:continue
            f=f'function:{v}:{owner["entry"]:x}'
            yield link(f,c,'HAS_CALLSITE',row['evidence'])
            if row['target'] is not None and (v,row['target']) in entries:
                target=f'function:{v}:{row["target"]:x}'
                yield link(c,target,row['kind'],row['evidence'],'observed' if row['kind']=='CALLS' else 'candidate')
                if row['kind']=='CALLS':yield link(f,target,'CALLS',row['evidence'],site=row['site'])
    for row in con.execute('SELECT payload FROM extra_links ORDER BY id'):
        yield json.loads(row['payload'])
    semantic=Path(__file__).with_name('chain.json')
    if semantic.exists():
        for edge in load(semantic)['edges']:yield edge

def batches(iterator,size=1500):
    batch=[]
    for row in iterator:
        batch.append(row)
        if len(batch)>=size:yield batch;batch=[]
    if batch:yield batch

def import_graph():
    space()
    started=time.monotonic();con=db(True);g=Graph();g.schema()
    inventory=load(RUNTIME/'inventory.json')
    # Source + completed assembly catalog + semantic evidence define an immutable graph snapshot.
    h=hashlib.sha256(canonical(inventory['inputs']))
    for row in con.execute('SELECT version,start,end,sha FROM assemblies ORDER BY version,start,end'):
        h.update(canonical(list(row)))
    # Bind all normalized graph inputs. Read API or index-only edits do not
    # change data identity; new refs or review evidence necessarily do.
    for table,order in [('functions','version,rva'),('symbols','version,record'),
                        ('spans','version,entry,start,end,origin'),('refs','version,entry,site,kind'),
                        ('extra_nodes','key'),('extra_links','id'),('enrich_inputs','path')]:
        h.update(table.encode())
        for row in con.execute(f'SELECT * FROM {table} ORDER BY {order}'):
            h.update(canonical(list(row)))
    h.update(inspect.getsource(entities).encode());h.update(inspect.getsource(links).encode())
    h.update(inspect.getsource(scope_class).encode())
    h.update(digest(PROJECT/'tracking/dependency-exclusions.json').encode())
    p=Path(__file__).with_name('chain.json')
    if p.exists():h.update(p.read_bytes())
    snapshot=h.hexdigest()[:24]
    exists=g.query('MATCH (s:Snapshot {id:$s}) RETURN s.complete AS complete',{'s':snapshot})
    if exists and exists[0]['complete']:
        atomic(RUNTIME/'published-catalog-binding.json',{'snapshot':snapshot,'catalog_sha256':digest(RUNTIME/'catalog.sqlite3')})
        con.close();return {'snapshot':snapshot,'unchanged':True,'seconds':time.monotonic()-started}
    g.query('MERGE (s:Snapshot {id:$s}) SET s.complete=false,s.schema=$schema',{'s':snapshot,'schema':SCHEMA})
    n=e=0
    for batch in batches(entities(con,snapshot)):
        space()
        for row in batch:
            row['id']=nid(snapshot,row['key']);row['snapshot']=snapshot
        g.query('UNWIND $rows AS row MERGE (n:Entity {id:row.id}) SET n += row',{'rows':batch},timeout=60)
        n+=len(batch)
        if n%15000==0:print(json.dumps({'import_nodes':n}),flush=True)
    for batch in batches(links(con)):
        space()
        rows=[]
        for edge in batch:
            props=dict(edge)
            props['id']=hashlib.sha256(canonical(edge)).hexdigest()
            rows.append({'source':nid(snapshot,edge['source']),'target':nid(snapshot,edge['target']),'props':props})
        result=g.query('''UNWIND $rows AS row MATCH (a:Entity {id:row.source}) MATCH (b:Entity {id:row.target})
               MERGE (a)-[r:LINK {id:row.props.id}]->(b) SET r += row.props RETURN count(r) AS count''',{'rows':rows},timeout=60)
        if result[0]['count'] != len(rows):raise RuntimeError('Dangling graph relationship; snapshot unpublished')
        e+=len(rows)
        if e%30000==0:print(json.dumps({'import_edges_processed':e}),flush=True)
    actual=g.query('MATCH (n:Entity {snapshot:$s}) RETURN count(n) AS count',{'s':snapshot})[0]['count']
    g.query('''MATCH (s:Snapshot {id:$s}) SET s.complete=true,s.entities=$n
       MERGE (p:Publication {id:'current'}) SET p.snapshot=$s''',{'s':snapshot,'n':actual})
    report={'snapshot':snapshot,'nodes':actual,'node_records':n,'edges_processed':e,'seconds':time.monotonic()-started,
            'scope':coverage(),'semantic_review_complete':False}
    atomic(RUNTIME/'import-latest.json',report);con.close()
    atomic(RUNTIME/'published-catalog-binding.json',{'snapshot':snapshot,'catalog_sha256':digest(RUNTIME/'catalog.sqlite3')})
    return report

def bounded(value,minimum,maximum,name):
    if isinstance(value,bool) or not isinstance(value,int) or not minimum<=value<=maximum:
        raise ValueError(f'{name} must be {minimum}..{maximum}')
    return value

def query_tool(tool,args):
    g=Graph();snapshot=args.get('snapshot') or g.current()
    limit=bounded(args.get('limit',200),1,200,'limit')
    offset=bounded(args.get('offset',0),0,1000000,'offset')
    inferred=args.get('key','').split(':')
    version=args.get('version',inferred[1] if len(inferred)>1 and inferred[1] in ('A','B','C') else 'B')
    if version not in ('A','B','C','unbound'):raise ValueError('Unknown binary version')
    if args.get('version') and len(inferred)>1 and inferred[1] in ('A','B','C') and inferred[1]!=version:
        raise ValueError('Key and version disagree')
    params={'s':snapshot,'limit':limit+1,'offset':offset,'v':version}
    # MCP accepts only these canned read queries. Never execute caller Cypher.
    if tool=='search':
        text=str(args.get('text',''))
        if not text:raise ValueError('Nonempty search text required')
        if text.startswith('0x'):
            rva=int(text,16);rva=rva-0x140000000 if rva>=0x140000000 else rva
            params['rva']=rva
            statement='MATCH (n:Entity) WHERE n.snapshot=$s AND n.version=$v AND n.rva=$rva RETURN properties(n) AS node ORDER BY n.id SKIP $offset LIMIT $limit'
        else:
            import re
            words=re.findall(r'[A-Za-z0-9_]+',text)
            if len(text)>512 or not words or len(words)>64:raise ValueError('Search requires 1..64 word tokens, at most 512 characters')
            params['text']=text;params['query']=' AND '.join('*'+word+'*' for word in words)
            ready=g.query('''MATCH (p:SearchProjection {id:$s})
                WHERE p.complete=true AND p.chunk=8000 AND p.overlap=511
                RETURN p.complete AS ready''',{'s':snapshot})
            if ready and ready[0].get('ready') is True:
                from search_index import statement as indexed_search
                statement=indexed_search()
            else:
                statement="CALL db.index.fulltext.queryNodes('entity_names',$query) YIELD node AS n WHERE n.snapshot=$s AND n.version=$v AND n.name CONTAINS $text RETURN properties(n) AS node ORDER BY n.id SKIP $offset LIMIT $limit"
    elif tool in ('neighbors','paths','evidence_pack'):
        depth=bounded(args.get('depth',2),1,6,'depth')
        direction=args.get('direction','out' if tool=='evidence_pack' else 'both')
        if direction not in ('out','in','both'):raise ValueError('direction must be out, in or both')
        params['id']=nid(snapshot,args['key'])
        kinds=args.get('kinds',['CALLS','PRODUCES','CONSUMES','READS','WRITES','REGISTERS','HAS_CONTRACT'])
        params['kinds']=kinds
        if tool=='neighbors':
            relation={'out':'-[r:LINK]->','in':'<-[r:LINK]-','both':'-[r:LINK]-'}[direction]
            statement=f'MATCH (a:Entity {{id:$id}}){relation}(b:Entity) WHERE r.kind IN $kinds RETURN properties(b) AS node,properties(r) AS relation ORDER BY b.id,r.id SKIP $offset LIMIT $limit'
        else:
            # Node-simple paths; depth and execution time remain bounded.
            target=' AND b.key=$target' if tool=='paths' and args.get('target') else ''
            params['target']=args.get('target')
            relation={'out':f'-[:LINK*1..{depth}]->','in':f'<-[:LINK*1..{depth}]-','both':f'-[:LINK*1..{depth}]-'}[direction]
            statement=f'''MATCH p=(a:Entity {{id:$id}}){relation}(b:Entity)
              WHERE all(r IN relationships(p) WHERE r.kind IN $kinds)
              AND all(n IN nodes(p) WHERE single(m IN nodes(p) WHERE m=n)){target}
              RETURN [n IN nodes(p)|properties(n)] AS nodes,[r IN relationships(p)|properties(r)] AS relations
              ORDER BY length(p),[n IN nodes(p)|n.key],[r IN relationships(p)|r.id]
              SKIP $offset LIMIT $limit'''
    elif tool=='state_users':
        params['key']=args['key']
        statement="MATCH (n:Entity {snapshot:$s,key:$key})-[r:LINK]-(b:Entity) WHERE r.kind IN ['READS','WRITES','PRODUCES','CONSUMES','RELEASES'] RETURN properties(b) AS node,properties(r) AS relation SKIP $offset LIMIT $limit"
    elif tool in ('module_map','unresolved'):
        if tool=='module_map':
            statement="MATCH (n:Entity {snapshot:$s,kind:'Module',version:$v}) RETURN properties(n) AS node SKIP $offset LIMIT $limit"
        else:
            statement="MATCH (n:Entity {snapshot:$s,version:$v}) WHERE n.relation_kind='INDIRECT_UNRESOLVED' OR n.semantic_status='unreviewed' OR n.ownership='unknown' RETURN properties(n) AS node SKIP $offset LIMIT $limit"
    elif tool=='coverage':
        g.read_snapshot('RETURN 1 AS valid',params)
        return {'snapshot':snapshot,**coverage(),'truncated':False,'evidence':['inventory.json','extraction-latest.json']}
    elif tool=='assembly':
        g.read_snapshot('RETURN 1 AS valid',params)
        con=db(True);key=args['key'].split(':')
        if len(key)!=3 or key[0]!='function':raise ValueError('Expected function:VERSION:RVA')
        rows=con.execute('''SELECT DISTINCT a.* FROM spans s JOIN assemblies a ON s.version=a.version AND s.start=a.start AND s.end=a.end
                          WHERE s.version=? AND s.entry=? ORDER BY a.start,a.end''',(key[1],int(key[2],16))).fetchall()
        output=[]
        for row in rows:
            doc=read_blob(row)
            instructions=doc['instructions']
            output.extend(instructions)
        con.close()
        return {'snapshot':snapshot,'key':args['key'],'rows':output[offset:offset+limit],
                'truncated':len(output)>offset+limit,'total':len(output),'next_offset':offset+limit if len(output)>offset+limit else None,
                'evidence':[r['blob']+'#'+r['member'] for r in rows],'unknowns':[] if rows else ['No extracted range']}
    else:raise ValueError('Unknown read tool: '+tool)
    rows=g.read_snapshot(statement,params)
    return {'snapshot':snapshot,'version':params['v'],'rows':rows[:limit],
            'truncated':len(rows)>limit,'next_offset':offset+limit if len(rows)>limit else None,
            'unknowns':['Static evidence does not establish complete runtime dispatch'],
            'evidence':'Each returned node/relation carries its evidence reference'}

def export_map():
    g=Graph();snapshot=g.current()
    nodes=g.query("MATCH (n:Entity {snapshot:$s,kind:'Module'}) RETURN n.key AS key,n.name AS name,n.version AS version,n.semantic_status AS status",{'s':snapshot})
    # Aggregate actual function calls through all PDB source attributions; preserve ambiguity.
    edges=g.query('''MATCH (m:Entity {snapshot:$s,kind:'Module'})<-[:LINK {kind:'IN_MODULE'}]-(src)<-[:LINK {kind:'SOURCE_ATTRIBUTION'}]-(sym)-[:LINK {kind:'DENOTES'}]->(f)
        MATCH (f)-[:LINK {kind:'CALLS'}]->(t)<-[:LINK {kind:'DENOTES'}]-(tsym)-[:LINK {kind:'SOURCE_ATTRIBUTION'}]->(tsrc)-[:LINK {kind:'IN_MODULE'}]->(tm:Entity)
        WHERE tm.snapshot=$s AND m<>tm
        RETURN m.key AS source,tm.key AS target,count(*) AS attributed_call_paths''',{'s':snapshot},timeout=60)
    doc={'snapshot':snapshot,'modules':nodes,'dependencies':edges,'semantic_complete':False,
         'warning':'Source attribution may be many-to-many; path counts are not unique calls'}
    atomic(RUNTIME/'module-map.json',doc)
    ns='http://graphml.graphdrawing.org/xmlns';root=ET.Element('graphml',xmlns=ns)
    graph=ET.SubElement(root,'graph',id=snapshot,edgedefault='directed')
    for n in nodes:ET.SubElement(graph,'node',id=n['key'])
    for i,e in enumerate(edges):ET.SubElement(graph,'edge',id=str(i),source=e['source'],target=e['target'])
    ET.ElementTree(root).write(RUNTIME/'module-map.graphml',encoding='utf-8',xml_declaration=True)
    return {'modules':len(nodes),'dependencies':len(edges),'json':str(RUNTIME/'module-map.json'),'graphml':str(RUNTIME/'module-map.graphml')}
