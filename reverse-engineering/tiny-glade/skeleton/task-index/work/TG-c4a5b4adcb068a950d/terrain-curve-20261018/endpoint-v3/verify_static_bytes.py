from pathlib import Path
import hashlib,json,re,pefile
root=Path('/Users/yuluo/Desktop/game')
b=Path(__file__).resolve().parent
binary=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
raw=binary.read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True)
paths=[b.parent/'continuation-v2/native-pack/ui-continue-wall.asm.txt',b/'native-pack/curve2-reverse.asm.txt',root/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-772e8f649ffb32fc5a/terrain-curve-20261018/native-pack/draw-wall.asm.txt']
rows=[]
for path in paths:
    count=nbytes=0
    for line in path.read_text().splitlines():
        m=re.match(r'^([0-9a-f]+):\s+((?:[0-9a-f]{2} ?)+)\s*\t',line)
        if not m:continue
        address=int(m[1],16);expected=bytes.fromhex(m[2]);offset=pe.get_offset_from_rva(address-pe.OPTIONAL_HEADER.ImageBase)
        assert raw[offset:offset+len(expected)]==expected,(path,address)
        count+=1;nbytes+=len(expected)
    rows.append({'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'instructions':count,'instruction_bytes':nbytes,'mismatches':0})
out={'target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(),'target_executed':False,'scope':'static original byte identity, not native/Rust behavior execution','assemblies':rows}
(b/'byte-verification.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps(out,indent=2))
