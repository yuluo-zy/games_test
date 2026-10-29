"""Evidence-preserving corrections for two truncated Ghidra dispatch stubs."""
import hashlib
import json
import subprocess
from artifacts import RUNTIME,LLVM,atomic,baseline,canonical,db,pack_module

def dispatch_boundaries():
    b=baseline('B');pe=pack_module().PE(b['binary']['path']);con=db();records=[]
    for at,expected,name in [(0x289b680,'ffe0','_guard_dispatch_icall_nop'),
                              (0x289b6a0,'ff25ca5a8f00','_guard_xfg_dispatch_icall_nop')]:
        if not con.execute("SELECT 1 FROM symbols WHERE version='B' AND entry=? AND name=?",(at,name)).fetchone():
            raise ValueError('Expected dispatch identity absent')
        raw=pe.read_rva(at,len(bytes.fromhex(expected)))
        if raw.hex()!=expected:raise ValueError('Known dispatch bytes changed')
        output=subprocess.check_output([str(LLVM/'bin/llvm-objdump'),'-d',
            '--start-address='+hex(pe.image_base+at),'--stop-address='+hex(pe.image_base+at+len(raw)),b['binary']['path']],text=True)
        if 'jmpq' not in output:raise ValueError('Dispatch transfer not independently decoded')
        path=RUNTIME/f'dispatch-boundary-{at:x}.json'
        doc={'version':'B','binary_sha256':b['binary']['sha256'],'name':name,'entry':at,
             'retained_ghidra_range':[at,at+1],'reviewed_instruction_range':[at,at+len(raw)],
             'bytes':expected,'objdump':output,'runtime_target':'unresolved',
             'scope':'Complete first unconditional indirect-transfer instruction; not a resolved runtime call target'}
        if path.exists():
            if json.loads(path.read_text())!=doc:raise ValueError('Existing boundary evidence conflict')
        else:atomic(path,doc)
        con.execute('INSERT OR IGNORE INTO spans VALUES(?,?,?,?,?,?)',
                    ('B',at,at,at+len(raw),'instruction_boundary_review',str(path)))
        node={'key':f'dispute:B:{at:x}','kind':'RangeDispute','name':name+' truncated Ghidra body',
              'version':'B','status':'confirmed','evidence':str(path),'original_range_preserved':True}
        edge={'source':f'function:B:{at:x}','target':node['key'],'kind':'HAS_RANGE_DISPUTE','status':'confirmed','evidence':str(path)}
        con.execute('INSERT OR IGNORE INTO extra_nodes VALUES(?,?)',(node['key'],canonical(node).decode()))
        con.execute('INSERT OR IGNORE INTO extra_links VALUES(?,?)',(hashlib.sha256(canonical(edge)).hexdigest(),canonical(edge).decode()))
        records.append(doc)
    con.commit();con.close();atomic(RUNTIME/'range-disputes.json',records)
    return {'corrected_dispatch_ranges':2,'old_ranges_preserved':True,'runtime_targets_resolved':False}
