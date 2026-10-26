"""从完整分页原体提取动作源插入合同；不能按汇编相邻常量猜配对。"""
from pathlib import Path
import hashlib,json,re
OUT=Path(__file__).resolve().parent
ROOT=OUT.parents[3]
parts=[]
for name in ['toolbar-steam-page-native.json','toolbar-steam-rest-native.json']:
    result=json.loads((OUT/name).read_text());assert result['ok']
    for step in result['results']:
        assert not step['result'].get('isError',False)
        for block in step['result']['content']:
            if block['type']=='text':
                item=json.loads(block['text']);assert item['totalLines']==8158
                parts.append((item['offset'],item['decompilation']))
text='\n'.join(p[1] for p in sorted(parts)).replace('\\n','\n').replace('\\t','\t')
text=re.sub(r'^\s*\d+\t','',text,flags=re.M)
rows=[]
for match in re.finditer(r'\*\(undefined2 \*\).* = (0x[0-9a-f]+|[0-9]+);',text):
    start=text.rfind('local_118 =',0,match.start())
    statement=text[start:text.find('\n',start)]
    if '0xffffffffffffff00' in statement:key=0
    else:
        parsed=re.search(r'CONCAT71\(local_118\._1_7_,(0x[0-9a-f]+|[0-9]+)\)',statement)
        assert parsed,statement
        key=int(parsed[1],0)
    rows.append((key,int(match[1],0)))
assert len(rows)==31
assert sum(code&1==0 for key,code in rows)==22
assert sum(code&1==1 for key,code in rows)==9
fixture=ROOT/'tiny-glade/tests/fixtures/native/steam-action-origin-map.ron'
fixture.write_text('[\n'+''.join(f'  ({key}, {code}),\n' for key,code in rows)+']\n')
meta={'scope':'原8158行完整分页体的动作源插入静态合同，不是执行原Steam SDK；汇编公共尾部合并导致邻近常量不能机械配对。',
      'binary_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
      'source_results':{n:hashlib.sha256((OUT/n).read_bytes()).hexdigest() for n in ['toolbar-steam-page-native.json','toolbar-steam-rest-native.json']},
      'decompilation_lines':len(text.splitlines()),'rows':rows,'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest()}
(OUT/'steam-action-origin-map.json').write_text(json.dumps(meta,ensure_ascii=False,indent=2)+'\n')
print('mapped',len(rows),'rows to',fixture)
