"""先核查随包源文本及 PDB 嵌入源码，区分取回原文与行为恢复源码。"""
from pathlib import Path
import subprocess,json,re,hashlib
root=Path('D:/game/reverse-engineering/tiny-glade')
target=Path('D:/game/ljxsj_92385/Tiny Glade')
out=root/'evidence/source-discovery';out.mkdir(parents=True,exist_ok=True)
llvm='C:/Program Files/LLVM/bin/llvm-pdbutil.exe'
result=subprocess.run([llvm,'dump','--named-streams','--summary',str(target/'tiny_glade.pdb')],capture_output=True,text=True,check=True)
(out/'pdb-named-streams.txt').write_text(result.stdout,encoding='utf-8')
streams=[]
for match in re.finditer(r'^  (.+)\n    Index: (\d+)\n    Size in bytes: (\d+)',result.stdout,re.M):
    name,index,size=match.group(1),int(match.group(2)),int(match.group(3))
    entry=dict(name=name,index=index,size=size)
    if name.startswith('/src/files/'):
        file=out/f'embedded-{index}-{name.split(chr(92))[-1]}'
        subprocess.run([llvm,'export',f'--stream={index}',f'--out={file}',str(target/'tiny_glade.pdb')],check=True,capture_output=True)
        data=file.read_bytes();entry.update(exported=file.relative_to(root).as_posix(),sha256=hashlib.sha256(data).hexdigest(),has_xml_autovisualizer=b'AutoVisualizer' in data)
    streams.append(entry)
files=sorted(p.relative_to(target).as_posix() for p in target.rglob('*') if p.is_file())
rust_files=[f for f in files if f.lower().endswith('.rs')]
source_archives=[f for f in files if f.lower().endswith(('.zip','.tar','.tar.gz','.tgz','.7z','.rar'))]
source_server=[s for s in streams if any(k in s['name'].lower() for k in ['srcsrv','sourcelink'])]
metadata=subprocess.run([llvm,'dump','--files','--modi=498',str(target/'tiny_glade.pdb')],capture_output=True,text=True,check=True)
(out/'roof-source-metadata.txt').write_text(metadata.stdout,encoding='utf-8')
report=dict(target=str(target),disk_file_count=len(files),rust_source_files=rust_files,source_archives=source_archives,
  named_streams=streams,source_server_streams=source_server,embedded_rust_text_found=any(s['name'].lower().endswith('.rs') for s in streams),
  original_source_text_recovered=False,
  conclusion='当前发行树未见 .rs 或源码归档；PDB 命名源码流只有三个 .natvis，导出为调试可视化 XML。屋顶模块有源路径/校验和元数据，没有据此取得原始 Rust 文本。',
  next_method='依照匹配 PDB 的原函数与源路径组织 Rust 源码，恢复数据结构与完整控制流，机器码差分作为源码正确性校验。',
  limits=['检索范围是给定发行树及全部 PDB named streams，未声称遍历作者机器或私有仓库','PDB metadata 不是原始源内容','生成的 Rust 是行为恢复源码，不能承诺与作者源文本逐字一致'])
(out/'report.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({k:report[k] for k in ['disk_file_count','rust_source_files','source_archives','embedded_rust_text_found','conclusion']},ensure_ascii=False))
