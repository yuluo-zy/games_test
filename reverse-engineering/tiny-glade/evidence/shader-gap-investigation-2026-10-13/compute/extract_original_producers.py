"""从原发行 PE 异常表和原 LLVM PDB 定位 compute 配置生产者；不猜宏 hash。"""
import argparse, hashlib, json, struct, subprocess
from pathlib import Path

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parents[3])
    parser.add_argument('--original', type=Path, default=Path(r'D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne'))
    parser.add_argument('--llvm', type=Path, default=Path(r'C:/Program Files/LLVM/bin/llvm-objdump.exe'))
    args=parser.parse_args()
    root=args.root
    out=Path(__file__).resolve().parent
    original=args.original.read_bytes()
    u16=lambda p: struct.unpack_from('<H',original,p)[0]
    u32=lambda p: struct.unpack_from('<I',original,p)[0]
    pe=u32(0x3c); opt=pe+24
    if original[pe:pe+4]!=b'PE\0\0' or u16(opt)!=0x20b: raise ValueError('not PE32+ original')
    image=struct.unpack_from('<Q',original,opt+24)[0]
    sections=[]
    for i in range(u16(pe+6)):
        p=opt+u16(pe+20)+i*40
        sections.append(dict(name=original[p:p+8].rstrip(b'\0').decode(),rva=u32(p+12),vsize=u32(p+8),raw=u32(p+20),size=u32(p+16)))
    def off(rva):
        s=next(s for s in sections if s['rva']<=rva<s['rva']+max(s['size'],s['vsize']))
        return s['raw']+rva-s['rva']
    erva=u32(opt+112+24); elen=u32(opt+112+28)
    ranges=[dict(zip(('begin','end','unwind'),struct.unpack_from('<III',original,off(erva)+i))) for i in range(0,elen,12)]
    symbols=json.loads((root/'evidence/render-architecture/public-symbols.json').read_text(encoding='utf8'))
    producers=[s for s in symbols if ('AsComputeShaderSource' in s['demangled'] and 'as_compute_pipeline' in s['demangled']) or ('ComputePipelineDesc::with_entry' in s['demangled']) or ('ComputePipelineDesc' in s['demangled'] and 'compiled_file_path' in s['demangled'])]
    rows=[]; (out/'asm').mkdir(exist_ok=True)
    for s in producers:
        rva=int(s['va'],16)-image
        p=next((p for p in ranges if p['begin']==rva),None)
        row=dict(va=s['va'],name=s['demangled'],range=None)
        if p:
            size=p['end']-p['begin']; code=original[off(rva):off(rva)+size]
            row['range']={**p,'size':size,'bytes_sha256':hashlib.sha256(code).hexdigest()}
            run=subprocess.run([str(args.llvm),'--disassemble','--start-address='+s['va'],'--stop-address='+hex(image+p['end']),str(args.original)],capture_output=True,text=True,check=True)
            (out/'asm'/f"{s['va']}.txt").write_text(run.stdout,encoding='utf8')
        rows.append(row)
    result=dict(original_sha256=hashlib.sha256(original).hexdigest(),image_base=hex(image),exception_directory=dict(rva=erva,bytes=elen),functions=rows)
    (out/'producer-bounds.json').write_text(json.dumps(result,indent=2),encoding='utf8')
    (out/'producer-symbols.json').write_text(json.dumps([dict(va=s['va'],name=s['demangled']) for s in producers],indent=2),encoding='utf8')
    print(json.dumps(dict(producers=len(rows),bounded=sum(r['range'] is not None for r in rows),original_sha256=result['original_sha256'])))
if __name__=='__main__': main()
