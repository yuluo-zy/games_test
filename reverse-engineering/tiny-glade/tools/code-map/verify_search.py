"""Compare the derived search with the retained full-text reference, including long names."""
import hashlib
import re
from artifacts import RUNTIME,atomic,canonical
from graph import Graph
from search_index import CHUNK,statement

def verify():
    g=Graph();snapshot=g.current()
    cases=[('B','RaycastWorld::raycast_w_terrain',0),('B','raycast',0),('B','raycast',30),
           ('A','roof',0),('C','roof',0),('unbound','m01-global-ray',0)]
    rows=g.query('''MATCH (n:Entity {snapshot:$s,version:'B'}) WHERE size(n.name)>$chunk
        RETURN n.name AS name ORDER BY n.id LIMIT 3''',{'s':snapshot,'chunk':CHUNK})
    for row in rows:
        text=row['name'];boundary=min(CHUNK,len(text)-1)
        cases.append(('B',text[boundary-100:boundary+100],0))
    results=[]
    reference="CALL db.index.fulltext.queryNodes('entity_names',$query) YIELD node AS n WHERE n.snapshot=$s AND n.version=$v AND n.name CONTAINS $text RETURN properties(n) AS node ORDER BY n.id SKIP $offset LIMIT $limit"
    for version,text,offset in cases:
        params={'s':snapshot,'v':version,'text':text,'query':' AND '.join('*'+w+'*' for w in re.findall(r'[A-Za-z0-9_]+',text)),
                'offset':offset,'limit':201}
        expected=g.query(reference,params);actual=g.query(statement(),params)
        good=canonical(expected)==canonical(actual)
        results.append({'version':version,'query_sha256':hashlib.sha256(text.encode()).hexdigest(),
                        'characters':len(text),'offset':offset,'rows':len(actual),'equal':good,
                        'answer_sha256':hashlib.sha256(canonical(actual)).hexdigest()})
        if not good:raise AssertionError(results[-1])
    report={'ok':True,'snapshot':snapshot,'cases':results,'covers_long_boundary':len(rows)==3}
    atomic(RUNTIME/'optimization-search-equivalence.json',report)
    return report

if __name__=='__main__':
    import json
    print(json.dumps(verify(),indent=2))
