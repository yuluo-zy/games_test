"""Execute original B sheep snapshot projection with resolved ECS storage.

No allocator/query callbacks are replaced. The fake ECS table supplies only
resolved column addresses; native capture/apply perform all field copies,
iteration, shortest-zip termination and change-tick writes themselves.
"""
from pathlib import Path
import hashlib, json, random, struct
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RCX, UC_X86_REG_RDX, UC_X86_REG_RSP

ROOT = next(p for p in Path(__file__).resolve().parents if (p/'ljxsj_92385').is_dir())
BIN = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
EXPECTED = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
DATA = BIN.read_bytes()
assert hashlib.sha256(DATA).hexdigest() == EXPECTED
PE = struct.unpack_from('<I', DATA, 0x3c)[0]
OPT = PE + 24
BASE = struct.unpack_from('<Q', DATA, OPT + 24)[0]
SIZE = struct.unpack_from('<I', DATA, OPT + 56)[0]
SECTIONS = OPT + struct.unpack_from('<H', DATA, PE + 20)[0]
q = lambda x: struct.pack('<Q', x)
u = lambda x: struct.pack('<I', x)
vec = lambda a: struct.pack('<3f', *a)

def machine():
    uc = Uc(UC_ARCH_X86, UC_MODE_64)
    uc.mem_map(BASE, (SIZE + 4095) & ~4095)
    for i in range(struct.unpack_from('<H', DATA, PE + 6)[0]):
        off = SECTIONS + 40*i
        _, rva, n, raw = struct.unpack_from('<IIII', DATA, off+8)
        if n: uc.mem_write(BASE+rva, DATA[raw:raw+n])
    uc.mem_map(0x10000000, 0x100000)
    uc.mem_map(0x20000000, 0x10000)
    uc.mem_map(0x30000000, 0x1000)
    return uc

def call(uc, entry, rcx, rdx):
    sp = 0x2000f000-8
    uc.mem_write(sp, q(0x30000000))
    uc.reg_write(UC_X86_REG_RSP, sp)
    uc.reg_write(UC_X86_REG_RCX, rcx)
    uc.reg_write(UC_X86_REG_RDX, rdx)
    uc.emu_start(entry, 0x30000000, count=100000)

def state_bytes(state):
    tag = state['tag'] | (int(state.get('force', False)) << 8)
    return u(tag)+struct.pack('<2f', state['timeout'], state.get('goal_timeout', 0.0))

def row_bytes(row):
    transform = bytes.fromhex('11'*16)+vec(row['position'])+bytes.fromhex('22'*20)
    sheep = bytes.fromhex('33'*48)+state_bytes(row['state'])+bytes.fromhex('44'*44)
    goal = u(int(row['goal'] is not None))+vec(row['goal'] or [0,0,0])
    return transform, sheep, goal

def capture(rows):
    uc = machine(); iterator=0x10000000; out=0x10001000
    columns=[0x10010000,0x10020000,0x10030000]
    uc.mem_write(iterator+0xb0,q(len(rows)))
    uc.mem_write(iterator+0xc0,b'\1')
    for off,addr in zip([0x50,0x70,0x90],columns): uc.mem_write(iterator+off,q(addr))
    for i,row in enumerate(rows):
        for addr,stride,b in zip(columns,[48,104,16],row_bytes(row)):
            uc.mem_write(addr+i*stride,b)
    result=[]
    for _ in rows:
        call(uc,0x1412f03f0,out,iterator)
        result.append(bytes(uc.mem_read(out,40)))
    return result

def apply(rows,snapshots):
    uc=machine(); query=0x10000000; state=0x10001000; world=0x10002000
    table=0x10003000; ids=0x10004000; cols=0x10005000; mapping=0x10006000
    saved=0x10007000; saved_rows=0x10008000
    arrays=[0x10010000,0x10020000,0x10030000]
    ticks=[0x10040000,0x10050000,0x10060000]
    uc.mem_write(query,q(state)+q(world)+u(0)+u(1234))
    uc.mem_write(state+0xf8,q(ids)+q(1)+b'\1')
    uc.mem_write(state+0x110,q(0)+q(1)+q(2))
    uc.mem_write(world+0x1a8,q(table));uc.mem_write(ids,u(0))
    uc.mem_write(table+0x10,q(len(rows))+q(cols))
    uc.mem_write(table+0x38,q(mapping))
    for i,(addr,tick) in enumerate(zip(arrays,ticks)):
        uc.mem_write(mapping+i*8,q((~i)&((1<<64)-1)))
        uc.mem_write(cols+i*48+0x10,q(addr));uc.mem_write(cols+i*48+0x28,q(tick))
    for i,row in enumerate(rows):
        for addr,stride,b in zip(arrays,[48,104,16],row_bytes(row)):uc.mem_write(addr+i*stride,b)
    uc.mem_write(saved,q(len(snapshots))+q(saved_rows)+q(len(snapshots)))
    if snapshots:uc.mem_write(saved_rows,b''.join(snapshots))
    call(uc,0x1416f24f0,query,saved)
    output=[]
    for i in range(len(rows)):
        output.append([bytes(uc.mem_read(addr+i*stride,stride)).hex() for addr,stride in zip(arrays,[48,104,16])])
    changed=[[struct.unpack('<I',uc.mem_read(t+i*4,4))[0] for i in range(len(rows))] for t in ticks]
    return output,changed

def main():
    rng=random.Random(0x5e33); cases=[]
    for case in range(32):
        rows=[]
        for i in range(case%4+1):
            r=lambda: rng.randint(-100,100)/4
            rows.append({'position':[r(),r(),r()], 'goal':None if (case+i)%3==0 else [r(),r(),r()],
                         'state':{'tag':(case+i)%2,'force':bool(case%2),'timeout':r(),'goal_timeout':r()}})
        snapshots=capture(rows)
        for row,snapshot in zip(rows,snapshots):
            t,s,g=row_bytes(row)
            assert snapshot==g+s[48:60]+t[16:28]
        # The native loop must preserve every unrelated byte and stop at the shorter side.
        for count in [0,len(rows)-1,len(rows),len(rows)+1]:
            sent=(snapshots+[snapshots[-1]])[:count]
            blank=[{'position':[91,92,93],'goal':[81,82,83], 'state':{'tag':0,'timeout':71}} for _ in rows]
            output,ticks=apply(blank,sent)
            for i,(before,actual) in enumerate(zip(blank,output)):
                t,s,g=row_bytes(before)
                if i<len(sent):
                    snap=sent[i]; t=t[:16]+snap[28:40]+t[28:];s=s[:48]+snap[16:28]+s[60:];g=snap[:16]
                assert actual==[t.hex(),s.hex(),g.hex()]
                for tick in ticks:assert tick[i]==(1234 if i<len(sent) else 0)
        cases.append({'rows':rows,'native_snapshot_hex':[s.hex() for s in snapshots]})
    result={'status':'passed','binary_sha256':EXPECTED,'version':'B','cases':cases,
            'capture_entries':sum(len(c['rows']) for c in cases),'apply_cases':len(cases)*4,
            'scope':'Original capture hot dense cursor and complete apply with a resolved synthetic dense table; no allocator or query execution claim.'}
    (Path(__file__).parent/'evidence/native-projection.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k!='cases'}))

if __name__=='__main__':main()
