"""Evidence level: observed JSON structures only. No command/GPU replay or migration."""
from __future__ import annotations
import argparse, collections, hashlib, json, math
from pathlib import Path

DEFAULT_ROOT = Path('D:/game/ljxsj_92385/Tiny Glade')
DEFAULT_OUT = Path(__file__).resolve().parent

def escaped(key): return str(key).replace('~','~0').replace('/','~1')
def typ(v):
    if v is None: return 'null'
    if isinstance(v,bool): return 'boolean'
    if isinstance(v,int): return 'integer'
    if isinstance(v,float): return 'number'
    if isinstance(v,str): return 'string'
    if isinstance(v,list): return 'array'
    return 'object'

class Observer:
    def __init__(self):
        self.nodes={}; self.variants={}; self.pairs={}; self.curves=collections.Counter()
        self.curve_exceptions=[]; self.ids={}
    def walk(self,v,path,source,pointer):
        node=self.nodes.setdefault(path,{'types':collections.Counter(),'object_instances':0,
            'fields':collections.Counter(),'array_lengths':collections.Counter(),'values':set(),
            'number_min':None,'number_max':None,'sources':[]})
        t=typ(v); node['types'][t]+=1
        if len(node['sources'])<2 and {'file':source,'pointer':pointer} not in node['sources']:
            node['sources'].append({'file':source,'pointer':pointer})
        if t in {'integer','number'}:
            if not math.isfinite(v): raise ValueError(f'Nonfinite number {source} {pointer}')
            node['number_min']=v if node['number_min'] is None else min(node['number_min'],v)
            node['number_max']=v if node['number_max'] is None else max(node['number_max'],v)
        if t=='string' and len(node['values'])<=80: node['values'].add(v)
        if t=='array':
            node['array_lengths'][len(v)]+=1
            for i,x in enumerate(v): self.walk(x,path+'/[]',source,pointer+'/'+str(i))
        elif t=='object':
            node['object_instances']+=1
            for key,x in v.items():
                canon='{numeric_key}' if key.lstrip('-').isdigit() else key
                node['fields'][canon]+=1
                self.walk(x,path+'/'+escaped(canon),source,pointer+'/'+escaped(key))
                if key in {'wall_index','wall_id','decorator_id','index','id','roof_id','snapshot_id','chain_index','color_id','tree_id'}:
                    rec=self.ids.setdefault(path+'/'+key,{'count':0,'types':collections.Counter(),'examples':[]})
                    rec['count']+=1; rec['types'][typ(x)]+=1
                    if len(rec['examples'])<2: rec['examples'].append({'file':source,'pointer':pointer+'/'+key,'value':x})
            if len(v)==1:
                key=next(iter(v))
                if key and key[0].isupper():
                    var=self.variants.setdefault(path,{'tags':collections.Counter(),'sources':{}})
                    var['tags'][key]+=1
                    var['sources'].setdefault(key,{'file':source,'pointer':pointer})
            if 'before' in v and 'after' in v:
                record=self.pairs.setdefault(path,{'count':0,'equal_count':0,'before_types':collections.Counter(),
                    'after_types':collections.Counter(),'changed_top_fields':collections.Counter(),'examples':[]})
                b,a=v['before'],v['after']; record['count']+=1; record['equal_count']+=b==a
                record['before_types'][typ(b)]+=1; record['after_types'][typ(a)]+=1
                if isinstance(b,dict) and isinstance(a,dict):
                    changed=[k for k in sorted(set(b)|set(a)) if b.get(k)!=a.get(k)]
                    record['changed_top_fields'].update(changed)
                    added=sorted(set(a)-set(b)); removed=sorted(set(b)-set(a))
                else: changed=[]; added=[]; removed=[]
                if len(record['examples'])<2: record['examples'].append({'file':source,'pointer':pointer,
                    'changed_fields':changed,'added_keys':added,'removed_keys':removed})
            if 'points_u' in v and 'points' in v and isinstance(v['points'],list):
                points,u=v['points'],v['points_u']; self.curves['curve_instances']+=1
                ok=len(points)==len(u); self.curves['matching_point_u_counts']+=ok
                vec=all(isinstance(x,list) and len(x)==3 for x in points); self.curves['points_all_vec3']+=vec
                self.curves['points_all_vec2']+=all(isinstance(x,list) and len(x)==2 for x in points)
                monotone=all(u[i]<=u[i+1] for i in range(len(u)-1)); self.curves['u_nondecreasing']+=monotone
                norm=all(0<=x<=1 for x in u); self.curves['u_in_0_1']+=norm
                if not all([ok,vec,monotone,norm]) and len(self.curve_exceptions)<12:
                    self.curve_exceptions.append({'file':source,'pointer':pointer,'matching_counts':ok,'vec3':vec,'monotone':monotone,'normalized':norm})
    def export(self):
        out={}
        for path,node in sorted(self.nodes.items()):
            obj=node['object_instances']; fields=node['fields']
            out[path]={'observed_types':dict(node['types']),'object_instances':obj,
                'field_occurrences':dict(fields),'present_in_every_observed_object':[k for k,c in fields.items() if c==obj] if obj else [],
                'array_length_counts':dict(sorted(node['array_lengths'].items())),
                'numeric_range':[node['number_min'],node['number_max']] if node['number_min'] is not None else None,
                'observed_string_values':sorted(node['values']) if len(node['values'])<=80 else 'more than 80 distinct values; omitted',
                'sources':node['sources']}
        return out

