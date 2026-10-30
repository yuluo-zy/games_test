"""Import retained A/C native references with byte checks; no new disassembly."""
from artifacts import RUNTIME,SKELETON,baseline,db,digest,lines,load,pack_module,atomic

def import_associated():
    con=db();counts={}
    for v in 'AC':
        b=baseline(v);pe=pack_module().PE(b['binary']['path'])
        path=SKELETON/f'_dependencies/{v}/native-calls.jsonl.gz'
        metadata=load(path.parent/'native-input.json')
        if metadata['binary']['sha256']!=b['binary']['sha256']:raise ValueError('Reference baseline mismatch')
        before=con.total_changes
        for row in lines(path):
            if 'raw_hex' not in row:
                if row.get('event') in ('range_end','range_start','complete'):continue
                raise ValueError('Unknown native reference record without instruction bytes')
            raw=bytes.fromhex(row['raw_hex']);rva=row['va']-pe.image_base
            if raw!=pe.read_rva(rva,len(raw)):raise ValueError('Retained reference byte mismatch')
            if row['event']=='call':kind='CALLS' if row.get('direct') else 'INDIRECT_UNRESOLVED'
            elif row.get('target_va') is not None:kind='TAIL_CANDIDATE'
            else:continue
            target=row.get('target_va');target=None if target is None else target-pe.image_base
            con.execute('INSERT OR IGNORE INTO refs VALUES(?,?,?,?,?,?)',
                        (v,row['range_va']-pe.image_base,rva,target,kind,str(path)))
        counts[v]=con.total_changes-before;con.commit()
    con.close();atomic(RUNTIME/'associated-import.json',counts)
    return counts
