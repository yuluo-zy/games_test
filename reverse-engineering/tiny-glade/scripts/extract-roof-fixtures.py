"""提取原作 Roof 参数原文/指针，供差分输入覆盖；不冒充历史重放。"""
from pathlib import Path
import json, hashlib, collections

root = Path('D:/game/reverse-engineering/tiny-glade')
target = Path('D:/game/ljxsj_92385/Tiny Glade')
samples, files, tags = [], [], collections.Counter()
for file in sorted((target/'assets/starting-builds').rglob('history.json')):
    raw = file.read_bytes()
    doc = json.loads(raw)
    relative = file.relative_to(target).as_posix()
    files.append(dict(path=relative,sha256=hashlib.sha256(raw).hexdigest(),version=doc['Version']))
    for i, entry in enumerate(doc['History']['edit_chain']):
        for j, edit in enumerate(entry.get('edits',[])):
            if 'Roof' not in edit:
                continue
            for tag, data in edit['Roof'].items():
                tags[tag]+=1
                pointer = f'/History/edit_chain/{i}/edits/{j}/Roof/{tag}'
                if tag=='CreateRoof':
                    samples.append(dict(path=relative,pointer=pointer+'/params',operation=tag,
                                        wall_index=data.get('wall_index'),params=data['params'],complete_parameter_snapshot=True))
                elif tag=='VisualEdit':
                    for prefix in ['prev','new']:
                        # 名称映射仅来自同一 JSON 结构；字节偏移另外由消费者核实。
                        params = {f'{key}_01':data[f'{prefix}_{value}'] for key,value in
                                  [('eave_length','eave'),('height','height'),('profile','profile'),('ridge_length','ridge'),('tip_offset','tip_offset')]
                                  if f'{prefix}_{value}' in data}
                        samples.append(dict(path=relative,pointer=pointer,operation=tag,side=prefix,
                                            wall_index=data.get('wall_index'),params=params,complete_parameter_snapshot=False,
                                            field_source_pointers={k:pointer+f'/{prefix}_{v}' for k,v in
                                              [('eave_length_01','eave'),('height_01','height'),('profile_01','profile'),('ridge_length_01','ridge'),('tip_offset_01','tip_offset')]
                                              if f'{prefix}_{v}' in data}))
report = dict(files=files,roof_operation_counts=dict(tags),samples=samples,
              purpose='原作参数原文与来源供核心算法差分覆盖；没有验证历史消费者或执行重放',
              limitations=['VisualEdit 没有 ridge_dir，不补造默认值','墙形状与编辑时序尚未联接','JSON 名称不能代替原作字段布局验证'])
out = root/'evidence/roof-fixtures.json'
out.write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'files':len(files),'operations':dict(tags),'parameter_snapshots':len(samples)}))
