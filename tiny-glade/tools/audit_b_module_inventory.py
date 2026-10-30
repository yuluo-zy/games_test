"""Read-only B symbol/source triage. Name/body presence is NOT behavioral acceptance."""
from pathlib import Path
import collections, csv, gzip, hashlib, json, re, sys

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'tiny-glade/reports/business/2026-10-23-module-recovery'
INDEX = ROOT / 'reverse-engineering/tiny-glade/skeleton/task-index/functions.jsonl.gz'
NAMES = ['公共交互 / toolbar','地形编辑','路径 / 铺石','花园','水与林地边界','墙体 / 规则建筑','屋顶','楼梯 / 平台 / 扶手','装饰物','颜色 / 自定义','clutter','ivy','树工具','森林','小动物','菜单 / 设置 / 主题 / 引导']

# Exclude these from NAME-ONLY source associations, not from native evidence.
# These method names cannot establish nominal owner or behavior equivalence.
sys.path.insert(0,str(ROOT/'reverse-engineering/tiny-glade/tools/code-map'))
from graph_filter import association_exclusion

def module(p):
    # Exclusive primary ownership; shared resource/render/history helpers remain separate.
    rules = [(16,r'/onboarding/|/toolbar/src/menu/|/tiny-glade/src/gui/|theme_fetcher|/screenshot'),
             (1,r'/toolbar/|/backend/input/|mode_manager|/photomode|/rollercoaster'),
             (9,r'/tiny-glade/src/decorator_plugin'),
             (8,r'stairs|platform|handrail'), (12,r'ivy'),
             (10,r'/color/'),(11,r'clutter'),(15,r'/critters/'),(14,r'/forest/'),
             (13,r'/tree-tool/'),(7,r'/roof/|/roofs/'),(9,r'/decorator/'),
             (5,r'/water/|/glade-border/'),(4,r'/garden[/.]'),(3,r'/paths/|cobblestone'),
             (2,r'/terrain[/.]'),(6,r'/wall/|/walls/|/wall-constructor/|/timberframe/|ui_.*shape|ui_.*freehand')]
    for n,pattern in rules:
        if re.search(pattern,p): return f'M{n:02}'
    return 'SHARED'

def mask(s):
    # Preserve offsets while hiding comments and ordinary/raw string literals.
    pat = r'(?s)/\*.*?\*/|//[^\n]*|r(#+)".*?"\1|"(?:\\.|[^"\\])*"'
    return re.sub(pat,lambda m: ''.join('\n' if c=='\n' else ' ' for c in m[0]),s)

