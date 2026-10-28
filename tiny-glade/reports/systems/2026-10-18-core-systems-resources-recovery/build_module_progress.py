"""Render an evidence-scoped module progress snapshot; no completion percentages."""
from pathlib import Path
from datetime import datetime, timezone, timedelta
import json
import html

HERE = Path(__file__).resolve().parent
PROJECT = HERE.parent.parent
VIS = Path('/Users/yuluo/.codex/visualizations/2026/10/18/01a14d1d-e82e-7163-9e5f-a522c4e1e94f')
REPORT = 'reports/systems/2026-10-18-core-systems-resources-recovery/'

def item(key, group, title, status, done, remaining, paths, evidence):
    return dict(id=key, group=group, title=title, status=status, done=done,
                remaining=remaining, paths=paths, evidence=evidence)

rows = [
 item('wall','核心编辑','墙体 / 栅栏','partial',
      '新墙 / 栅栏、端点续画 / 反向、false 完成、Pending 历史与首输入录制已整合回归。',
      '复杂吸附 / 合墙仍待恢复；实际宿主工具交互、signifier 与 full draw 原侧动态验证待完成。',
      ['crates/country-core/src/systems/wall','crates/country-core/src/resources/walls'],
      [REPORT+'ordinary-wall-r1-result.json',REPORT+'ordinary-wall-r1.log']),
 item('history','核心编辑','历史 / 撤销 / 重做','partial',
      '录制 / Sync、树 / 地形 / 墙历史已验证；共享 pending 快照 ID、判定方向与真实 pop counter 已修复。',
      '完整真实编辑 → 保存 → 加载 → 回放、其余 EditType 与非空辅助 diff 仍需逐链验收。',
      ['crates/country-core/src/systems/history','crates/country-core/src/resources/history'],
      [REPORT+'core-native-r7-result.json',REPORT+'tree-history-r1-result.json']),
 item('terrain','核心编辑','地形绘制 / 编辑','partial',
      '曲线 / RDP、采样高度、拖动、typed 历史、回放及真实事件消费者已整合并分批验证。',
      '真实宿主工具入口、GPU mapped 创建 / 提交 / 同步，以及渲染和实体出生全链仍有缺口。',
      ['crates/country-core/src/systems/terrain','crates/country-core/src/resources/terrain_editing.rs','crates/systems/terrain'],
      [REPORT+'README.md',REPORT+'core-native-r7-result.json']),
 item('save','核心编辑','保存 / 加载 / 快照','partial',
      '14 域元数据、屋顶 / 装饰分桶、真实 ID 及 Motomoto / Deflate 文件编码已通过库回归。',
      '真实域采集 → 后台落盘 / hash → History，以及 State / Roof 解码和文件加载恢复仍未闭合。',
      ['crates/country-core/src/systems/save','crates/country-core/src/resources/save_file.rs'],
      [REPORT+'README.md',REPORT+'tree-history-r1-result.json']),
 item('trees','世界内容','树工具 / 树历史','partial',
      '墙旁 / 区域隐藏、路径清树、完整 placement、Move → Insert → Delete、六向回放已通过。',
      '预置树转换其余过滤、树弯曲 / 地形更新、上层 prefab 出生及实际场景 GPU 效果待补。',
      ['crates/systems/tree-tool','crates/country-core/src/resources/tree_storage.rs'],
      [REPORT+'tree-history-r1-result.json',REPORT+'tree-history-r1.log']),
 item('critters','世界内容','动物 / 羊','partial',
      '统一真实 ECS 组件；羊运动、快照捕获 / 应用、冻结与实时保存已完成有界验证。',
      '出生、完整物种生命周期、完整查询遍历与 RNG，以及场景行为尚未全部验收。',
      ['crates/country-core/src/resources/critters','crates/systems/critters'],
      [REPORT+'core-native-r7-result.json',REPORT+'README.md']),
 item('clutter','世界内容','植被 / 花园 / 杂物','partial',
      'Bush canonical mapped 9601 行、原位排序 / 归一 hash / dirty、真实类型库消费者与注册已修；8 项专项通过。',
      'populate 内另一 BushGrid 及未使用的下游参数仍待接通；真实 GPU 完成 / 同步、其他杂物分支尚未全审。',
      ['crates/country-core/src/systems/ui_clutter','crates/country-core/src/systems/garden','crates/systems/clutter'],
      [REPORT+'bush-clutter-r1-result.json',REPORT+'PENDING_RESOURCE_PRODUCERS.md']),
 item('building','世界内容','建筑构造 / 屋顶 / 楼梯 / 常春藤','review',
      '保留已有构造 / 屋顶 / 楼梯 / 常春藤；屋顶 7 字段、样式和圆旋转保存格式已修并回归。',
      '屋顶 Deserialize 正在恢复；完整几何 / 构造 / 楼梯 / 常春藤仍待逐链复核，不算整模块完成。',
      ['crates/country-core/src/resources/roofs','crates/country-core/src/resources/stairs','crates/country-core/src/resources/ivy','crates/systems/wall-constructor'],
      [REPORT+'README.md']),
 item('color','交互与反馈','颜色 / 材质命令','partial',
      '完整 u64 WallID、三个 reader 的查找 / 边界顺序已修复；避免每帧清空颜色表；专项 8 项通过。',
      '全部颜色 UI、材质发布与 GPU 上传 / 场景效果尚未整体验收。',
      ['crates/systems/color','crates/country-core/src/resources/color_id.rs'],
      [REPORT+'color-wall-r1-assessment.json',REPORT+'color-wall-r1.log']),
 item('ui','交互与反馈','输入 / 模式 / UI / Arrow','partial',
      '光标位移 / 聚焦、模式调度、Arrow 动画、续画 Sphere 与公开 bulldoze gate 已修。',
      '实际宿主输入 / modifier / 相机正在由对应 owner 接通；其他焦点、Arrow 出生与完整 UI 仍待验收。',
      ['crates/country-core/src/systems/mode_manager','crates/country-core/src/systems/ui','crates/country-core/src/systems/arrow_gizmo_animation.rs'],
      [REPORT+'core-native-r7-result.json',REPORT+'README.md']),
 item('onboarding','交互与反馈','教学 / 提示','partial',
      '三个地形 hint 消费者、实际 hover 标签、profile 与 completion 已接 CPU 配置；33 项专项通过。',
      '完整宿主尚未确认调用该新增配置入口；不能据 CPU 验证称游戏教学已可用。',
      ['crates/systems/onboarding','crates/country-core/src/resources/onboarding.rs'],
      [REPORT+'onboarding-native-r2-result.json',REPORT+'README.md']),
 item('audio','交互与反馈','音频生命周期','partial',
      '初始化保留现有状态；19 个实时 stop 调用传入实际时钟，避免常量零导致无法停止。',
      '全局声源出生 / 停止、所有业务音频分支与原全局调度仍未整体验收。',
      ['crates/country-core/src/systems/audio','crates/country-core/src/resources/audio_resources.rs'],
      [REPORT+'core-native-r7-result.json',REPORT+'README.md']),
 item('collision','资源与运行','碰撞 / 射线 / 空间查询','partial',
      '最近命中、TOI / miss 位模式、地形 sphere 投影、姿态与回调停止规则已按锁定 Rapier 修复。',
      '全局句柄归属、完整装饰物谓词及所有真实场景查询组合尚未验收。',
      ['crates/country-core/src/systems/collision'],
      [REPORT+'core-native-r7-result.json',REPORT+'README.md']),
 item('prefab','资源与运行','预制体 / 资产消费者','partial',
      '完整 48B placement 保留；Prefab Part / audio bounds 增量已接，主链真实资产初始化已通过有界 GPU 验证。',
      '其余 prefab 查询 / 过滤及所有实体出生 / 场景渲染仍待验收；不以资产初始化代表可玩世界。',
      ['crates/country-core/src/systems/prefab.rs','crates/country-core/src/resources/prefab.rs','crates/country-core/src/asset_libraries'],
      [REPORT+'core-native-r7-result.json','reports/foundation/2026-10-18-asset-libraries-rereview/README.md']),
 item('masks','资源与运行','GPU 资源 / Mask / SSBO','partial',
      'Mask / Bush / Dormer / Counter / Lantern / Grounded / Rocky 构造与类型已修；72 producer 主链有界验证通过。',
      'Rocky 全启动 / W4-W5、各 readback 完成 / CPU 接受与所有下游消费者仍需完整场景验收。',
      ['crates/country-core/src/resources/terrain_rock_mask.rs','crates/country-core/src/resources/water_mask.rs','crates/country-core/src/resources/garden_mask.rs'],
      [REPORT+'producers-r1-result.json',REPORT+'producers-r1-assessment.json',REPORT+'PENDING_RESOURCE_PRODUCERS.md']),
 item('host','资源与运行','渲染循环 / 实际宿主','active',
      '主链已验证窗口 / loading / present、真实资产初始化、W3 和多个后处理 GPU pass；编辑图通过。',
      '完整生产世界图 / warmup、场景几何与工具交互 / 保存加载闭环尚未完成，仍非完整可玩验收。',
      ['crates/country-core/src/systems/render_loop','crates/tiny-glade','crates/rhapsody'],
      ['reports/runtime/2026-10-18-main-chain-recovery/RENDERER.md','reports/runtime/2026-10-18-main-chain-recovery/MAIN-GRAPH-CONSUMERS.md',REPORT+'README.md']),
]

