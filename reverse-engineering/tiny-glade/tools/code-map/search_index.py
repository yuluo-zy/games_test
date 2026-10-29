"""Derived substring indexes; long Rust symbols are indexed without truncation."""
import hashlib
from artifacts import RUNTIME,atomic,canonical,space
from graph import Graph

CHUNK=8000
OVERLAP=511  # Public search strings are at most 512 Unicode characters.

def chunks(text):
    step=CHUNK-OVERLAP
    for index,start in enumerate(range(0,len(text),step)):
        yield index,text[start:start+CHUNK]
        if start+CHUNK>=len(text):break

def build():
    g=Graph();snapshot=g.current();space()
    g.query('CREATE CONSTRAINT name_chunk_id IF NOT EXISTS FOR (n:NameChunk) REQUIRE n.id IS UNIQUE')
    g.query('CREATE CONSTRAINT search_projection_id IF NOT EXISTS FOR (n:SearchProjection) REQUIRE n.id IS UNIQUE')
    g.query('CREATE TEXT INDEX short_symbol_names IF NOT EXISTS FOR (n:SearchShort) ON (n.name)')
    g.query('CREATE TEXT INDEX long_symbol_chunks IF NOT EXISTS FOR (n:NameChunk) ON (n.text)')
    existing=g.query('MATCH (p:SearchProjection {id:$s}) RETURN p.complete AS complete',{'s':snapshot})
    if existing and existing[0]['complete']:return {'snapshot':snapshot,'unchanged':True}
    g.query('MERGE (p:SearchProjection {id:$s}) SET p.complete=false,p.chunk=$chunk,p.overlap=$overlap',
            {'s':snapshot,'chunk':CHUNK,'overlap':OVERLAP})
    cursor='';short=0
    while True:
        space()
        rows=g.query('''MATCH (n:Entity {snapshot:$s}) WHERE n.id>$cursor
            RETURN n.id AS id,size(n.name) AS length ORDER BY n.id LIMIT 4000''',{'s':snapshot,'cursor':cursor})
        if not rows:break
        ids=[row['id'] for row in rows if row['length'] is not None and row['length']<=CHUNK]
        if ids:g.query('UNWIND $ids AS id MATCH (n:Entity {id:id}) SET n:SearchShort',{'ids':ids})
        short+=len(ids);cursor=rows[-1]['id']
    cursor='';long=parts=0
    while True:
        space()
        rows=g.query('''MATCH (n:Entity {snapshot:$s}) WHERE n.id>$cursor AND size(n.name)>$chunk
            RETURN n.id AS id,n.name AS name ORDER BY n.id LIMIT 200''',{'s':snapshot,'cursor':cursor,'chunk':CHUNK})
        if not rows:break
        batch=[]
        for row in rows:
            for index,text in chunks(row['name']):
                key=hashlib.sha256(canonical([row['id'],index,text])).hexdigest()
                batch.append({'id':key,'snapshot':snapshot,'entityId':row['id'],'part':index,'text':text})
            long+=1
        if batch:g.query('UNWIND $rows AS row MERGE (n:NameChunk {id:row.id}) SET n += row',{'rows':batch})
        parts+=len(batch);cursor=rows[-1]['id']
    states=g.query("SHOW INDEXES YIELD name,state WHERE name IN ['short_symbol_names','long_symbol_chunks'] RETURN name,state")
    if len(states)!=2 or any(row['state']!='ONLINE' for row in states):
        raise RuntimeError('Search indexes not online; projection remains unpublished; rerun to resume')
    g.query('MATCH (p:SearchProjection {id:$s}) SET p.complete=true,p.short=$short,p.long=$long,p.parts=$parts',
            {'s':snapshot,'short':short,'long':long,'parts':parts})
    report={'snapshot':snapshot,'short_names':short,'long_names':long,'chunks':parts,'max_search_characters':512,
            'original_names_unchanged':True,'index_states':states}
    atomic(RUNTIME/'optimization-search-index.json',report)
    return report

def statement():
    return '''CALL {
        MATCH (n:Entity:SearchShort)
        USING TEXT INDEX n:SearchShort(name)
        WHERE n.name CONTAINS $text AND n.snapshot=$s AND n.version=$v RETURN n
        UNION
        MATCH (c:NameChunk)
        USING TEXT INDEX c:NameChunk(text)
        WHERE c.text CONTAINS $text AND c.snapshot=$s
        MATCH (n:Entity {id:c.entityId})
        WHERE n.snapshot=$s AND n.version=$v AND n.name CONTAINS $text RETURN n
    } RETURN properties(n) AS node ORDER BY n.id SKIP $offset LIMIT $limit'''

if __name__=='__main__':
    import json
    print(json.dumps(build(),indent=2))
