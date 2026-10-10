"""从明确的验收记录生成里程碑/进度；不按符号数推算整游戏百分比。"""
from pathlib import Path
import json
from datetime import datetime, timezone

root = Path('D:/game/reverse-engineering/tiny-glade')
tracking = root/'tracking'
tracking.mkdir(exist_ok=True)
project_file = tracking/'project.json'
if not project_file.exists():
    rows = [
      ('M00','原作身份与研究输入','complete',[],['RNE/PDB 匹配身份；副本哈希一致；EXE 差异登记'],['evidence/binary/baseline.json','workspace/input/identity.json']),
      ('M01','可构建 Rust 工程与证据追踪','in_progress',['M00'],['Cargo 锁定构建成功；每个已验收算法对应原作地址/报告；追踪工具可复现'],['reconstruction/Cargo.toml']),
      ('M02','已定位 Roof 状态函数','complete',['M00'],['Roof::new/ty/is_gable 的观察输出通过机器码差分；浮点/内存边界登记'],['reconstruction/roof/roof_state.rs','evidence/native/roof-state-verification.json']),
      ('M03','屋顶曲面核心','in_progress',['M00','M01'],['原作实际曲面类型及参数来源确定；曲面/导数/法线 Rust 可运行；各原作分支差分通过'],[]),
      ('M04','脊线与屋顶形状组装','in_progress',['M00','M01'],['脊向/脊长/檐口/屋顶分支输入输出明确；完整几何子链 Rust 与机器码差分'],[]),
      ('M05','铺瓦核心','in_progress',['M00','M01'],['屋顶采样到瓦实例布局/索引/变换的完整原作链恢复；CPU/GPU职责明确；差分通过'],[]),
      ('M06','屋顶编辑与完整场景验收','not_started',['M02','M03','M04','M05'],['历史命令到状态/曲面/脊线/瓦实例贯通；矩形、圆形及原作其他已观察类型编辑与撤销验证'],[]),
      ('M07','世界数据/历史/保存恢复','in_progress',['M00','M01'],['33 历史样本消费者、版本迁移、ID与快照恢复实现；撤销/重做/重放与原作对照；缺失样本显式登记'],['reconstruction/history/findings.md','reconstruction/history/sample-verification.json']),
      ('M08','墙体与砖/支撑/开口','not_started',['M01','M07'],['各实际原作形状与相交关系实现；墙编辑到实例输出差分；建筑场景验收'],[]),
      ('M09','门窗/装饰/附着关系','not_started',['M06','M07','M08'],['放置/尺寸/变形/冲突/宿主变化与恢复实现；原作样本与局部规则验证'],[]),
      ('M10','地形/道路/水/园景','not_started',['M01','M07'],['坐标/笔刷/曲线/插值及跨对象变化传播实现；对应原作场景操作对照'],[]),
      ('M11','输入/拾取/任务调度','not_started',['M06','M08','M09','M10'],['原作时序与脏更新依赖明确；连续拖动、反向、撤销与异步任务一致性通过'],[]),
      ('M12','资源/着色器/渲染','not_started',['M01','M06','M08','M09','M10'],['原作实际 pass、实例、材质与资源格式恢复；固定相机画面及性能对照'],[]),
      ('M13','生态/天气/音频/UI/相机','not_started',['M07','M11','M12'],['观察到的外围系统行为/参数/资源依赖逐项实现并验证'],[]),
      ('M14','完整 Rust 游戏集成验收','not_started',['M06','M07','M08','M09','M10','M11','M12','M13'],['可运行交互游戏；全部确认的原作功能清单关闭；历史、场景、视觉、性能及异常恢复回归通过'],[]),
    ]
    project = {'schema_version':1,'target':'D:/game/ljxsj_92385/Tiny Glade',
      'source_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
      'objective':'完成一个 Rust 项目，完整复现当前原作构建的游戏实现与行为',
      'overall_completion_percent':None,
      'percent_policy':'完整功能枚举与验收分母尚未建立，不从函数数、文件数或里程碑数量推算整游戏百分比',
      'milestones':[dict(id=i,title=t,status=s,depends_on=d,acceptance=a,evidence=e,remaining=[]) for i,t,s,d,a,e in rows],
      'local_algorithms':[
        dict(name='WindowSize::aspect_ratio',status='verified_local',source='reconstruction/math/window_aspect.rs',report='evidence/native/window-aspect-verification.json',scope='仅局部 f32 结果，1280 输入'),
        dict(name='Roof::new/ty/is_gable',status='verified_local',source='reconstruction/roof/roof_state.rs',report='evidence/native/roof-state-verification.json',scope='仅字节写入/布尔返回；256 构造器 + 两函数各1348输入')],
      'system_acceptances':[],
      'next_actions':['M03/M04/M05：原作曲面、脊线与铺瓦候选实现及机器码差分','M06：history 参数到屋顶实例完整链路','M07/M08：数据消费者与墙体几何'],
      'known_gaps':['游戏原始 Rust 源未取得，PDB 部分 void() 为占位','22 个快照引用目标缺失','没有完整游戏交互、场景、画面或性能验收','局部机器码差分不等于完整系统复现']}
    project_file.write_text(json.dumps(project,ensure_ascii=False,indent=2),encoding='utf-8')
