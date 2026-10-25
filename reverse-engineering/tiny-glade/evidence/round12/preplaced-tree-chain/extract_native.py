from pathlib import Path
import json,hashlib,subprocess
import pefile
out=Path(__file__).resolve().parent
exe=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');raw=exe.read_bytes()
pe=pefile.PE(data=raw);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
ranges=[('spawn-preplaced-full',0x140862270,6013),('convert-preplaced-full',0x14226be70,4827),('occupied-circle-full',0x1425defd0,837),('placement-config-full',0x141416520,5827),('insert-starting-full',0x140a844f0,216),('cell-center-full',0x140aa5c70,345),('placement-component-registration',0x140b7e440,0x300),('contents-component-registration',0x141552040,0x300)]
rows=[]
for name,start,size in ranges:
    code=image[start-base:start-base+size]
    (out/(name+'.bin')).write_bytes(code)
    command=['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d','--no-show-raw-insn',f'--start-address={start:#x}',f'--stop-address={start+size:#x}',str(exe)]
    result=subprocess.run(command,capture_output=True,check=True);(out/(name+'.asm')).write_bytes(result.stdout)
    rows.append({'name':name,'va':hex(start),'bytes':size,'sha256':hashlib.sha256(code).hexdigest(),'extent':'full original function' if name not in ('placement-component-registration','contents-component-registration') else 'bounded original registration function excerpt; contains native size write'})
(out/'native-ranges.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'ranges':rows},indent=2)+'\n')
print(json.dumps({'ranges':len(rows)}))
