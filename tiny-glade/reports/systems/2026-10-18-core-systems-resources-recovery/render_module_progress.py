"""Offline PNG overview; detailed evidence remains in the HTML/JSON snapshot."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

HERE=Path(__file__).resolve().parent
VIS=Path('/Users/yuluo/.codex/visualizations/2026/10/18/01a14d1d-e82e-7163-9e5f-a522c4e1e94f')
data=json.loads((HERE/'module-progress.json').read_text())
summary={
 'wall':('新墙 / 栅栏多帧 CPU 链通过\n端点 / 反向 / Pending 已验','复杂合墙、实际宿主与绘制'),
 'history':('录制 / Sync 与 typed 历史已验\n共享快照 ID / pop counter 已修','其他保存类型与完整宿主数据'),
 'terrain':('曲线 / RDP、绘制、拖动、\n回放与真实事件端口已验证','宿主工具接入、GPU 完整同步'),
 'save':('14 域 / 非空分桶已修\nMotomoto 原格式编码已验','真实采集 / 落盘 / 读回全链'),
 'trees':('隐藏 / 路径清树、placement\n命令顺序与六向回放已通过','出生、弯曲与地形变更消费者'),
 'critters':('羊运动、实时快照 / 应用\n真实 ECS 组件身份已统一','出生、完整生命周期与 RNG'),
 'clutter':('Bush 真实映射、位排序 / hash\n脏格发布与 8 项专项已过','populate 网格消费者、GPU 同步'),
 'building':('保留手工构造与几何实现\n屋顶 7 字段保存格式已验','屋顶读回及四模块完整复核'),
 'color':('u64 墙 ID、三个 reader 已修\n修复每帧重置颜色表','完整颜色 UI 与 GPU 发布'),
 'ui':('光标位移、聚焦、模式发布\nArrow 双 reader 动画已验证','实际输入 / 相机、焦点与出生'),
 'onboarding':('三种地形提示、hover、\nprofile / completion 已验证','实际宿主调用新增配置入口'),
 'audio':('19 处 stop 改用实际时钟\n修复初始化与无法停止问题','全局声源生命周期与调度'),
 'collision':('命中 / TOI / miss、姿态、\n地形 sphere 与停止规则已验','全局句柄、完整装饰物谓词'),
 'prefab':('完整 48B / audio bounds 保留\n真实资产初始化已局部验证','其余出生 / 过滤、全部场景效果'),
 'masks':('Mask / Rocky 等原构造已修\n72 producer 主链局部已验','全部 readback / 场景消费者'),
 'host':('窗口 / 资产与多个 GPU 已跑\n实际编辑调度图已通过','世界主图、gameplay 与可玩闭环'),
}
BG='#0b1420';PANEL='#142230';LINE='#2b3c4d';FG='#edf3f8';MUTED='#a4b6c8'
GREEN='#7cdaad';GOLD='#f0c176';GRAY='#b0b9c7';BLUE='#8abdf7'
font_path='/System/Library/Fonts/Hiragino Sans GB.ttc'
font_bold='/System/Library/Fonts/STHeiti Medium.ttc'
def font(size,bold=False):return ImageFont.truetype(font_bold if bold else font_path,size)
im=Image.new('RGB',(1800,1420),BG);d=ImageDraw.Draw(im)
def text(x,y,s,size=20,fill=FG,bold=False):d.text((x,y),s,font=font(size,bold),fill=fill,spacing=8)
def wrap(s,size,maxwidth):
 f=font(size);out=[]
 for paragraph in s.split('\n'):
  line=''
  for c in paragraph:
   if d.textlength(line+c,font=f)>maxwidth and line:out.append(line);line=c
   else:line+=c
  out.append(line)
 return '\n'.join(out)
def box(rect,fill=PANEL,outline=LINE,radius=14):d.rounded_rectangle(rect,radius,fill,outline,width=1)
text(48,29,'TINY GLADE  /  BINARY → RUST',17,GREEN,True)
text(48,62,'按项目模块 · 恢复进度图',43,FG,True)
text(48,122,'country-core / systems + resources 及已涉及的上层消费者',21,MUTED)
text(1457,49,'2026.10.18',22,FG,True)
text(1457,83,data['generated_at'][11:16]+' CST · 证据快照',16,MUTED)
box((48,169,613,252));box((627,169,1192,252));box((1206,169,1752,252))
text(68,181,'保存批次源码无漂移 · 独立 GPU 源在 run 中变更 1 项',12,MUTED)
text(68,207,'2454 核心 + 1 编辑图通过 · 0 失败',22,GREEN,True)
text(647,181,'当前攻坚',14,MUTED)
text(647,207,'快照读回 · 实际保存 / 宿主闭环',22,FG,True)
text(1226,181,'完整游戏验收',14,MUTED)
text(1226,207,'尚未闭合',22,GOLD,True)
text(49,268,'● 局部回归通过',16,GREEN)
text(252,268,'● 正在恢复',16,GOLD)
text(415,268,'● 待整模块复核',16,GRAY)
text(1150,268,'绿色仅覆盖卡片所列范围，不代表模块完成',16,MUTED)

groups=['核心编辑','世界内容','交互与反馈','资源与运行']
cardw=414; gap=16
for gi,g in enumerate(groups):
 top=309+gi*247
 text(48,top,f'0{gi+1}  /  {g}',20,FG,True)
 d.line((263,top+18,1751,top+18),LINE,1)
 for ci,row in enumerate(r for r in data['rows'] if r['group']==g):
  x=48+ci*(cardw+gap);y=top+39;bottom=y+191
  col={'partial':GREEN,'active':GOLD,'review':GRAY}[row['status']]
  box((x,y,x+cardw,bottom))
  d.rounded_rectangle((x+1,y+1,x+cardw-1,y+4),2,col)
  title=row['title']
  if row['id']=='building':title='建筑构造 / 屋顶 / 楼梯 / 常春藤'
  if row['id']=='ui':title='输入 / 模式 / UI / Arrow'
  text(x+16,y+14,title,19 if len(title)<19 else 17,FG,True)
  text(x+16,y+45,data['status_labels'][row['status']],13,col)
  done,nxt=summary[row['id']]
  text(x+16,y+72,wrap(done,18,cardw-32),18,'#d6e2ed')
  d.line((x+16,y+137,x+cardw-16,y+137),LINE,1)
  text(x+16,y+149,'待',13,GOLD)
  text(x+42,y+147,wrap(nxt,15,cardw-60),15,MUTED)

d.line((48,1310,1752,1310),LINE,1)
text(48,1325,'其他模块已有实现，尚待本轮逐链复核：camera / backend / steam / weather / photo / timelapse / forest / water / toolbar 等。',16,MUTED)
text(48,1354,'保留手工重构，增量补功能和修算法；文件数、测试数、原执行案例数不换算完成率。',16,MUTED)
text(48,1382,'详版可按模块筛选、展开源码与证据链接。主链 / GPU 成果来自相应独立任务，未计为本轮独占。',13,'#7e99b0')
out=VIS/'tiny-glade-module-progress.png';im.save(out)
im.save(HERE/'module-progress.png')
print(out)
