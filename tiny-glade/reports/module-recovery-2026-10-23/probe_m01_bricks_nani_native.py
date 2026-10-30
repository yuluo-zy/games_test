"""执行原两种砖实例完整数值指令与原半精度助手，不复写原算法。"""
from pathlib import Path
import hashlib,json,math,random,struct,subprocess,tempfile
ROOT=Path(__file__).resolve().parents[2]
ORIGINAL=ROOT.parent/'B版本内容/tiny-glade.rne'
SOURCE=Path(__file__).with_name('probe_m01_bricks_nani_x86.c')
FIXTURE=ROOT/'tests/fixtures/native/m01-bricks-nani.json'
REPORT=Path(__file__).with_name('m01-bricks-nani-native.json')
def sha(data):return hashlib.sha256(data).hexdigest()
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def main():
    original=ORIGINAL.read_bytes();assert sha(original)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
    rng=random.Random(0x2076230);cases=[]
    special=[0,0x80000000,bits(1),bits(-2),bits(65504),bits(65520),bits(2**-24),bits(2**-25),0x7f800000,0xff800000,0x7fc12345,0x7f812345,bits(1.00048828125),bits(1.00146484375)]
    for index in range(256):
        angle=(index%17-8)*0.125
        if index%4==0:
            rotation=[0.,0.,0.,math.cos(angle)];rotation[index%3]=math.sin(angle)
        else:
            rotation=[rng.uniform(-1,1) for _ in range(4)];length=math.sqrt(sum(v*v for v in rotation));rotation=[v/length for v in rotation]
        scale=[[-2.,0.,-0.,1.25,8.,0.01][(index+n*2)%6] for n in range(3)]
        transform=[bits(v) for v in rotation+[rng.uniform(-20,20) for _ in range(3)]+scale]+[0x12345678,0xabcdef01]
        brick=[rng.getrandbits(32),rng.getrandbits(32),special[index%14],special[(index+3)%14],rng.getrandbits(32),rng.getrandbits(32)]
        brick[5]=(brick[5]&0xffffff00)|[0,1,2,3,127,128,255][index%7]
        deformation=[special[(index+n)%14] for n in range(6)]+[rng.getrandbits(32) for _ in range(9)]
        for kind in [0,1]:cases.append(dict(id=len(cases),kind=kind,transform=transform,brick=brick,deformation=deformation))
    raw=b''.join(struct.pack('<35I',c['id'],c['kind'],*c['transform'],*c['brick'],*c['deformation']) for c in cases)
    with tempfile.TemporaryDirectory(prefix='m01-bricks-') as d:
        p=Path(d);exe=p/'probe';inputs=p/'input';outputs=p/'output';inputs.write_bytes(raw)
        subprocess.run(['clang','-arch','x86_64','-O2',str(SOURCE),'-o',str(exe)],check=True)
        run=subprocess.run([str(exe),str(ORIGINAL),str(inputs),str(outputs)],capture_output=True,text=True,check=True)
        output=outputs.read_bytes();assert len(output)==len(cases)*124
        for case,row in zip(cases,struct.iter_unpack('<31I',output)):
            assert case['id']==row[0] and case['kind']==row[1]
            case['expected_words']=list(row[2:18] if case['kind']==0 else row[2:30]);case['expected_sort']=row[30]
        FIXTURE.write_text(json.dumps(dict(original_sha256=sha(original),cases=cases),separators=(',',':'))+'\n')
        REPORT.write_text(json.dumps(dict(original_sha256=sha(original),source_sha256=sha(SOURCE.read_bytes()),executable_sha256=sha(exe.read_bytes()),input_sha256=sha(raw),output_sha256=sha(output),fixture_sha256=sha(FIXTURE.read_bytes()),cases=len(cases),stderr=run.stderr,native_blocks=['1420767df..142076980','14207756e..142077820'],scope='Complete original per-row numeric blocks and original half helpers; not whole native Bevy queries or GPU execution.'),indent=2)+'\n')
        print(run.stderr.strip())
if __name__=='__main__':main()