def investigate(root=DEFAULT_ROOT):
    observer=Observer(); commands=collections.Counter(); groups={}; files=[]; issues=[]
    topkeys=collections.Counter(); versions=collections.Counter(); version_tags={};
    diff_stats={}; command_samples={}; decorator_targets=collections.Counter(); destination_tags=collections.Counter(); destination_examples={}
    def check(condition,source,pointer,label):
        if not condition: issues.append({'file':source,'pointer':pointer,'check':label})
    for p in sorted((root/'assets/starting-builds').rglob('history.json')):
        raw=p.read_bytes(); v=json.loads(raw); source=p.relative_to(root).as_posix()
        topkeys.update(v.keys()); version=v.get('Version'); versions[version]+=1
        h=v['History']; chain=h['edit_chain']; cursor=h['chain_index']
        check(isinstance(cursor,int) and -1<=cursor<len(chain),source,'/History/chain_index','cursor index in observed chain range')
        f={'file':source,'sha256':hashlib.sha256(raw).hexdigest(),'version':version,'keys':list(v),
            'history_keys':list(h),'chain_length':len(chain),'chain_index':cursor,
            'entries_beyond_cursor':len(chain)-cursor-1,'history_start':h.get('history_start'),'snapshots':[]}
        for i,s in enumerate(h.get('snapshots',[])):
            sp=p.parent/'snapshots'/f"{s['snapshot_id']}.snapshot"
            exists=sp.exists(); f['snapshots'].append({**s,'file_exists':exists,'file':sp.relative_to(root).as_posix()})
            check(exists,source,f'/History/snapshots/{i}','snapshot reference file exists')
        files.append(f); vt=version_tags.setdefault(version,collections.Counter())
        for ci,entry in enumerate(chain):
            ep=f'/History/edit_chain/{ci}'
            check(isinstance(entry,dict) and isinstance(entry.get('edits'),list),source,ep,'entry has edits array')
            for ei,edit in enumerate(entry['edits']):
                ptr=ep+f'/edits/{ei}'
                check(isinstance(edit,dict) and len(edit)==1,source,ptr,'edit has one outer tag')
                for tag,payload in edit.items():
                    commands[tag]+=1; vt[tag]+=1
                    observer.walk(payload,'/commands/'+tag,source,ptr+'/'+tag)
                    command_samples.setdefault(tag,{'file':source,'pointer':ptr})
                    if tag in {'WallNewSystem','Wall'}:
                        check(isinstance(payload,list),source,ptr,'wall payload is nested array')
                        gr=groups.setdefault(tag,{'group_length_counts':collections.Counter(),'operations_by_group':{}})
                        gr['group_length_counts'][len(payload)]+=1
                        for gi,group in enumerate(payload):
                            check(isinstance(group,list),source,ptr+f'/{tag}/{gi}','group is array')
                            ops=gr['operations_by_group'].setdefault(gi,collections.Counter())
                            for oi,op in enumerate(group):
                                check(isinstance(op,dict) and len(op)==1,source,ptr+f'/{tag}/{gi}/{oi}','wall operation has one tag')
                                ops.update(op.keys())
            for section in ['decorators','stairs']:
                if section not in entry: continue
                observer.walk(entry[section],'/entry/'+section,source,ep+'/'+section)
                for di,pair in enumerate(entry[section].get('diffs',[])):
                    dp=ep+f'/{section}/diffs/{di}'
                    check(isinstance(pair,list) and len(pair)==2,source,dp,'diff is [target, before_after] pair')
                    target,state=pair
                    target_tag=next(iter(target)) if isinstance(target,dict) else target
                    decorator_targets[str(target_tag)]+=1
                    check(isinstance(state,dict) and 'before' in state and 'after' in state,source,dp+'/1','diff contains before and after')
                    if isinstance(state['before'],dict) and isinstance(state['after'],dict):
                        before,after=state['before'],state['after']; bk,ak=set(before),set(after)
                        stat=diff_stats.setdefault(section,collections.Counter())
                        stat['diff_pairs']+=1; stat['added_id_occurrences']+=len(ak-bk); stat['removed_id_occurrences']+=len(bk-ak)
                        stat['modified_id_occurrences']+=sum(before[k]!=after[k] for k in bk&ak)
                        stat['unchanged_shared_id_occurrences']+=sum(before[k]==after[k] for k in bk&ak)
                        stat['empty_before']+=not bool(before); stat['empty_after']+=not bool(after)
                        for side,mapping in [('before',before),('after',after)]:
                            for id,d in mapping.items():
                                dst=d.get('dst'); dst_tag=next(iter(dst)) if isinstance(dst,dict) else dst
                                destination_tags[side+'/'+str(dst_tag)]+=1
                                destination_examples.setdefault(str(dst_tag),{'file':source,'pointer':dp+'/1/'+side+'/'+escaped(id)+'/dst',
                                    'target':target,'decorator_id_string':id,'value':dst})
        # Metadata is separate: no assumed lifecycle or replay semantics.
        observer.walk({k:x for k,x in v.items() if k!='History'},'/document_metadata',source,'')
        observer.walk({k:x for k,x in h.items() if k!='edit_chain'},'/history_metadata',source,'/History')
    return {'evidence_id':'TG-HISTORY-OBSERVED-20261010','evidence_level':'OBSERVED_STATIC_JSON',
        'root':str(root),'target_executed':False,'history_files':files,'history_count':len(files),
        'top_key_counts':dict(topkeys),'version_counts':dict(versions),
        'version_command_counts':{k:dict(v) for k,v in version_tags.items()},'command_counts':dict(commands),
        'command_sample_locations':command_samples,
        'wall_group_observations':{k:{'group_length_counts':dict(v['group_length_counts']),
             'operations_by_group':{i:dict(c) for i,c in v['operations_by_group'].items()}} for k,v in groups.items()},
        'nested_tag_observations':{k:{'tags':dict(v['tags']),'sources':v['sources']} for k,v in sorted(observer.variants.items())},
        'path_schema':observer.export(),
        'before_after_observations':{k:{a:dict(b) if isinstance(b,collections.Counter) else b for a,b in v.items()} for k,v in observer.pairs.items()},
        'diff_map_changes':{k:dict(v) for k,v in diff_stats.items()},
        'decorator_target_counts':dict(decorator_targets),'decorator_destination_counts':dict(destination_tags),
        'decorator_destination_examples':destination_examples,
        'id_field_observations':{k:{a:dict(b) if isinstance(b,collections.Counter) else b for a,b in v.items()} for k,v in observer.ids.items()},
        'curve_checks':dict(observer.curves),'curve_exceptions':observer.curve_exceptions,'structural_check_issues':issues,
        'limits':['All tags/fields/types are observed, not a closed schema for every game command.',
            'Array ordering is preserved; grouped wall operation phase meaning is not established.',
            'Numeric JSON keys identify map entries; their allocation/lifecycle requires implementation evidence.',
            'Before/after difference is observed; actual forward/backward application and frame timing are unverified.',
            'Version is a serialized document field, not the executable product version.',
            'Snapshot binaries were only checked for referenced file existence; no decoding.']}

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--root',type=Path,default=DEFAULT_ROOT); ap.add_argument('--out',type=Path,default=DEFAULT_OUT)
    args=ap.parse_args(); report=investigate(args.root); args.out.mkdir(parents=True,exist_ok=True)
    (args.out/'observed-history-schema.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
    print(json.dumps({k:report[k] for k in ['history_count','version_counts','command_counts','curve_checks','diff_map_changes','structural_check_issues']},ensure_ascii=False))
if __name__=='__main__': main()