def main():
    candidates=collections.defaultdict(list)
    for p in (ROOT/'tiny-glade/crates').rglob('*.rs'):
        if any(x in p.parts for x in ('tests','examples','benches','target')) or p.stem.endswith('_tests'): continue
        s=p.read_text(encoding='utf-8',errors='replace'); m=mask(s)
        cutoff=m.find('#[cfg(test)]')
        if cutoff>=0: m=m[:cutoff]
        for hit in re.finditer(r'\bfn\s+([A-Za-z_][A-Za-z_0-9]*)\s*(?:<|\()',m):
            start=m.find('{',hit.end()); semi=m.find(';',hit.end())
            if start<0 or (semi>=0 and semi<start): continue
            depth=1; end=start+1
            while end<len(m) and depth:
                depth += (m[end]=='{')-(m[end]=='}'); end+=1
            body=m[start+1:end-1].strip()
            state='body_candidate'
            if re.fullmatch(r'(?:todo!|unimplemented!)\s*\([^)]*\)\s*;?',body): state='explicit_stub'
            elif not body: state='empty_body_review'
            elif re.search(r'\b(todo|unimplemented)!',body): state='partial_stub'
            candidates[hit[1]].append(dict(path=p.relative_to(ROOT).as_posix(),line=s.count('\n',0,hit.start())+1,
                state=state,sha256=hashlib.sha256(s.encode()).hexdigest()))
    groups={}; raw=0; excluded=collections.Counter()
    with gzip.open(INDEX,'rt',encoding='utf-8') as f:
        for l in f:
            r=json.loads(l)
            if r['version']!='B' or not r.get('category'): continue
            raw+=1; name=r['name']; p=r['path']
            if re.search(r'credits|report_bug|third_party_licenses',p): excluded['user_removed']+=1; continue
            if re.search(r'closure\$|closure_env\$',name): excluded['closure_instance']+=1; continue
            # Strip generic arguments with balanced brackets, keeping full owner name.
            depth=0; norm=''
            for c in name:
                if c=='<': depth+=1
                elif c=='>': depth=max(0,depth-1)
                elif depth==0: norm+=c
            key=(p,norm)
            if key not in groups: groups[key]=dict(module=module(p),b_path=p,b_name=norm,b_ids=[],candidates=[])
            groups[key]['b_ids'].append(r['id'])
    rows=[]; excluded_associations=[]
    for (p,n),r in groups.items():
        reason=association_exclusion(n)
        if reason:
            excluded_associations.append({**r,'reason':reason})
            continue
        leaf=n.split('::')[-1].strip(); found=candidates.get(leaf,[])
        # Require same primary module, or original source basename, for name candidates.
        found=[x for x in found if module(x['path'].replace('tiny-glade/','',1))==r['module'] or Path(x['path']).name==Path(p).name]
        r['candidates']=found
        if len(found)==1: r['status']=found[0]['state']
        elif found: r['status']='ambiguous_candidates'
        else: r['status']='unmatched_review'
        rows.append(r)
    rows.sort(key=lambda r:(r['module'],r['b_path'],r['b_name']))
    summary={k:dict(collections.Counter(r['status'] for r in rows if r['module']==k)) for k in [f'M{i:02}' for i in range(1,17)]+['SHARED']}
    payload=dict(method='lexical triage only; body_candidate does not mean implemented/restored; unmatched does not mean missing',
        index_sha256=hashlib.sha256(INDEX.read_bytes()).hexdigest(),raw_b_categorized=raw,excluded=dict(excluded),
        excluded_association_counts=dict(collections.Counter(r['reason'] for r in excluded_associations)),
        excluded_associations=excluded_associations,summary=summary,functions=rows)
    (OUT/'B-MODULE-INVENTORY.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2),encoding='utf-8')
    with (OUT/'B-MODULE-INVENTORY.csv').open('w',encoding='utf-8-sig',newline='') as f:
        w=csv.writer(f,lineterminator='\n');w.writerow(['module','b_path','b_name','status','b_ids','current_candidates'])
        for r in rows:w.writerow([r['module'],r['b_path'],r['b_name'],r['status'],';'.join(r['b_ids']),';'.join(f"{x['path']}:{x['line']} [{x['state']}]" for x in r['candidates'])])
    lines=['# B 函数与当前源码：初筛清单','',
        '本表是可复跑的源码定位初筛，尚不是业务复原率。body_candidate 仅表示唯一同名候选有非空且无显式 TODO 的函数体；没有审计其算法、完整性、生产接线或原版等价。unmatched_review 包含改名、合并、宏和真正缺失，不能归为未实现。',
        '','分母：B 索引 category 非空条目；排除 closure/closure_env、用户删除的三类页面、trait/serde/SystemParam 等机械名称及无法凭同名确定 owner 的 new/get/insert/remove 后，按原路径和去泛型后的完整符号名合并。后两类不生成候选关联，原 B ID 与排除理由保留在 JSON 的 excluded_associations；不删除原函数、汇编或调用证据，也不据此宣称这些方法没有业务。保留其他 impl$ 等不确定符号。M01/M16 按工具栏及菜单主归属分开；跨模块 helper 列 SHARED，不重复分摊。M17 没有独立函数分母。',
        '', '| 模块 | 初筛函数组 | 唯一非占位体候选 | 含 TODO | 纯占位 | 空体待审 | 多候选待审 | 未匹配待审 |', '|---|---:|---:|---:|---:|---:|---:|---:|']
    for k,c in summary.items():
        total=sum(c.values()); label=k+(' '+NAMES[int(k[1:])-1] if k!='SHARED' else ' 共享及未分配')
        vals=[c.get(x,0) for x in ('body_candidate','partial_stub','explicit_stub','empty_body_review','ambiguous_candidates','unmatched_review')]
        lines.append('| '+label+' | '+str(total)+' | '+' | '.join(f'{v} ({v/total:.1%})' if total else '0' for v in vals)+' |')
    lines += ['', '## 明确占位位置（仍需判定是否原业务必要分支）','']
    for r in rows:
        if r['status'] in ('partial_stub','explicit_stub','empty_body_review'):
            x=r['candidates'][0];lines.append(f"- {r['module']} `{r['b_name']}` → `{x['path']}:{x['line']}`：{r['status']}")
    lines += ['', '完整逐项清单见 B-MODULE-INVENTORY.csv / .json；JSON 保存候选源码 SHA256。未运行游戏或测试。']
    (OUT/'B-MODULE-INVENTORY.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
    print(json.dumps(dict(raw=raw,groups=len(rows),excluded=dict(excluded),excluded_associations=len(excluded_associations),summary=summary),ensure_ascii=False,indent=2))

if __name__=='__main__': main()
