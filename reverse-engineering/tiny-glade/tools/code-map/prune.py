"""Remove mechanical graph entities globally and publish a new filtered snapshot."""
from collections import Counter
import hashlib
from pathlib import Path
import time

from artifacts import RUNTIME, atomic, canonical, db, digest, load
from graph import Graph, batches, snapshot_identity
from graph_filter import Projection, REVISION, association_exclusion

def audit_graph_filter(g):
    cursor='';checked=0;remaining=[]
    while True:
        rows=g.query('''MATCH (n:Entity) USING INDEX n:Entity(id)
            WHERE n.id>$cursor AND n.kind IN ['Symbol','InlineInstance']
            RETURN n.id AS id,n.key AS key,n.name AS name ORDER BY n.id LIMIT 3000''',
            {'cursor':cursor},timeout=60)
        if not rows:break
        for row in rows:
            reason=association_exclusion(row['name'])
            if reason:remaining.append({'id':row['id'],'key':row['key'],'reason':reason})
        checked+=len(rows);cursor=rows[-1]['id']
    report={'ok':not remaining,'checked_named_entities':checked,'remaining':remaining,'policy':REVISION}
    atomic(RUNTIME/'graph-filter-audit.json',report)
    if remaining:raise RuntimeError('Mechanical named entities remain in graph; retain unpublished snapshot and audit')
    return report

def prune(apply=False):
    started=time.monotonic();con=db(True);policy=Projection(con);g=Graph()
    policy_sha=digest(Path(__file__).with_name('graph_filter.py'))
    checkpoint=RUNTIME/'graph-prune-checkpoint.json'
    if checkpoint.exists():
        plan=load(checkpoint)
        if plan['policy_sha256']!=policy_sha:
            raise ValueError('Prune policy changed; audit the retained checkpoint before continuing')
    else:
        current=g.current()
        plan={'source':current,'target':snapshot_identity(con),'policy':REVISION,
              'policy_sha256':policy_sha,'catalog_sha256':digest(RUNTIME/'catalog.sqlite3'),
              'snapshots':[r['id'] for r in g.query('MATCH (s:Snapshot) RETURN s.id AS id')],
              'before':g.query('MATCH (n:Entity) RETURN n.snapshot AS snapshot,count(n) AS count ORDER BY snapshot',timeout=60),
              'phase':'planned'}
        if plan['source']==plan['target']:
            return {'ok':True,'unchanged':True,'snapshot':plan['target'],'policy':REVISION}
        if apply:atomic(checkpoint,plan)
    if plan['catalog_sha256']!=digest(RUNTIME/'catalog.sqlite3'):
        raise ValueError('Catalog changed during prune; retain checkpoint and audit')
    summary={'policy':REVISION,'hidden_logical_keys':len(policy.hidden),
             'hidden_by_kind':dict(sorted(Counter(k.split(':')[0] for k in policy.hidden).items())),
             'removed_physical_entries_by_version':dict(sorted(Counter(v for v,_ in policy.removed_entries).items())),
             'retained_mixed_alias_entries':len(policy.retained_mixed_entries),
             'source_snapshot':plan['source'],'target_snapshot':plan['target']}
    atomic(RUNTIME/'graph-prune-plan.json',{**plan,**summary})
    if not apply:return {'ok':True,'dry_run':True,**summary}
    if plan['phase']=='complete':return load(RUNTIME/'graph-prune-result.json')
    # All old snapshots become unavailable before mutation. The old ID is kept
    # only as an incomplete tombstone; kept entities move to a new snapshot ID.
    g.query('MATCH (s:Snapshot) WHERE s.id IN $ids SET s.complete=false,s.filter_policy=$policy',
            {'ids':plan['snapshots'],'policy':REVISION})
    g.query('MERGE (s:Snapshot {id:$target}) SET s.complete=false,s.filter_policy=$policy,s.schema=1',
            {'target':plan['target'],'policy':REVISION})
    deleted=0
    for snapshot in sorted(set(plan['snapshots']+[plan['target']])):
        for batch in batches(iter(sorted(policy.hidden)),1000):
            row=g.query('''UNWIND $keys AS key MATCH (n:Entity {snapshot:$s,key:key})
                WITH collect(n) AS nodes FOREACH (n IN nodes | DETACH DELETE n)
                RETURN size(nodes) AS count''',{'keys':batch,'s':snapshot},timeout=60)
            deleted+=row[0]['count']
        # Remove chunks that could otherwise retain deleted generic names.
        while True:
            count=g.query('''MATCH (n:NameChunk {snapshot:$s}) WITH n LIMIT 1000
                WITH collect(n) AS nodes FOREACH (n IN nodes | DETACH DELETE n)
                RETURN size(nodes) AS count''',{'s':snapshot})[0]['count']
            if not count:break
        g.query('MATCH (p:SearchProjection {id:$s}) DETACH DELETE p',{'s':snapshot})
        print(canonical({'pruned_snapshot':snapshot,'deleted_entities_so_far':deleted}).decode(),flush=True)
    # Move the currently published retained graph without cloning millions of
    # nodes. Every batch is idempotent; readers remain blocked until completion.
    moved=0
    while True:
        count=g.query('''MATCH (n:Entity {snapshot:$old}) WITH n LIMIT 3000
            SET n.snapshot=$new,n.id=$new+':'+n.key RETURN count(n) AS count''',
            {'old':plan['source'],'new':plan['target']},timeout=60)[0]['count']
        if not count:break
        moved+=count
        if moved%60000==0:print(canonical({'moved_entities':moved}).decode(),flush=True)
    # Audit every source snapshot and the new one for any key the shared policy
    # excludes. No API/UI-only hiding: all matching nodes were detached/deleted.
    remaining=0
    for batch in batches(iter(sorted(policy.hidden)),1000):
        remaining+=g.query('''UNWIND $keys AS key MATCH (n:Entity {snapshot:$s,key:key})
            RETURN count(n) AS count''',{'keys':batch,'s':plan['target']})[0]['count']
    if remaining:raise RuntimeError('Excluded graph entities remain; snapshot unpublished')
    audit=audit_graph_filter(g)
    count=g.query('MATCH (n:Entity {snapshot:$s}) RETURN count(n) AS count',{'s':plan['target']})[0]['count']
    g.query('''MATCH (s:Snapshot {id:$new}) SET s.complete=true,s.entities=$count
        MERGE (p:Publication {id:'current'}) SET p.snapshot=$new''',{'new':plan['target'],'count':count})
    g.query('MATCH (s:Snapshot {id:$old}) SET s.replaced_by=$new',{'old':plan['source'],'new':plan['target']})
    atomic(RUNTIME/'published-catalog-binding.json',{'snapshot':plan['target'],'catalog_sha256':plan['catalog_sha256']})
    result={'ok':True,**summary,'deleted_entities_this_run':deleted,'published_entities':count,
            'excluded_entities_remaining':remaining,'seconds':time.monotonic()-started,
            'named_entities_audited':audit['checked_named_entities'],
            'raw_catalog_and_assembly_preserved':True,'semantic_review_complete':False}
    atomic(RUNTIME/'graph-prune-result.json',result)
    plan['phase']='complete';atomic(checkpoint,plan)
    con.close();return result
