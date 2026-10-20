from pathlib import Path
import hashlib,json,subprocess

ROOT=Path('D:/game'); OUT=Path(__file__).parent/'zero-extent'
binary=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
expected='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
assert hashlib.sha256(binary.read_bytes()).hexdigest()==expected
objdump='C:/Program Files/LLVM/bin/llvm-objdump.exe'
ranges=[('resize-publisher',0x1404dacb3,0x1404dad54),
        ('zero-extent-gate',0x1404dc850,0x1404dc980),
        ('common-loop-tail',0x1404de84d,0x1404de93e)]
artifacts=[]
for label,start,end in ranges:
    text=subprocess.check_output([objdump,'--disassemble',f'--start-address={start}',f'--stop-address={end}',str(binary)],text=True)
    target=OUT/(label+'.asm');target.write_text(text,encoding='utf-8')
    artifacts.append({'path':target.name,'start':hex(start),'end_exclusive':hex(end),'sha256':hashlib.sha256(target.read_bytes()).hexdigest()})
symbols=json.loads((ROOT/'reverse-engineering/tiny-glade/evidence/render-architecture/public-symbols.json').read_text())
selected=[s for s in symbols if s.get('va') in ['0x140d55980','0x140609210']]
assert len(selected)==2
(OUT/'original-evidence.json').write_text(json.dumps({'original':str(binary),'sha256':expected,'artifacts':artifacts,'symbols':selected,'observed_branch':{'extent_source':'FrameRenderConfig.extent_0 (TypeId a4c02661e67b048b/eae761e6aa2a67ec)','zero_condition':'width == 0 || height == 0','wait':'std::thread::sleep(Duration::new(0, 1000000))','join':'0x1404de84d (common loop tail)','skips':['resolution/resource rebuild','egui begin/end','App::update','MEI frame flush','render','temporal frame increment','FPS limiter']}},indent=2)+'\n')
print(json.dumps({'original_sha256':expected,'ranges':len(artifacts),'symbols':len(selected)}))
