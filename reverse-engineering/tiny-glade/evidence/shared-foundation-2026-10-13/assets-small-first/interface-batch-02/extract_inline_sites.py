"""Extract existing LLVM code-offset annotations without inventing standalone Rust ABIs."""
import hashlib,json,pathlib,re,struct
OUT=pathlib.Path(__file__).resolve().parent;ASSETS=OUT.parent.parent/'assets'
targets={3167:[0x1a669,0x1a65f,0x1a661,0x1a663,0x1a665],1553:[0x7bf7],4589:[0x20bb7],1615:[0x7872]}
report=[]
for mod,inlinees in targets.items():
    p=(OUT/f'module-{mod}.txt') if mod in [1553,1615] else ASSETS/f'module-{mod}.txt'
    raw=p.read_bytes();text=raw.decode('utf-16') if raw[:2] in [b'\xff\xfe',b'\xfe\xff'] else raw.decode('utf8')
    if mod in [1553,1615]:p.write_text(text,encoding='utf8')
    records=[]
    for m in re.finditer(r'^\s*(\d+) \| (S_\w+).*$',text,re.M):
        end=text.find('\n',m.end());records.append((int(m[1]),m[2],m.start(),m.end()))
    byoff={};proc=None
    for i,(off,kind,start,end) in enumerate(records):
        block=text[start:records[i+1][2] if i+1<len(records) else len(text)]
        if kind.startswith(('S_GPROC32','S_LPROC32')):
            addr=re.search(r'addr = (\d+):(\d+), code size = (\d+)',block)
            name=re.search(r'`([^`]+)`',block)
            if addr:assert int(addr[1])==1, 'Only section 1 .text is mapped by this parser'
            proc={'offset':off,'name':name[1] if name else None,'address':hex(0x140001000+int(addr[2])),'size':int(addr[3])} if addr else None
        byoff[off]=(kind,block,proc)
        match=re.search(r'inlinee = 0x([0-9a-f]+)\b',block,re.I)
        inlinee=int(match[1],16)if match else None
        if kind=='S_INLINESITE' and inlinee in inlinees:
            intervals=[];beg=None
            for line in block.splitlines():
                at=re.search(r'code (0x[0-9A-F]+) ',line,re.I)
                endat=re.search(r'code end (0x[0-9A-F]+)',line,re.I)
                if at:
                    nxt=int(at[1],16)
                    if beg is not None and nxt>beg:
                        intervals.append({'offset_start':beg,'offset_end':nxt,
                            'start':hex(int(proc['address'],16)+beg)if proc else None,'end':hex(int(proc['address'],16)+nxt)if proc else None})
                    beg=nxt
                if endat and beg is not None:
                    stop=int(endat[1],16)
                    intervals.append({'offset_start':beg,'offset_end':stop,
                        'start':hex(int(proc['address'],16)+beg) if proc else None,'end':hex(int(proc['address'],16)+stop) if proc else None})
                    beg=None
            report.append({'module':mod,'record_offset':off,'inlinee':hex(inlinee),'parent_procedure':proc,
                'llvm_annotation':block,'ranges':intervals,'standalone_callable_function':False})
(OUT/'inline-sites.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(json.dumps({'sites':len(report),'by_module':{mod:sum(x['module']==mod for x in report) for mod in targets}}))
