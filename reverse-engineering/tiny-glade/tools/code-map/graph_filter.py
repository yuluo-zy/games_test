"""Global business-map projection. Raw catalog and byte evidence remain intact."""
import json
import re

REVISION = 'business-functions-2026-10-30-v1'
MECHANICAL_LEAVES = re.compile(
    r'^(?:deserialize|serialize|expecting|visit_[A-Za-z_0-9]+|'
    r'default|clone|clone_from|fmt|eq|partial_cmp|cmp|hash|display|'
    r'apply|get_param|init_state|new_archetype|queue|validate_param|'
    r'register_required_components|clone_behavior|next|from|into)$')
COMMON_OWNER_METHODS = {'new','get','insert','remove'}
UNKNOWN_NAME = re.compile(r'^(?:(?:thunk_)?FUN_|sub_)[0-9a-fA-F]+$')

def normalized_name(name):
    depth=0; text=[]
    for char in name:
        if char=='<':depth+=1
        elif char=='>':depth=max(0,depth-1)
        elif not depth:text.append(char)
    return ''.join(text).strip()

def association_exclusion(name):
    leaf=normalized_name(name).rsplit('::',1)[-1].strip()
    if MECHANICAL_LEAVES.fullmatch(leaf):return 'mechanical_name_no_business_association'
    if leaf in COMMON_OWNER_METHODS:return 'generic_method_requires_owner_evidence'
    return None

class Projection:
    def __init__(self, con):
        bad=set();good=set();self.hidden=set();self.reasons={}
        for row in con.execute('SELECT version,entry,record,name FROM symbols'):
            entry=(row['version'],row['entry']);reason=association_exclusion(row['name'])
            if reason:
                key=f'symbol:{row["version"]}:{row["record"]}'
                self.hidden.add(key);self.reasons[key]=reason;bad.add(entry)
            elif not UNKNOWN_NAME.fullmatch(row['name']):good.add(entry)
        # An address shared through ICF can have a real business alias. Never
        # destroy that body because another alias is Default or a serde helper.
        self.removed_entries=bad-good
        self.retained_mixed_entries=bad&good
        self.hidden.update(f'function:{v}:{at:x}' for v,at in self.removed_entries)
        kept_ranges=set();bad_ranges=set();owners={}
        entries={(r['version'],r['rva']) for r in con.execute('SELECT version,rva FROM functions')}
        for r in con.execute('SELECT * FROM spans'):
            v,entry,start,end=r['version'],r['entry'],r['start'],r['end']
            if (v,entry) in self.removed_entries:
                self.hidden.add(f'range:{v}:{entry:x}:{start:x}:{end:x}:{r["origin"]}')
                bad_ranges.add((v,start,end))
            else:
                kept_ranges.add((v,start,end))
                if (v,entry) in entries:owners.setdefault((v,start),[]).append((entry,end))
        self.hidden.update(f'assembly:{v}:{start:x}:{end:x}' for v,start,end in bad_ranges-kept_ranges)
        for r in con.execute('SELECT version,entry,site,target FROM refs'):
            v,site=r['version'],r['site']
            if ((v,r['target']) in self.removed_entries or
                not any(site<end for _,end in owners.get((v,r['entry']),[]))):
                self.hidden.add(f'callsite:{v}:{site:x}')
        for r in con.execute('SELECT key,payload FROM extra_nodes'):
            node=json.loads(r['payload'])
            if node.get('kind')=='InlineInstance' and association_exclusion(node.get('name','')):
                self.hidden.add(r['key'])

    def allows_node(self,node):
        return node['key'] not in self.hidden

    def allows_edge(self,edge):
        return edge['source'] not in self.hidden and edge['target'] not in self.hidden
