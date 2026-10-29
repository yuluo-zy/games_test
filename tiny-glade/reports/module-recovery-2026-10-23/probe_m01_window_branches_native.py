"""从原RNE执行窗随机/cloth/XZ投影合同，保留真实输出作逐位fixture。"""
from pathlib import Path
import hashlib,json,random,struct,subprocess,tempfile
ROOT=Path(__file__).resolve().parents[2]
ORIGINAL=ROOT.parent/'B版本内容/tiny-glade.rne'
SOURCE=Path(__file__).with_name('probe_m01_window_branches_x86.c')
FIXTURE=ROOT/'tests/fixtures/native/m01-window-branches.json'
REPORT=Path(__file__).with_name('m01-window-branches-native.json')
def sha(data):return hashlib.sha256(data).hexdigest()
def bits(value):return struct.unpack('<I',struct.pack('<f',value))[0]
def main():
    original=ORIGINAL.read_bytes();assert sha(original)=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
    rng=random.Random(0x7ebfd0);cases=[]
    words=[0,1,2,3,0xffffffff,0x80000000]+[rng.getrandbits(32) for _ in range(122)]
    for index,word in enumerate(words):
        for rank in [1,2,3]:
            direction=[bits(rng.uniform(-20,20)),bits(rng.uniform(-100,100)),bits(rng.uniform(-20,20))]
            cases.append(dict(id=len(cases),word=word,rank=rank,direction=direction))
    raw=b''.join(struct.pack('<6I',c['id'],c['word'],c['rank'],*c['direction']) for c in cases)
    with tempfile.TemporaryDirectory(prefix='m01-window-') as d:
        p=Path(d);exe=p/'probe';inputs=p/'input';outputs=p/'output';inputs.write_bytes(raw)
        subprocess.run(['clang','-arch','x86_64','-O2',str(SOURCE),'-o',str(exe)],check=True)
        run=subprocess.run([str(exe),str(ORIGINAL),str(inputs),str(outputs)],check=True,capture_output=True,text=True)
        output=outputs.read_bytes();assert len(output)==len(cases)*36
        for case,row in zip(cases,struct.iter_unpack('<9I',output)):
            assert case['id']==row[0];case.update(cloth=row[1],first_top2=row[2],rotation=list(row[3:7]),seed=row[7]|row[8]<<32)
        FIXTURE.write_text(json.dumps(dict(original_sha256=sha(original),cases=cases),separators=(',',':'))+'\n')
        # 测试直接include原结果，避免为一个局部oracle扩大业务crate的依赖。
        rust='&[\n'+''.join(f"({c['word']}, {c['rank']}, {c['direction']!r}, {str(bool(c['cloth'])).lower()}, {c['first_top2']}, {c['rotation']!r}, {c['seed']}),\n" for c in cases)+']\n'
        FIXTURE.with_suffix('.rs').write_text(rust)
        REPORT.write_text(json.dumps(dict(original_sha256=sha(original),source_sha256=sha(SOURCE.read_bytes()),input_sha256=sha(raw),output_sha256=sha(output),executable_sha256=sha(exe.read_bytes()),fixture_sha256=sha(FIXTURE.read_bytes()),cases=len(cases),stderr=run.stderr,scope='Original seed and XZ functions plus original cloth predicate/random instruction blocks; output exits instrumented, not whole generator execution.'),indent=2)+'\n')
        print(run.stderr.strip())
if __name__=='__main__':main()