project = json.loads(project_file.read_text(encoding='utf-8'))
project['updated_at_utc'] = datetime.now(timezone.utc).isoformat()
by_id = {m['id']:m for m in project['milestones']}
statuses = {'complete':'完成验收','in_progress':'进行中','not_started':'未开始','blocked':'有阻塞'}
for milestone in project['milestones']:
    assert milestone['status'] in statuses
    for dep in milestone['depends_on']:
        assert dep in by_id
        if milestone['status']=='complete':
            assert by_id[dep]['status']=='complete',f"{milestone['id']} has unfinished dependency {dep}"
    for evidence in milestone['evidence']:
        assert (root/evidence).exists(),evidence
    if milestone.get('source_gate',{}).get('status') in ('verified','verified_partial'):
        reports=[json.loads((root/e).read_text(encoding='utf-8')) for e in milestone['evidence'] if e.endswith('verification.json')]
        assert reports and all(r.get('ok') is True for r in reports),milestone['id']
for item in project['local_algorithms']:
    assert (root/item['source']).exists()
    if item['status']=='verified_local':
        report = json.loads((root/item['report']).read_text(encoding='utf-8'))
        assert report['ok'] is True,item['name']
        import hashlib
        for path,digest in report.get('candidate_sources',{}).items():
            assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,f'源码变化，需要重新校验: {path}'
        if report.get('candidate_cargo_lock_sha256'):
            assert hashlib.sha256((root/'reconstruction/Cargo.lock').read_bytes()).hexdigest()==report['candidate_cargo_lock_sha256'],'依赖锁文件变化，需要重新校验'
pipeline=root/'evidence/roof-pipeline-verification.json'
if pipeline.exists():
    import hashlib
    proof=json.loads(pipeline.read_text(encoding='utf-8'))
    for path,digest in proof.get('candidate_sources',{}).items():
        assert hashlib.sha256((root/path).read_bytes()).hexdigest()==digest,f'源码变化，需要重新校验: {path}'
lines = ['# Tiny Glade 原作还原：里程碑与进度','',f"更新：{project['updated_at_utc']}。目标：{project['objective']}。",'',
         '**当前不是完整游戏。整体完成百分比暂不报告：完整原作功能清单及验收分母仍在建立。**', '',
         '进度必须区分：定位原作 → 结构/调用链确认 → Rust 实现 → 局部机器码差分 → 系统场景验收。仅反编译不算还原验收。', '',
         '当前用户指定**源码恢复优先**；场景回放和 GPU 研究后置。源码关口与最终系统验收分列，局部差分通过不替代完整游戏集成。', '',
         '| ID | 里程碑 | 最终状态 | 源码关口 | 前置验收 | 验收条件 | 证据 |','|---|---|---|---|---|---|---|']
for m in project['milestones']:
    evidence = '<br>'.join(f'[{Path(e).name}]({e})' for e in m['evidence']) or '尚无验收证据'
    source=m.get('source_gate',{})
    source_text={'verified':'源码实现及原码校验通过','verified_partial':'部分源码及原码校验通过','in_progress':'恢复中','not_started':'未开始','metadata_only':'已核查元数据'}.get(source.get('status'),'—')
    if source.get('scope'):
        source_text+='：'+source['scope']
    lines.append(f"| {m['id']} | {m['title']} | {statuses[m['status']]} | {source_text} | {', '.join(m['depends_on']) or '—'} | {'；'.join(m['acceptance'])} | {evidence} |")
lines += ['', '## 已验证局部算法', '', '| 算法 | 状态/范围 | 实现 | 原作参照 |', '|---|---|---|---|']
for a in project['local_algorithms']:
    lines.append(f"| {a['name']} | {a['status']}：{a['scope']} | [Rust]({a['source']}) | [差分报告]({a['report']}) |")
lines += ['', '## 尚未关闭的边界','']+[f'- {gap}' for gap in project['known_gaps']]
lines += ['', '## 当前后置工作','']+[f'- {work}' for work in project.get('deferred_work',[])]
lines += ['', '## 下一批可执行任务','']+[f'{i+1}. {action}' for i,action in enumerate(project['next_actions'])]
lines += ['', '工程：[Cargo.toml](reconstruction/Cargo.toml)。完整研究计划：[MASTER_PLAN.md](MASTER_PLAN.md)。源码任务与优先级：[TASK_BACKLOG.md](TASK_BACKLOG.md)。依赖排除与持久记忆：[PROJECT_MEMORY.md](PROJECT_MEMORY.md)。机器可读台账：[project.json](tracking/project.json)。', '',
          '依赖列描述最终验收依赖，源码阶段可以沿已确认的数据接口继续推进。排期以实际源码覆盖和验证吞吐滚动更新，不承诺未经测量的整游戏完工日期。更新台账后执行 `scripts/update-progress.py`，它会核验已完成项的依赖、证据文件与局部差分结果。','']
(root/'PROJECT_PROGRESS.md').write_text('\n'.join(lines),encoding='utf-8')
project_file.write_text(json.dumps(project,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'milestones':len(by_id),'complete':sum(m['status']=='complete' for m in by_id.values()),'local_algorithm_groups':len(project['local_algorithms']),'system_acceptances':len(project['system_acceptances'])}))
