"""原自动杂物碰撞体的全部尺寸/局部中心指令，独立于容器和Rapier分配。"""
from pathlib import Path
import hashlib,json,random,struct,subprocess,tempfile
ROOT=Path(__file__).resolve().parents[2]
ORIGINAL=ROOT.parent/'B版本内容/tiny-glade.rne'
SOURCE=Path(__file__).with_name('probe_m01_auto_clutter_collider_x86.c')
def sha(data):return hashlib.sha256(data).hexdigest()
def bits(value):return struct.unpack('<I',struct.pack('<f',value))[0]
def main():
    original=ORIGINAL.read_bytes();assert sha(original)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
    rng=random.Random(0x8d0b93);cases=[]
    special=[0,0x80000000,bits(1.0),bits(-1.0),1,0x80000001,0x7f7fffff,0x7f800000,0xff800000,0x7fc12345,0x7f812345]
    for index in range(256):
        values=([special[index%11],special[(index+3)%11],special[(index+7)%11],special[(index+9)%11]]
            if index<128 else [bits(rng.uniform(-30,30)) for _ in range(4)])
        cases.append(dict(id=index,bounds=values))
    raw=b''.join(struct.pack('<5I',case['id'],*case['bounds']) for case in cases)
    with tempfile.TemporaryDirectory(prefix='m01-auto-collider-') as directory:
        p=Path(directory);exe=p/'probe';inputs=p/'input';outputs=p/'output';inputs.write_bytes(raw)
        subprocess.run(['clang','-arch','x86_64','-O2',str(SOURCE),'-o',str(exe)],check=True)
        run=subprocess.run([str(exe),str(ORIGINAL),str(inputs),str(outputs)],capture_output=True,text=True,check=True)
        output=outputs.read_bytes();assert len(output)==len(cases)*28
        for case,row in zip(cases,struct.iter_unpack('<7I',output)):
            assert case['id']==row[0];case['expected']=list(row[1:])
        fixture=ROOT/'tests/fixtures/native/m01-auto-clutter-collider.json'
        fixture.write_text(json.dumps(dict(cases=cases),separators=(',',':'))+'\n')
        SOURCE.with_name('m01-auto-clutter-collider-native.json').write_text(json.dumps(dict(
            original_sha256=sha(original),source_sha256=sha(SOURCE.read_bytes()),executable_sha256=sha(exe.read_bytes()),
            input_sha256=sha(raw),output_sha256=sha(output),fixture_sha256=sha(fixture.read_bytes()),cases=len(cases),
            block='1408d0b93..1408d0bfb',stderr=run.stderr,
            scope='Complete original scalar/vector dimensions and local center; excludes ECS/container/shape allocation.'),indent=2)+'\n')
        print(run.stderr.strip())
if __name__=='__main__':main()
