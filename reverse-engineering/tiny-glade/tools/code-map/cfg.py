"""On-demand candidate basic-block graph; unresolved transfers remain explicit."""
from artifacts import RUNTIME,atomic,db,read_blob

def branch(at,raw,text):
    data=bytes.fromhex(raw);end=at+len(data)
    if data[0] in (0xe8,0xe9) and len(data)==5:
        return ('call' if data[0]==0xe8 else 'jump',end+int.from_bytes(data[1:],'little',signed=True))
    if data[0]==0xeb and len(data)==2:return 'jump',end+int.from_bytes(data[1:],'little',signed=True)
    if (0x70<=data[0]<=0x7f or 0xe0<=data[0]<=0xe3) and len(data)==2:
        return 'conditional',end+int.from_bytes(data[1:],'little',signed=True)
    if len(data)==6 and data[0]==0x0f and 0x80<=data[1]<=0x8f:
        return 'conditional',end+int.from_bytes(data[2:],'little',signed=True)
    if text.startswith(('ret','iret')):return 'return',None
    if text.startswith(('jmp','ljmp')):return 'indirect_unresolved',None
    if text.startswith(('ud2','int3')):return 'trap',None
    return 'next',None

def candidate_cfg(key):
    parts=key.split(':')
    if len(parts)!=3 or parts[0]!='function' or parts[1]!='B':raise ValueError('Expected function:B:RVA')
    entry=int(parts[2],16);con=db(True)
    row=con.execute('''SELECT a.* FROM spans s JOIN assemblies a ON a.version=s.version AND a.start=s.start AND a.end=s.end
        WHERE s.version='B' AND s.entry=? AND s.origin='pdb' ORDER BY a.end-a.start DESC LIMIT 1''',(entry,)).fetchone()
    if not row:raise ValueError('No retained PDB range; do not guess CFG ownership')
    doc=read_blob(row);con.close();instructions=doc['instructions'];index={x[0]:x for x in instructions}
    leaders={entry};transfers={};unknowns=[]
    for at,raw,text in instructions:
        end=at+len(raw)//2;kind,target=branch(at,raw,text);transfers[at]=(kind,target,end)
        if kind in ('jump','conditional'):
            if target in index:leaders.add(target)
            else:unknowns.append({'site':at,'kind':'target_outside_selected_range','target':target})
        if kind in ('jump','conditional','return','indirect_unresolved','trap') and end in index:leaders.add(end)
        if kind=='indirect_unresolved':unknowns.append({'site':at,'kind':kind})
    blocks=[];current=None
    for ins in instructions:
        at,raw,text=ins
        if current is None or at in leaders or at!=current['end']:
            current={'start':at,'end':at,'instruction_count':0};blocks.append(current)
        current['end']=at+len(raw)//2;current['instruction_count']+=1;current['last']=at
    owner={}
    for block in blocks:
        owner[block['start']]=block['start']
    edges=[]
    for block in blocks:
        kind,target,end=transfers[block['last']]
        if kind in ('jump','conditional') and target in owner:edges.append({'source':block['start'],'target':target,'kind':kind})
        if kind in ('next','call','conditional') and end in owner:edges.append({'source':block['start'],'target':end,'kind':'fallthrough'})
    result={'function':key,'blocks':blocks,'edges':edges,'undecoded':doc['undecoded'],'unknowns':unknowns,
            'complete':False,'evidence':row['blob']+'#'+row['member'],
            'scope':'Candidate CFG from decoded PDB span; no jump-table resolution, noreturn inference or EH reachability proof'}
    path=RUNTIME/'cfg'/f'{entry:x}.json';atomic(path,result)
    return {'path':str(path),'blocks':len(blocks),'edges':len(edges),'unresolved':len(unknowns),'complete':False}