data = dict(
    title='Tiny Glade · 模块恢复进度',
    generated_at=datetime.now(timezone(timedelta(hours=8))).isoformat(timespec='seconds'),
    scope='以 country-core/systems 与 resources 为中心，包含已涉及的上层消费者；不代表全仓完整验收。',
    target='B 发行 RNE / 匹配 PDB',
    status_labels={'partial':'局部回归通过','active':'正在恢复','review':'待整模块复核'},
    core_inventory={'systems_rs':len(list((PROJECT/'crates/country-core/src/systems').rglob('*.rs'))),'resources_rs':len(list((PROJECT/'crates/country-core/src/resources').rglob('*.rs'))),'note':'当前文件盘点；数量不是恢复完成率。'},
    latest_core_test={'run':'snapshot-batch-r1','passed':2454,'failed':0,'editing_graph_passed':1,'source_drift_total':1,'core_scoped_drift':1,'post_run_drift':'其他owner后续增量另验','scope':'仅gpu_generate_draw_lists.rs在run中漂移；保存批次无漂移，不接受该GPU新源/完整最新宿主。'},
    rules=['保留当前手工重构，增量补功能和修算法。','局部验证不外推为模块完成。','候选冻结不等于生产整合。','原二进制取证、Rust 回归和真实宿主验收分开。'],
    rows=rows,
    other_scope='camera_rig、backend、steam、debug、weather/time、photo/timelapse、forest/water/toolbar 等其他模块已有实现；本轮尚无完整业务清单及逐链验收结论。',
    project_crates=sorted(p.name for p in (PROJECT/'crates').iterdir() if p.is_dir()),
)
for row in rows:
    for p in row['paths'] + row['evidence']:
        if not (PROJECT/p).exists():
            raise FileNotFoundError(p)
