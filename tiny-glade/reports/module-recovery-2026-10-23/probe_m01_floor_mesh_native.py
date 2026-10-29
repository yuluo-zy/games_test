"""原World完整两种floor几何分支，截在Parry构造前；外部allocator/CRT逐项记录。"""
from pathlib import Path
import ctypes,hashlib,json,math,random,struct,subprocess,tempfile
ROOT=Path(__file__).resolve().parents[2]
SOURCE=Path(__file__).with_name('probe_m01_floor_mesh_x86.c')
def sha(data):return hashlib.sha256(data).hexdigest()
def bits(value):return struct.unpack('<I',struct.pack('<f',value))[0]
def main():
    original=ROOT.parent/'B版本内容/tiny-glade.rne';assert sha(original.read_bytes())=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
    rng=random.Random(0x8d3014);cases=[]
    for index in range(96):
        angle=(index-48)*0.17;center=[rng.uniform(-20,20),rng.uniform(-20,20)];height=[0.0,-0.0,2.25,-3.5][index%4]
        if index%2:
            shape=[math.cos(angle),math.sin(angle),*center,rng.uniform(0.1,10),rng.uniform(0.1,10)]
            if index%11==0:shape[4]*=-1;shape[5]*=-1
            kind=1
        else:shape=[*center,rng.uniform(0.05,2.5),angle,0.0,0.0];kind=0
        cases.append(dict(id=index,kind=kind,input=[bits(v) for v in shape+[height]]))
    raw=b''.join(struct.pack('<9I',case['id'],case['kind'],*case['input']) for case in cases)
    with tempfile.TemporaryDirectory(prefix='m01-floor-') as directory:
        p=Path(directory);exe=p/'probe';inputs=p/'input';outputs=p/'output';inputs.write_bytes(raw)
        subprocess.run(['clang','-arch','x86_64','-O2',str(SOURCE),'-o',str(exe)],check=True)
        trig_x86=p/'trig-x86';trig_arm=p/'trig-arm';trig_replay=p/'trig-replay'
        run=subprocess.run([str(exe),str(original),str(inputs),str(outputs),'-',str(trig_x86)],capture_output=True,text=True)
        if run.returncode:raise RuntimeError(f'probe exit{run.returncode}: {run.stderr}')
        x86_output=outputs.read_bytes()
        # 保存首轮完整返回数组；后续ARM external重放不能覆盖它或只留下通过版。
        original_cases=[dict(case) for case in cases];cursor=0
        for case in original_cases:
            header=struct.unpack_from('<5I',x86_output,cursor);cursor+=20;assert header[0]==case['id'];case['panic']=bool(header[1])
            if not case['panic']:
                for name,count in zip(['indices','vertices','normals'],header[2:]):
                    case[name]=list(struct.unpack_from('<'+'I'*(count*3),x86_output,cursor));cursor+=count*12
        assert cursor==len(x86_output)
        SOURCE.with_name('m01-floor-mesh-x86-external.json').write_text(json.dumps(dict(external_math='x86_64 host libm, not Windows CRT',cases=original_cases),separators=(',',':'))+'\n')
        # Python自身运行ARM64，直接调用该体系结构的真实libm；不是Python双精度math近似。
        library=ctypes.CDLL('/usr/lib/libSystem.B.dylib');functions=[library.cosf,library.sinf]
        for function in functions:function.argtypes=[ctypes.c_float];function.restype=ctypes.c_float
        external_calls=list(struct.iter_unpack('<3I',trig_x86.read_bytes()));target_calls=[]
        for kind,argument,result in external_calls:
            value=struct.unpack('<f',struct.pack('<I',argument))[0]
            target_calls.append((kind,argument,bits(functions[kind](value))))
        trig_arm.write_bytes(b''.join(struct.pack('<3I',*row) for row in target_calls))
        run=subprocess.run([str(exe),str(original),str(inputs),str(outputs),str(trig_arm),str(trig_replay)],capture_output=True,text=True)
        if run.returncode:raise RuntimeError(f'ARM external replay exit{run.returncode}: {run.stderr}')
        assert trig_arm.read_bytes()==trig_replay.read_bytes()
        count_inputs=[0,0x80000000,0x7f800000,0xff800000,0x7fc00001,0xffc01234,0x7f800001,0x3f800000,
            1,0x80000001,0x00800000,0x7f7fffff]+[rng.getrandbits(32) for _ in range(244)]
        count_file=p/'counts-in';count_output=p/'counts-out'
        count_file.write_bytes(b''.join(struct.pack('<9I',i,0,0,0,radius,0,0,0,0) for i,radius in enumerate(count_inputs)))
        count_run=subprocess.run([str(exe),str(original),str(count_file),str(count_output),'-',str(p/'counts-trig'),'counts'],capture_output=True,text=True)
        assert count_run.returncode==0,count_run.stderr
        count_cases=[dict(radius=radius,expected=count) for radius,(index,count) in zip(count_inputs,struct.iter_unpack('<IQ',count_output.read_bytes()))]
        output=outputs.read_bytes();at=0
        for case in cases:
            header=struct.unpack_from('<5I',output,at);at+=20;assert header[0]==case['id'];case['panic']=bool(header[1])
            if not case['panic']:
                for name,count in zip(['indices','vertices','normals'],header[2:]):
                    case[name]=list(struct.unpack_from('<'+'I'*(count*3),output,at));at+=count*12
        assert at==len(output)
        fixture=ROOT/'tests/fixtures/native/m01-floor-mesh.json';fixture.write_text(json.dumps(dict(external_math='ARM64 libm returns replayed at original x86 external CRT call sites',cases=cases,count_cases=count_cases),separators=(',',':'))+'\n')
        SOURCE.with_name('m01-floor-mesh-external-trig.json').write_text(json.dumps(dict(
            calls=external_calls,arm64_calls=target_calls,differences=sum(a!=b for a,b in zip(external_calls,target_calls)),
            x86_geometry_sha256=sha(x86_output),arm_external_geometry_sha256=sha(output)),separators=(',',':'))+'\n')
        SOURCE.with_name('m01-floor-mesh-native.json').write_text(json.dumps(dict(
            original_sha256=sha(original.read_bytes()),source_sha256=sha(SOURCE.read_bytes()),executable_sha256=sha(exe.read_bytes()),
            input_sha256=sha(raw),output_sha256=sha(output),x86_output_sha256=sha(x86_output),fixture_sha256=sha(fixture.read_bytes()),cases=len(cases),stderr=run.stderr,
            count_entry='0x140c97bc0',count_cases=len(count_cases),count_input_sha256=sha(count_file.read_bytes()),count_output_sha256=sha(count_output.read_bytes()),
            blocks=['1408d3014..326a','1408d3585..398c'],
            bridges=['Rust alloc/dealloc/realloc/alloc_zeroed via tracked posix allocation','external CRT ceilf/cosf/sinf/fmodf via host float libm','external memcpy/memmove'],
            scope='Complete original floor geometry/control flow through three output Vecs with explicitly replayed ARM64 external libm results. Original x86 host external outputs also retained separately; excludes original ECS/hashmap and Parry construction. Windows CRT numerical identity is not asserted.'),indent=2)+'\n')
        print(run.stderr.strip())
if __name__=='__main__':main()