(HERE/'module-progress.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n')

page = '''<!doctype html><html lang="zh-CN"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Tiny Glade · 模块恢复进度</title>
<style>
:root{color-scheme:dark;--bg:#0c1420;--panel:#142130;--line:#29394a;--text:#eaf0f6;--muted:#a8b8c8;--green:#77d9ac;--gold:#f3bf68;--blue:#86bdff;--gray:#b2b7c3}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:15px/1.6 -apple-system,BlinkMacSystemFont,"PingFang SC","Microsoft YaHei",sans-serif}main{max-width:1480px;margin:auto;padding:38px 40px 28px}.eyebrow{color:var(--green);font-size:12px;letter-spacing:2px;font-weight:650}h1{font-size:34px;line-height:1.25;font-weight:650;margin:8px 0 10px;letter-spacing:-.7px}.intro{color:var(--muted);max-width:1030px;margin:0}.top{display:flex;justify-content:space-between;align-items:flex-start;gap:20px}.stamp{text-align:right;color:var(--muted);font-size:12px;padding-top:5px;white-space:nowrap}.stamp strong{font-size:13px;color:var(--text);font-weight:500}.context{display:grid;grid-template-columns:1.2fr 1fr 1fr;gap:14px;margin:25px 0 18px}.context>div{background:#142231;border:1px solid var(--line);border-radius:13px;padding:15px 18px}.micro{font-size:11px;letter-spacing:.8px;color:var(--muted);margin-bottom:3px}.value{font-size:19px;font-weight:600}.value b{color:var(--green);font-size:27px;margin-right:7px}.note{font-size:12px;color:var(--muted);margin-top:3px}.controls{display:flex;justify-content:space-between;align-items:center;gap:12px;flex-wrap:wrap;margin:22px 0 20px}.filters{display:flex;gap:6px;flex-wrap:wrap}button{background:transparent;color:var(--muted);border:1px solid var(--line);border-radius:20px;padding:7px 15px;font:inherit;font-size:12px;cursor:pointer}button:hover,button.active{background:#24415a;color:#fff;border-color:#437296}.legend{font-size:11px;display:flex;gap:15px;color:var(--muted);flex-wrap:wrap}.legend span:before{content:'●';margin-right:5px}.legend .partial:before{color:var(--green)}.legend .active:before{color:var(--gold)}.legend .review:before{color:var(--gray)}.group{margin:22px 0 25px}.grouphead{display:flex;gap:12px;align-items:center;margin:0 0 12px}.grouphead h2{font-size:15px;margin:0;font-weight:600}.grouphead .rule{height:1px;flex:1;background:var(--line)}.grouphead small{font-size:11px;color:var(--muted)}.grid{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:12px}.card{background:var(--panel);border:1px solid var(--line);border-radius:12px;overflow:hidden;position:relative}.card:before{content:'';display:block;height:3px;background:var(--green)}.card.active:before{background:var(--gold)}.card.review:before{background:var(--gray)}summary{list-style:none;padding:14px 16px 13px;cursor:pointer}summary::-webkit-details-marker{display:none}.cardhead{display:flex;justify-content:space-between;gap:8px;align-items:flex-start}.card h3{font-size:16px;line-height:1.4;margin:0;font-weight:600}.badge{font-size:10px;white-space:nowrap;color:var(--green);background:#203d36;padding:2px 6px;border-radius:5px;margin-top:1px}.active .badge{color:var(--gold);background:#423725}.review .badge{color:var(--gray);background:#303947}.done{font-size:12px;line-height:1.65;margin:11px 0;color:#d0dfed;min-height:60px}.next{display:flex;gap:6px;font-size:11px;line-height:1.55;color:var(--muted);border-top:1px solid var(--line);padding-top:9px;min-height:66px}.next label{color:var(--gold);white-space:nowrap}.more{font-size:10px;color:#7f9eb5;margin-top:10px;display:flex;justify-content:space-between}.card[open] .more:after{content:'−'}.more:after{content:'+'}.details{border-top:1px solid var(--line);padding:12px 16px;background:#111d29;font-size:11px;color:var(--muted)}.details code{overflow-wrap:anywhere;display:block;font:11px/1.7 ui-monospace,SFMono-Regular,monospace;margin-bottom:5px}.details a{color:var(--blue);display:block;overflow-wrap:anywhere;margin-top:5px}.unreviewed{border:1px dashed #425268;border-radius:11px;color:var(--muted);padding:14px 18px;font-size:12px;margin-top:23px}.unreviewed strong{color:var(--text);font-weight:500}footer{display:flex;justify-content:space-between;gap:20px;border-top:1px solid var(--line);margin-top:20px;padding-top:16px;color:#8fa5ba;font-size:11px}footer a{color:var(--blue)}.hidden{display:none!important}.empty{color:var(--muted);padding:30px;text-align:center}@media(max-width:1150px){.grid{grid-template-columns:repeat(2,minmax(0,1fr))}.done,.next{min-height:0}.stamp{display:none}}@media(max-width:650px){main{padding:24px 18px}.grid,.context{grid-template-columns:1fr}h1{font-size:27px}.legend{gap:8px}.top{display:block}.controls{align-items:flex-start}footer{display:block}}
@media print{body{background:#fff;color:#111}.card,.context>div{break-inside:avoid;background:#fff}button{display:none}main{padding:10px}.grid{grid-template-columns:repeat(2,1fr)}.details{display:none}.done,.next,.intro,.note,.micro,footer,.unreviewed{color:#444}}
</style><main>
<div class="top"><div><div class="eyebrow">BINARY → RUST · EVIDENCE SNAPSHOT</div><h1>Tiny Glade · 模块恢复进度</h1><p class="intro">围绕 <b>country-core / systems & resources</b>，按实际业务模块展示。绿色表示列出的局部业务链已通过回归；完整模块与可玩游戏仍未全部验收。</p></div><div class="stamp"><strong>2026.10.18 · __TIME__ CST</strong><br>B 发行 RNE / 匹配 PDB<br>进度快照，不自动追踪后续修改</div></div>
<div class="context"><div><div class="micro">最新核心库回归 · SNAPSHOT-BATCH-R1</div><div class="value"><b>2454</b>通过 · 0 失败</div><div class="note">保存批次无漂移；并行GPU源漂移1项。含已有测试。</div></div><div><div class="micro">正在攻坚</div><div class="value">快照读回 · 实际保存 / 宿主闭环</div><div class="note">State / Roof 原版解码与真实owner接线并行。</div></div><div><div class="micro">完整游戏验收</div><div class="value" style="color:var(--gold)">尚未闭合</div><div class="note">生产世界主图、GPU 同步与全部工具仍有缺口。</div></div></div>
<div class="controls"><div class="filters"><button class="active" data-filter="all">全部模块</button><button data-filter="核心编辑">核心编辑</button><button data-filter="世界内容">世界内容</button><button data-filter="交互与反馈">交互与反馈</button><button data-filter="资源与运行">资源与运行</button></div><div class="legend"><span class="partial">局部回归通过</span><span class="active">正在恢复</span><span class="review">待整模块复核</span></div></div>
<div id="groups"></div>
<div class="unreviewed"><strong>其余模块：已有实现，尚无本轮完整验收结论。</strong><br>__OTHER__<br>当前目录盘点：systems __NSYS__ + resources __NRES__ = __NALL__ 个 Rust 文件；文件数、TODO 数、测试数均不换算完成百分比。</div>
<footer><span>保留手工重构 · 在现有模块补功能 / 修算法 · 不建立第二套业务状态</span><span>点击模块卡片查看源码映射与证据 · <a href="__REPORT__">恢复报告</a> · <a href="__LESSONS__">经验沉淀</a></span></footer>
</main><script id="snapshot" type="application/json">__DATA__</script><script>
const data=JSON.parse(document.getElementById('snapshot').textContent);const root=__ROOT__;
const esc=s=>s.replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
const href=p=>'file://'+root+'/'+p.split('/').map(encodeURIComponent).join('/');
const groups=['核心编辑','世界内容','交互与反馈','资源与运行'];
document.getElementById('groups').innerHTML=groups.map((g,i)=>`<section class="group" data-group="${g}"><div class="grouphead"><h2>0${i+1} / ${g}</h2><div class="rule"></div><small>点开查看证据</small></div><div class="grid">${data.rows.filter(r=>r.group===g).map(r=>`<details class="card ${r.status}" id="${r.id}"><summary><div class="cardhead"><h3>${esc(r.title)}</h3><span class="badge">${esc(data.status_labels[r.status])}</span></div><p class="done">${esc(r.done)}</p><div class="next"><label>待闭环</label><span>${esc(r.remaining)}</span></div><div class="more"><span>源码 / 验证边界</span></div></summary><div class="details">${r.paths.map(p=>`<code>${esc(p)}</code>`).join('')}<div style="margin-top:8px;color:#eaf0f6">证据与执行记录</div>${r.evidence.map(p=>`<a href="${href(p)}">${esc(p.split('/').pop())}</a>`).join('')}</div></details>`).join('')}</div></section>`).join('');
document.querySelectorAll('[data-filter]').forEach(b=>b.addEventListener('click',()=>{document.querySelectorAll('[data-filter]').forEach(x=>x.classList.toggle('active',x===b));document.querySelectorAll('[data-group]').forEach(x=>x.classList.toggle('hidden',b.dataset.filter!=='all'&&x.dataset.group!==b.dataset.filter));}));
window.progressSnapshot=data;
</script></html>'''
page=page.replace('__NSYS__',str(data['core_inventory']['systems_rs'])).replace('__NRES__',str(data['core_inventory']['resources_rs'])).replace('__NALL__',str(sum(data['core_inventory'][k] for k in ['systems_rs','resources_rs']))).replace('__TIME__',data['generated_at'][11:16]).replace('__OTHER__',html.escape(data['other_scope'])).replace('__REPORT__',(HERE/'README.md').as_uri()).replace('__LESSONS__',(HERE/'RECOVERY_LESSONS.md').as_uri()).replace('__DATA__',json.dumps(data,ensure_ascii=False).replace('</','<\\/')).replace('__ROOT__',json.dumps(str(PROJECT)))
(HERE/'module-progress.html').write_text(page)
VIS.mkdir(parents=True,exist_ok=True)
(VIS/'tiny-glade-module-progress.html').write_text(page)
print(json.dumps({'rows':len(rows),'groups':len(set(r['group'] for r in rows)),'output':str(VIS/'tiny-glade-module-progress.html'),'generated_at':data['generated_at']},ensure_ascii=False))
