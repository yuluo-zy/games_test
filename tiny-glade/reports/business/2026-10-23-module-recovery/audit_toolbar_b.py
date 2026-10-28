"""按 B 源路径提取逐函数审计账；分类来自人工源码审核，不由同名命中判完成。"""
from pathlib import Path
import collections
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
B = ROOT / 'reverse-engineering/tiny-glade/skeleton/B/crates/systems/toolbar/src'
T = 'tiny-glade/crates/systems/toolbar/src/'
H = 'tiny-glade/crates/tiny-glade/src/'
C = 'tiny-glade/crates/country-core/src/'

def profile(paths, purpose, status, limit):
    return dict(paths=paths, purpose=purpose, status=status, limit=limit)

P = {
'lib.rs': profile([T+'lib.rs',H+'toolbar_plugin.rs',H+'mei_host.rs'], '注册资源、事件、启动与逐帧 toolbar/Ui2d 系统', '部分接线', '主工具条已迁入宿主；不能把它当完整 MenuPlugin/demo 注册；configure 只是较小的独立入口。'),
'mei/audio.rs': profile([T+'mei/core.rs',T+'mei/widgets/menu_button.rs',T+'mei/systems.rs'], 'MEI 悬停/点击音效默认值与节点声音合同', '接口迁移待全证', '原 default 返回 0/2；现由控件 set_sounds 与 PlayMenuUiSound 实现；无独立 MeiUiSounds 类型不等于缺功能。'),
'mei/config.rs': profile([T+'mei/config.rs',T+'mei/core.rs'], 'RON schema、语言文本替换、递归建树、模板插槽、字体和配置加载', '实现存在未全判等', '存在真实解析和递归；serde/ron 复用不算空实现。非法输入/全部自定义 visitor 未逐函数原码复跑。'),
'mei/core.rs': profile([T+'mei/core.rs',T+'mei/systems.rs',H+'mei_host.rs',H+'toolbar_plugin.rs'], '节点身份、内容所有权、布局/焦点/输入/绘制和会话生命周期', '实现存在未全判等', '主循环不是空壳；部分 View API 迁为直接字段/类型安全 Any；缺失控件类型不能因通用 NodeContent 可运行而算恢复。'),
'mei/layout.rs': profile([T+'mei/layout.rs'], '尺寸/百分比、盒模型、主交叉轴、间距、扩张、对齐及子节点布局', '实现存在未全判等', '实际有双轴布局和完整五对齐分派；原PDB内联范围证明Rect尺寸是四边分别points_or_zero后做f32加法，已移除臆造的LengthPercentage混合加法。'),
'mei/widgets/decorate.rs': profile([T+'mei/widgets/menu_button.rs'], '悬停时创建装饰相对根节点、图片与 z 层', '已迁移实现未全判等', 'decorate_hovered 已在既有 menu_button 中实现并有 Escape Continue 消费者；不能按原文件缺失判未恢复。'),
'mei/widgets/tweaker.rs': profile([T+'mei/widgets/tweaker.rs',H+'gui/settings.rs',H+'gui/gui_controls.rs'], '设置控件增减/格式化、布尔/枚举/列表/浮点步长、提示确认与输入阶段', '控件已恢复、页面未闭合', '原浮点/布尔/枚举/列表、提示、真实左右按钮和capture/bubble阶段已实现；216例浮点原指令及实际控件树测试通过。Settings顶部/视频/键位入口仍缺，不能由leaf通过标页面完成。'),
'mei/widgets.rs': profile([T+'mei/widgets.rs',T+'mei/widgets/menu_button.rs'], 'Label/MenuButton/Image/Frame/Pill/TabSwitcher/ActionHint 控件和 View', '具体内容已补、整模块未验收', 'file图片实际上传/缓存、ActionHint动作源/轮换/图片、TabSwitcher及hotkey提示已恢复，新增资源Arc随MEI快照保留；真实OS输入和Steam SDK平台验收仍未执行。'),
'menu/mod.rs': profile([T+'menu/mod.rs',H+'run_game.rs',H+'gameplay_startup.rs',C+'resources/gui.rs'], 'Menu/Welcome 插件注册及菜单 Next→Current/Prev 状态推进', '部分接线', 'advance_menu_state 已迁 core；Welcome/Load/LUT 有注册，损坏告警已恢复及注册；NewSession/每日/保存错误、Settings整页仍待闭合；Credits与许可证按用户要求删除。'),
'menu/credits.rs': profile([], '原制作人员页面与滚动控件', '用户删除范围', '用户明确删除整个页面；原证只作历史留存。'),
'menu/load.rs': profile([T+'menu/load.rs',T+'menu/load_ui.rs',C+'resources/session.rs'], '存档缩略图滚动/导航/选中、元数据、删除/备份/覆盖/Reload事务与错误反馈', '实现存在未全判等', '有实际滚动状态机和磁盘业务，ui 已set36注册；不能用局部历史通过声称所有错误/备份/OS流程完成。'),
'menu/welcome.rs': profile([T+'menu/welcome.rs',H+'run_game.rs'], '首次欢迎、动画、确认、profile记录与后续状态', '实现存在未全判等', '实际模板和set36接线存在；当前用户暂停真实输入验收，不把头部通过记录扩大到完整窗口。'),
'menu/splash.rs': profile([T+'menu/splash.rs',H+'loading_graph.rs'], '加载 splash 原版资源、布局与透明度', '已迁移实现未全判等', '真实加载图消费该UI；有世界/启动GPU证据，不等于所有启动失败分支。'),
'menu/report_bug.rs': profile([T+'menu/report_bug.rs',H+'gui/gui_escape_menu.rs'], '反馈表单、多行文本编辑、附件/提交状态', '用户删除范围', '用户已删除反馈功能及入口，不计当前游戏缺口；残留TextEdit仅共享字符串/开关，没有原编辑和draw，不能称完整控件。'),
'photomode/mod.rs': profile([T+'photomode.rs',T+'photomode/timelapse.rs',C+'systems/backend/photomode.rs',H+'toolbar_plugin.rs'], '摄影进入退出、相机切换/声音、回放、焦点/SSBO响应、marker和步行提示', '部分实现', '拆到现有owner是合法迁移；PROMPT_ROLLERCOASTER真实生产者→摄影事件分支尚缺；焦点真实输入/读回闭环未验。'),
'photomode/photomode_buttons.rs': profile([T+'photomode.rs',H+'toolbar_plugin.rs'], '摄影动作0..17、截图/目录、LUT、focus pin、配置开关和离开', '实现存在未全判等', '有动作整表及真实资源/命令，LUT/截图后续消费者和实际画面仍须分别验证。'),
'photomode/setup_photomode_toolbar.rs': profile([T+'photomode/setup_photomode_toolbar.rs',T+'toolbar_startup.rs'], 'full/demo 摄影工具树、类别/回调/导航', '实现存在未全判等', '原树夹具与当前回调存在；数据树一致不证明所有消费者完整。'),
'photomode/ui_slider.rs': profile([T+'photomode/ui_slider.rs',T+'photomode/slider_math.rs',T+'photomode.rs',T+'resouces_and_components/toolbar_layout.rs'], '13种滑条范围/归一化/复位、拖拽与键盘、knob实体、声音和工具切换', '实现存在未全判等', '有真实分支和会话清理，非空简化壳；各相机union与极值依原证逐项保留，仍无完整窗口验收。'),
'photomode/lut_gallery.rs': profile([T+'photomode/lut_gallery.rs',H+'gameplay_startup.rs',C+'systems/render_loop/lut_gallery.rs'], 'LUT缩略图浏览、悬停声、选择默认/实际LUT、返回和图像配置写回', '已生产接线未全判等', 'gameplay_startup 中实际set36注册，不能只搜run_game误判未接；局部点击测试并非完整GPU预览验收。'),
'resouces_and_components/mod.rs': profile([T+'resouces_and_components/mod.rs',T+'photomode/slider_math.rs',T+'resouces_and_components/toolbar_layout.rs'], '组件/事件schema、滑条默认值/范围、对数滑条双向转换', '实现存在未全判等', '复用真实Bevy组件注册，不能把派生代码当独立业务；类型布局适配与行为等价分别判断。'),
'resouces_and_components/taffy_layout.rs': profile([T+'resouces_and_components/taffy_layout.rs'], 'Taffy树所有权、root/anchor/child、锚点顺序、绝对坐标转换', '实现存在未全判等', '真正调用Taffy并计算4向锚点；依赖算法复用不是空实现，浮点/失败路径未逐实例全判等。'),
'resouces_and_components/taffy_utils.rs': profile([T+'toolbar_systems.rs',T+'resouces_and_components/toolbar_layout.rs'], '居中Style和克隆→修改→设置样式 helper', '接口迁移待全证', 'taffy_mut_style有实际实现，centered可由item_style表达；无同名文件不是业务缺失。'),
'resouces_and_components/toolbar_layout.rs': profile([T+'resouces_and_components/toolbar_layout.rs',T+'resouces_and_components/mod.rs'], '工具树8变体、容器递归、类别/上下文菜单/实体、双向循环导航', '实现存在未全判等', '有FnOnce回调、真正实体和Taffy调用，row/button以构造表达式替代小封装；不要求私有ABI复刻。'),
'resouces_and_components/undo_redo_layout.rs': profile([T+'resouces_and_components/undo_redo_layout.rs'], '构建撤销重做按钮、根锚点、UiHistoryButton与导航', '实现存在未全判等', '有实际HistoryButton实体；完整历史消费者和存档不是setup单函数能证明。'),
'taffy_update.rs': profile([T+'taffy_update.rs',H+'mei_host.rs'], 'size乘数/文本界限→Taffy计算→Ui2dExtent真实位置尺寸', '已生产接线未全判等', 'set44真实消费者已接，非默认Rect替代；之前Welcome尺寸0由原隐藏规则产生，不能判为算法缺失。'),
'ui_interaction.rs': profile([T+'ui_interaction.rs',T+'ui_interaction/hide.rs',T+'ui_interaction/focus.rs',T+'ui_interaction/ordering.rs',T+'ui_presentation.rs',H+'toolbar_plugin.rs'], '隐藏/鼠标与drag事件、父焦点、导航、出现选择锁定动画、声音/提示/绘制顺序', '已生产接线未全判等', '有实际算法/真实事件；部分历史注释过时，不能凭旧未实现字样否定现代码。'),
}
for key,paths,purpose in [
 ('toolbar_button.rs',[T+'toolbar_button.rs',H+'toolbar_plugin.rs'],'按钮保持焦点、AppMode/类别切换、TOD开关及动画事件'),
 ('toolbar_startup.rs',[T+'toolbar_startup.rs',T+'photomode/setup_photomode_toolbar.rs'],'构建full/demo工具树、摄影树和历史树并发布布局资源'),
 ('toolbar_systems.rs',[T+'toolbar_systems.rs',T+'undo_redo_systems.rs',H+'toolbar_plugin.rs'],'4种对齐、toolbar/历史自动隐藏、输入抢占和激活几何'),
 ('toolbar_update.rs',[T+'toolbar_update.rs',H+'toolbar_plugin.rs'],'分辨率/用户比例、按钮展开缩放、根与上下文间距'),
 ('third_party_licenses_server.rs',[T+'menu/credits.rs'],'绑定loopback、只读许可证文件HTTP响应、后台线程服务'),
 ('text.rs',[H+'mei_host.rs',H+'loading_graph.rs'],'加载字体、按窗口缩放、准备/绘制文本片段和GPU流上传'),
]: P[key]=profile(paths,purpose,'已迁移实现未全判等','源码存在实际业务；宿主接线见具体消费者，不以同名函数或单元测试判整个业务验收。')
for key,purpose in [
 ('menu/new_session.rs','新建场景、主题/尺寸选择与实际会话创建/加载'),
 ('menu/daily_welcome.rs','每日欢迎、主题/日期相关状态与确认跳转'),
 ('menu/save_error.rs','自动保存失败的通知、重试/目录操作和菜单生命周期'),
 ('menu/corrupted_save_file.rs','损坏存档告警事件的提示页面和确认关闭'),
 ('demo_stuff/demo_show_full_glade_size.rs','demo可建区域边界提示、动画与绘制'),
 ('demo_stuff/demo_show_max_elevation.rs','demo最大高度提示和资源消费'),
]: P[key]=profile([],purpose,'缺少原业务族','未在当前toolbar及宿主找到对应原页面/系统；底层会话/保存/警告事件存在不等于原UI已恢复。')

P['menu/credits.rs']=profile([], '原制作人员页面与滚动控件', '用户删除范围', '用户2026-10-24明确删除整个Credits页面；源码、入口、菜单变体及专属测试均移除，不列恢复阻碍。')
P['third_party_licenses_server.rs']=profile([], '原许可证HTTP服务', '用户删除范围', '用户2026-10-24明确删除许可证模块；不恢复按钮、服务或端口。')
P['menu/corrupted_save_file.rs']=profile([T+'menu/corrupted_save_file.rs',H+'gameplay_startup.rs'],'损坏存档事件打开模态、确认关闭与再次打开','已接线待联合验收','原UI与按钮槽已恢复，原模板/字体CPU消费者测试通过；原System::initialize140ff8a20确认Local默认1；真实窗口仍未验收。')

ROLES={
'ui':'构建页面/处理该页输入、权威状态与事件', 'build':'插件资源/事件与调度注册',
'prepare_layout':'准备文字/形状/滚动状态并发布内容尺寸', 'render':'实际绘制或发布共享绘制指标',
'pre_update':'上一帧内容输入边沿更新', 'process_input_action':'处理该内容的输入事件或动作阶段',
'on_input':'处理该内容输入', 'interactive':'决定该内容能否参与拾取',
'default':'原类型默认状态', 'create_node_content':'创建持久内容所有者', 'drop':'释放该类型持有资源',
'as_any':'安全内容类型投影', 'as_any_mut':'可变内容类型投影', 'type_name':'内容类型标识',
'deref':'透传真实节点/内容引用','deref_mut':'透传可变节点/内容引用',
'content':'取当前节点的具体内容','content_mut':'初始化或取可变具体内容','get_content_impl':'校验并读取具体内容',
'get_content_mut_impl':'初始化/校验并取可变内容', 'node':'读取真实节点','node_mut':'修改真实节点',
'with_node_mut':'在真实节点上执行调用者修改', 'with':'对上下文执行调用者闭包', 'into_inner':'取出原节点上下文','as_inner_mut':'借用原节点上下文',
'as_id':'名字/值生成稳定节点身份','to_typed_id':'构建类型化节点身份','to_global_id':'转换全局节点身份',
'id':'返回节点身份','mei':'访问同帧MEI所有者', 'reset_keeping_content':'跨帧重建节点但保留内容/旧布局',
'ignore_input':'屏蔽节点及布局传播后的后代输入','force_interactive':'允许空内容根参与输入阻挡','default_focus':'标记默认导航焦点',
'steal_focus':'要求本帧夺取焦点','lose_focus_on_invalid_nav':'焦点无效时的原节点策略','with_nav':'设置节点导航资格',
'used':'读取鼠标点击或键盘激活位（不是空函数）','hover':'读取悬停位','has_focus':'读取导航焦点位',
'interaction':'读取原交互状态位','prev_computed_layout':'读取上一帧位置尺寸','block_input':'写阻挡输入标志',
'set_sounds':'设置节点悬停/点击音效','sounds':'节点音效设置适配','fullscreen':'创建全屏阻挡根',
'fullscreen_without_block_input':'创建不阻挡输入的全屏根','safe_frame':'创建安全宽度根并发布帧事件',
'wide_safe_frame':'创建宽安全区域根','new_at_pos':'在指定位置尺寸锚点创建根','new_relative':'相对真实参考节点创建根',
'new_root':'节点复用与每帧根事件发布','reuse_or_create_node':'复用内容/旧布局或初始化新节点',
'mei_pre_update':'输入动作重建、上一帧节点交互/焦点/音效和帧节点回收',
'mei_post_update':'内容prepare→拓扑布局→导航→实际draw→输入边沿清理',
'clear_state':'清空原会话/MEI临时状态','post_update':'原后更新阶段封装','prepare_content':'调用真实内容prepare并更新固有尺寸',
'perform_layout':'执行真实尺寸/锚点/子树布局','compute_z_order_and_toposort':'根事件遍历、z与继承flag传播和拓扑顺序',
'on_key_event':'转发真实键盘载荷','on_clipboard_paste':'转发剪贴板输入','update_nav':'MEI方向/默认焦点/祖先兄弟子树导航',
'process_input_action_handlers':'沿父链分发动作capture/bubble阶段','find_node_path':'构建根至焦点的真实父链路径','update_parent_mapping':'从当前节点child关系重建父映射',
'child':'类型和局部名字生成子节点','child_with_content':'创建子节点并初始化指定内容','child_with_global_id':'按确定身份连接子节点',
'into_child':'建立子节点并返回新上下文','into_child_with_global_id':'按确定身份建立子节点并返回上下文',
'has_global_id':'检查本帧节点是否存在','expect_child_with_global_id':'按原失败策略获取已存在节点',
'into_expect_child_with_global_id':'检查并返回已有子节点上下文','layout':'设置/读取真实布局','with_layout':'设置布局并返回上下文',
'expand':'设置布局扩张权重','set_default_focus':'节点默认焦点设置适配',
'draw_rect':'中心尺寸转换为原点并输出真实矩形流', 'load_fonts':'遍历安装目录并加载实际字体',
'load_configs':'读取原RON模板集合','load_font_table':'读取原字体角色表','read_node_config_from_file':'用原RON Options解析模板',
'deserialize_optional':'原可省略字段反序列化','instantiate_ui_template_impl':'递归创建原模板frame/label/slot',
'localize_ui_text_from_config':'替换t!(...)语言占位符','template':'在原节点下实例化模板','template_with_id':'以调用者ID实例化原模板',
'into_slot':'返回模板命名slot','slot':'查找模板命名slot','with_slot':'对命名slot执行调用者业务闭包',
'param_to_value':'对数滑条归一化参数转权威值','value_to_param':'权威值转对数滑条参数','default_value_01':'取得原滑条默认归一化值',
'range':'取得原滑条数值范围','slider_range':'原滑条范围表','slider_widget_size':'原滑条控件几何',
'offset':'读取内容当前滚动偏移','max_scroll_pos':'计算滚动上限','min_scroll_pos':'计算滚动下限',
'background_color':'设置滚动背景颜色','scroll':'构建持久滚动内容节点','thumbnail_scroll':'构建共享缩略图指标和导航节点',
'clicked_item':'读取实际点击缩略图','hovered_item':'读取悬停缩略图','hovered_item_pos':'读取悬停项位置',
'abandon_focus':'取消缩略图键盘焦点','initial_focused_item':'设置初始键盘焦点','tile_height':'配置缩略图大小',
'gap':'配置主轴间距','run_count':'设置/计算缩略图每排数','horizontal':'设置主轴为横向','lock':'锁定该内容输入',
'nav_scroll_speed':'设置/计算导航滚动速度','advance_menu_state':'推进Next→Current并保存Prev',
'increase':'按该原Tweakable类型增加值','decrease':'按该原Tweakable类型减少值','format':'按原显示语义格式化值',
'delta':'读取输入产生的增减量','text':'读写提示/编辑内容（依具体类型）','confirm':'确认提示/编辑状态',
'hotkey':'绑定真实动作并创建原提示子树','make_hotkey_tooltip':'构建原按键/手柄动作提示',
'force_highlight':'强制按钮高亮','inactive_opacity':'设置未悬停时图像不透明度','mute_click':'关闭点击音效',
'color':'设置原文字颜色','weight':'设置原字体字重','align':'设置原文字对齐','metrics':'设置原字体尺寸',
'family':'设置字体角色','max_width':'设置文字宽度约束','tint':'设置图片颜色','blur_extend':'设置图片模糊延展标志',
'back_button':'构建返回按钮及原声音/尺寸','menu_button':'构建可导航菜单按钮','menu_button_no_nav':'构建不参与导航的菜单按钮',
'menu_button_with_id':'按指定身份创建菜单按钮','settings_button':'构建原设置行/按钮','tab_switcher':'构建原标签页增减输入内容',
'action_hint':'构建原真实按键或手柄提示内容','image':'构建命名/文件图片内容节点','label':'构建持久文字内容',
'label_with_id':'以指定身份构建文字节点','frame':'构建形状背景内容节点','space':'构建原占位间距尺寸节点',
'draw':'写原形状流','on_click':'读取/处理原点击激活','size':'设置控件尺寸','points':'读取尺寸原始f32','points_or_zero':'百分比项对minimum贡献0',
'width':'取得左右边距和','height':'取得上下边距和','size_vec2':'合并矩形宽高','from_points':'从绝对长度构建尺寸类型',
'is_horizontal':'判断原布局主轴','vertical':'设置纵向布局或纵向导航','padding':'设置4边padding','space_between':'设置均分空白布局',
'align_center':'主轴居中','xalign_center':'交叉轴居中','center':'原点加半尺寸计算中心','from':'原类型转换/派生胶水',
'centered':'复用居中Style组合','taffy_mut_style':'克隆现有Style→调用者修改→set_style触发布局失效',
'add_node':'创建真实Taffy节点builder','style_mut':'修改真实Taffy样式','style':'读取真实Taffy样式','roots':'读取根列表',
'roots_mut':'修改根列表','root_mut':'修改确定根','children':'取得真实子节点','root':'登记根布局节点',
'anchored_to':'登记相对锚点、边向、gap、order','child_of':'连接真实父子Taffy节点','make_layout_absolute':'将子树局部位置转换到绝对位置',
'setup_screen_and_toolbar_nodes':'构建屏幕和工具条真实根','add_toolbar_item':'递归恢复八变体工具树并调用真实回调',
'button':'工具项构造API','row':'工具容器构造API','add_contextual_menu':'关联类别上下文树和父焦点实体',
'link_up_nav':'连接原实体双向导航','contextual_menus':'返回当前上下文菜单记录','link_up_cyclic_list_nav':'设置环形前后导航',
'spawn_drawable':'创建真实UI可绘制实体','setup':'构建撤销/重做根和实际按钮实体',
'load_session_meta':'读取存档元数据及原失败策略','display_save_file_load_error':'创建/显示真实存档读取错误',
'show_contextual_buttons':'按真实存档行构建加载/删除/备份操作按钮','resolve_new_glade':'将原主题/布局选择解析为实际新会话',
'spawn_third_party_license_http_server':'Credits条件下绑定本地许可证服务并记录URL/错误','http':'创建监听socket/路径服务所有者','run':'请求循环读取许可证文件并响应HTTP',
}
ROLES.update({'add_category_button': '创建类别按钮实体/根节点、执行真实回调、关联上下文记录',
 'clear': '清空该原内存/编辑会话所有者',
 'constructor$0': '构建该原枚举变体的有效载荷',
 'compute_min_size_with_children': '从内容、绝对边距、主交叉轴子节点计算最小尺寸',
 'debug_hide_ui': '调试UI关闭时清零实际UI尺寸防止绘制与拾取',
 'decorate_hovered': '悬停时创建原相对装饰根/图像和z层',
 'demo_show_full_glade_size': 'demo完整建造区域的显示、动画及输入/状态消费',
 'demo_show_max_elevation': 'demo达到最大编辑高度时的原提示',
 'disable_input': '提交/模式变化时禁用具体内容输入',
 'input_allowed': '查询具体内容的输入门控',
 'do_photomode': '进入退出摄影、相机命令/音频、timelapse、焦点请求/读回及提示',
 'flush_text_render': '消费真实文字和形状流、上传GPU缓冲及图像资源',
 'hide_ui': '菜单/Welcome/摄影门控及焦点失效，发布交互可见性',
 'iter_subtree': '沿实际Taffy/MEI子树过滤和查找导航候选',
 'mei_load_configs': '注册/调用原字体和RON模板加载',
 'parabola': '原动画抛物线数值表达式',
 'sinc': '原动画sinc数值表达式',
 'photomode_buttons': '摄影动作字节整表→真实相机/截图/目录/LUT/配置/菜单副作用',
 'pill_button': '构建真实Pill图像按钮、尺寸、声音和导航',
 'place_childen': '按原swizzle分派两轴子节点布局',
 'place_childen_impl': '处理主交叉轴尺寸、padding/gap、expand、justify与百分比',
 'prepare_text_fragments': '准备实际字体glyph/文本尺寸，供Taffy与绘制读取',
 'register_required_components': 'Bevy派生组件登记/依赖钩子，不是单独游戏算法',
 'render_taffy_ui_to_widgets': '实际Ui2dExtent→拾取注册→交互/导航→click/drag/history事件',
 'render_text_fragments': '按实际UI矩形、颜色、字体流绘制文字实体',
 'serialize': '该原schema的序列化字段合同（依赖算法由serde复用）',
 'setup_photomode_toolbar': '构建full/demo摄影工具项、类别/回调/实体导航',
 'setup_toolbar': '构建普通full/demo类别树与上下文菜单并连接导航',
 'slider_val_01': '读取真实相机/配置/笔刷/橡皮权威值并转归一化位置',
 'sort_contextual_buttons_draw_order': '计算上下文按钮实际绘制顺序',
 'switch_app_mode_button': '处理合法按钮点击、模式命令和选择动画/状态',
 'switch_toolbar_category_button': '处理类别切换、有效last_selected模式和选择状态',
 'taffy_finalize_element_size': '原base×一次性缩放×用户比例×分辨率比例，与文本bounds取大并复位一次性值',
 'taffy_perform_layout': '按窗口尺寸/真实缩放执行Taffy布局和原锚点',
 'taffy_update_drawables': '将实际Taffy位置/尺寸发布到Ui2dExtent',
 'text_edit': '创建共享真实编辑内存的原输入节点',
 'text_startup_system': '加载实际字体并初始化文字渲染资源',
 'tod_v2_button': '真实按钮边沿切换TOD目标/动画状态并发原反馈',
 'toggle_icon': '原可切换图标的内容/布局/绘制与输入',
 'toolbar_alignment': '按4种边缘对齐重设根/上下文样式与锚点',
 'toolbar_auto_hide': '以真实鼠标/keep/菜单/按下状态更新隐藏动画、激活区域和声音',
 'toolbar_button_keep_focus': '维持有效按钮焦点并处理冷却/模式变化',
 'toolbar_button_size_and_padding_update': '原BackOut缩放与主/上下文菜单gap写入',
 'toolbar_setup': '重建Taffy并发布普通/摄影/历史三个真实布局资源',
 'toolbar_size_mult_update': '按窗口与用户配置计算工具条尺度权威值',
 'toolbar_steal_mouse_input': '按真实UI激活矩形抢占鼠标并通知核心输入消费者',
 'try_remove_file': '删除存在的临时反馈文件并按原失败策略记录',
 'tweaker': '原Tweakable类型的设置行构建、值增减和提示确认',
 'tweaker_impl': '原设置行共同模板/输入/值展示业务',
 'tweaker_w_font_size': '按指定字体大小构建原设置行',
 'ui_animation_state_update': '推进显隐/选择动画，保持原状态边沿语义',
 'ui_appear_animation': '按原CubicOut出现值更新一次性尺寸缩放',
 'ui_focus_animation': '原焦点缩放与滑条/复位/自定义尺寸分流',
 'ui_focus_parent_if_self_in_focus': '两阶段祖先传播keep并避免循环',
 'ui_locked_animation': '锁定按钮的原震动/缩放及生命周期',
 'ui_nav_update': '真实实体导航、方向互斥、显式链接/父子树/确认/返回和焦点权威',
 'ui_notify_hints_if_toolbar_in_focus': '有工具keep时发布规范ToolbarUiIsInFocus提示仲裁事件',
 'ui_select_animation': '点击选择脉冲、一次性缩放与完成后组件删除',
 'ui_send_focus_begin_events': '维护会话聚焦成员并只对新keep=1发事件',
 'ui_show_if_parent_in_focus': '父keep发布子可见性，摄影相机限制另判',
 'ui_slider': '真实滑条knob生成/删除、拖拽/键盘导航、权威值与声音/类别写回',
 'ui_slider_reset': '按原默认配置/相机/笔刷和各kind恢复权威值',
 'ui_sounds': '按原focus→click→drag顺序发实际规范音效事件',
 'undo_redo_alignment': '历史按钮3处真实样式写入及4向对齐',
 'undo_redo_auto_hide': '历史按钮激活几何与原隐藏/声音状态机',
 'update_text_renderer_scale': '根据真实窗口高度发布文字renderer比例'})

ALIAS={'render':'draw','process_input_action':'process_input_action','localize_ui_text_from_config':'localize_label',
'load_session_meta':'meta','text_startup_system':'load_text_fonts','flush_text_render':'flush_mei_frame',
'link_up_nav':'link_up_nav','slider_val_01':'slider_current_position','slider_range':'range',
'compute_z_order_and_toposort':'perform_layout','post_update':'mei_post_update','prepare_content':'mei_post_update',
'update_parent_mapping':'build_parent_links','reset_keeping_content':'reuse_or_create_node',
'with_slot':'with_slot','scroll':'credits_scroll','delta':'delta','points_or_zero':'points_or_zero'}

def leaf(name):
    # 先取外层函数，避免泛型参数内的 :: 把方法名截断。
    outer=name.split('<',1)[0]
    return outer.rsplit('::',1)[-1]

def methods(paths, method):
    result=[]
    for path in paths:
        p=ROOT/path
        if not p.is_file(): continue
        for i,line in enumerate(p.read_text().splitlines(),1):
            if re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?fn\s+'+re.escape(method)+r'\b',line):
                result.append({'path':path,'line':i,'match':'同名/显式迁移候选；不是原码等价证明'})
    return result

def decide(file, name, prof):
    status=prof['status']; purpose=ROLES.get(leaf(name),'该源模块的业务函数/辅助分支；具体语义尚需原体核实')
    if 'closure$' in name: purpose='父业务内部闭包：'+prof['purpose']
    detail=prof['limit']; kind='按业务族检查当前源码；未逐实例机器码判等'
    if not name.startswith('toolbar::'):
        return '外部/归属重叠', '标准库/依赖单态或相邻源码内联记录，不作为toolbar自有业务恢复', '保留原来源，不将其重写为私有算法。','元数据分类'
    if 'report_bug::' in name:
        return '用户删除范围', purpose, '反馈生产入口已按用户要求删除；残留TextEdit是部分实现，无编辑/draw，不计可交付功能。','源码审核，按用户范围排除'
    if 'menu::credits::' in name or 'third_party_licenses_server::' in name:
        return '用户删除范围', purpose, '用户明确删除Credits与许可证模块；原证仅历史保留，不再恢复或计阻塞。','用户范围裁定'
    if file=='mei/widgets/tweaker.rs' or any(v in name for v in ['TweakerContent','TweakerView']):
        return '控件已恢复、消费者继续闭合', purpose, P['mei/widgets/tweaker.rs']['limit'],'新鲜原体、原指令216例及实际控件树，具体边界见STATUS'
    if any(v in name for v in ['TabSwitcher','ActionHint']):
        return '具体内容已恢复、平台未验收', purpose, 'Tab动作24/25、阶段、delta清理；ActionHint真实动作源、两秒轮换、文件图片；Steam SDK/OS不由mock调度测试替代。','原函数+31条来源静态合同+CPU调度，见STATUS'
    if file=='mei/widgets.rs':
        if any(v in name for v in ['impl$14::','impl$15::','impl$21::','impl$22::']):
            return '具体内容已恢复、平台未验收',purpose,'原TabSwitcher/ActionHint内容及真实图片绘制已补；平台Glyph来源仍需真实SDK验收。','新鲜原体与当前内容实现'
        if 'hotkey' in name or 'make_hotkey_tooltip' in name:
            return '输入与提示已接线',purpose,'Pill/MenuButton同步同一原动作字节的两个投影；真实相对提示根/ActionHint已建树，18仅跳过提示；无OS验收。','原指令+控件/MEI调度测试'
        if any(v in name for v in ['impl$9::prepare_layout','impl$9::render']):
            return '文件分支已恢复、局部GPU通过',purpose,'成功缓存、失败重试、实际解码/上传/索引与原后备纹理；Arc owner随待执行渲染快照保留。当前源码真实GPU上传/descriptor/像素回读/排队帧owner最终释放已通过；不替代整模块资源或性能验收。','原4266B体+实际上传/descriptor/readback探针'
        if leaf(name)=='settings_button':
            return '原设置按钮已恢复',purpose,'原68c790完整体确认同MenuButton身份、导航、0/2音效与40px/Weight700，已补直接消费者入口。','新鲜原体及对应指令参数'
    if file=='mei/core.rs':
        if any(v in name for v in ['::wide_safe_frame','::steal_focus','::lose_focus_on_invalid_nav']):
            return '原入口已补、联合未验收',purpose,'wide_safe_frame复用WideSafe事件；steal_focus写原bit3；lose_focus与with_nav由原PDB物理别名683c60核定同一bit5。','原flag消费者、PDB别名及现有根事件合同'
        if any(v in name for v in ['::as_any','::type_name','::to_typed_id','::to_global_id','::as_id']):
            return '语言/接口适配待全证', purpose, 'Any/GlobalId和固定TypeId高字节直接在现有owner表达；无需仿私有vtable，仍受具体内容是否存在约束。','源码适配检查'
    if any(v in name for v in ['::deserialize','::serialize','::visit_','::expecting']):
        return '依赖派生/schema待全证', '该原配置/枚举/字段的序列化、访问器或错误合同', '复用serde/ron是正确边界；核对schema与原可选字段，未逐个恶意/错误输入全判等，不记独立业务恢复。','schema源码检查，派生算法复用'
    if 'closure$' in name:
        return status, '父业务中的闭包/分支：'+prof['purpose'], detail+' 闭包捕获/每个分支没有单独新增判等，继承父业务未决。','归属父业务审核，非独立恢复计数'
    return status,purpose,detail,kind

files=[]; rows=[]; src_hashes={}
for p in sorted(B.rglob('*.rs')):
    file=str(p.relative_to(B)); assert file in P,file
    t=p.read_text(); prof=P[file]; original=[]
    src_hashes[str(p.relative_to(ROOT))]=hashlib.sha256(p.read_bytes()).hexdigest()
    for path in prof['paths']:
        q=ROOT/path; assert q.is_file(),path
        src_hashes[path]=hashlib.sha256(q.read_bytes()).hexdigest()
    for m in re.finditer(r'^// 函数符号: ([^\n]+)\n',t,re.M):
        b=t[m.end():m.end()+1300];name=m.group(1)
        rec=re.search(r'记录形式: ([^\n]+)',b); rv=re.search(r'编译实例 RVAs: ([^\n]*)',b)
        status,purpose,detail,review=decide(file,name,prof)
        method=ALIAS.get(leaf(name),leaf(name));targets=methods(prof['paths'],method)
        row={'id':f'B-TB-{len(rows)+1:04}', 'original_file':file,'original_symbol':name,
             'original_metadata_line':t.count('\n',0,m.start())+1,
             'record_kind':rec.group(1) if rec else None,'original_rvas':rv.group(1) if rv else None,
             'purpose':purpose,'status':status,'review_boundary':review,'current_candidates':targets,
             'current_owner_files':prof['paths'],'finding':detail,'module_accepted':False}
        rows.append(row);original.append(row['id'])
    files.append({'original_file':file,**prof,'records':original})

original=ROOT/'B版本内容/tiny-glade.rne';sha=hashlib.sha256(original.read_bytes()).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
report={'verdict':'B toolbar 未完整恢复；即使暂停OS验收，源码/业务/接线仍有明确缺口。',
 'method':'从B原始源路径与函数注释独立提取记录；当前源码人工按业务族检查；新只读Ghidra复核关键原体。没有使用旧恢复率或测试数推算。',
 'scope_limit':'全量记录已入账≠每个函数机器码全域判等。字段含review_boundary，未决不升级为完成。测试按STATUS的指定源码与范围解释，不执行真实OS输入。',
 'binary_sha256':sha,'pdb_sha256':'2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911',
 'counts_not_progress':{'original_files':len(files),'all_metadata_records':len(rows),'toolbar_names':sum(x['original_symbol'].startswith('toolbar::') for x in rows),'status_records':dict(collections.Counter(x['status'] for x in rows))},
 'fresh_native_evidence':['toolbar-b-native-audit.json','toolbar-b-controls-native-audit.json','toolbar-image-hint-native.json','toolbar-tabs-native.json','toolbar-steam-page-native.json','toolbar-steam-rest-native.json','toolbar-steam-handles-native.json','toolbar-tweakers-native.json','toolbar-settings-followup-native.json','toolbar-settings-slots-native.json','toolbar-fullscreen-native.json','toolbar-settings-button-native.json','toolbar-rect-points-native.json','toolbar-tutorial-event-identity.json','toolbar-input-sound-order-native.json'],
 'files':files,'functions':rows,'source_hashes':src_hashes,
 'audit_evidence_hashes':{name:hashlib.sha256((OUT/name).read_bytes()).hexdigest()
     for name in ['audit_toolbar_b.py','toolbar-b-native-audit.json','toolbar-b-controls-native-audit.json',
                  'toolbar-b-audit-original.asm','pill-hotkey-audit-probe.rs',
                  'pill-hotkey-audit-build.log','pill-hotkey-audit-build-absolute.log',
                  'pill-hotkey-audit-build-matched.log','pill-hotkey-field-audit-build.log',
                  'pill-hotkey-field-audit-run.log']},
 'historical_pre_repair_probe':{'kind':'现存toolbar构建产物的实际构造/setter→consumer字段交接，非OS输入或pre_update执行',
          'artifacts':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in
              [Path('/tmp/pill-hotkey-audit-probe')] if p.is_file()},
          'build_exit':0,'run_exit':101,'expected_action':19,'observed_hotkey':19,'observed_consumer_action':40,
          'scope_limit':'没有借此宣称完整原二进制执行或全源码冻结构建。原字段语义由RNE指令独立核定。',
          'build_command':'rustc --edition=2024 reports/business/2026-10-23-module-recovery/pill-hotkey-audit-probe.rs --crate-name pill_hotkey_audit_probe -L dependency=/Users/yuluo/Desktop/game/tiny-glade/target/debug/deps --extern toolbar=/Users/yuluo/Desktop/game/tiny-glade/target/debug/deps/libtoolbar-2837e05e7be8a352.rlib -o /tmp/pill-hotkey-audit-probe',
          'run_command':'/tmp/pill-hotkey-audit-probe',
          'earlier_attempts':'显式链接country_core的三次尝试E0463，均未执行，不作业务失败；最终字段交接探针只链接实际toolbar，构建成功后才复现失败。'}}
(OUT/'toolbar-b-function-audit.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
head='# B 版 toolbar 逐目录、逐函数源码审计（2026-10-24滚动更新）\n\n**M01仍未完成，不是只差OS验收。** 本表按原B文件保存全部元数据记录，分类来自人工按原证/源码复核；不从测试数或同名命中推算恢复率。用户暂停真实OS输入，并明确删除反馈入口、许可证和整个Credits页面；这些记录保留为用户删除范围，不计阻塞或成果。\n\n## 当前已修与待闭合\n\n- 已补file:图片真实解码/上传/缓存/资源owner、ActionHint及hotkey提示、TabSwitcher和Tweaker完整内容族。31条Steam动作来源、controller传参、持续按住和右键拖拽阈值已修。平台SDK/OS仍未验收。\n- 原216例浮点Tweaker、126例开发者解锁、420例视频配置应用掩码通过；依赖桥、NaN比较边界及原码覆盖见对应native元数据。数字计数不是完成率。\n- 已补损坏存档告警事件/模板/确认/再次打开与生产注册；设置音量、游戏/教学、UI/自定义光标、开发者及相机控制消费者已恢复，4项设置CPU测试已通过（含实际reset_hints与保存请求/光标事件）；不含整页或磁盘落盘。设置顶层/视频/键位页面仍缺，不能标整个页面完成。\n- 继续阻碍：NewSession真实选择到Reload、DailyWelcome、自动保存错误页面、摄影PROMPT writer→事件、原demo两个提示、Settings未闭合子链；之后复验历史/保存/重入/GPU资源。Credits/许可证不再进入计划。\n- Rect原内联是四边先points_or_zero再f32加法；已移除旧臆造的LengthPercentage混合加法。完整资源/渲染/OS验收与局部CPU/原指令证明分开记账。\n\n历史Pill字段交接失败探针保留，修复已使两个投影同步，不能把旧失败当当前源码结论。每次运行本脚本仅更新分类与源码指纹，不执行测试。详细阶段、日志与经验见STATUS.md。深入review另见[当前审查](toolbar-deep-review.json)：已执行原输入函数反证长按连发差异，并核定摄影提示查询码和键盘载荷错误；设置/新建可进入状态不等于目标页面已恢复。该审查不使用本表记录数作为完成率。\n\n## B目录映射与各文件审核\n\n'
lines=[head,'| B原文件 | 当前owner | 业务 | 结论/限制 |','|---|---|---|---|']
for f in files:
 owners='<br>'.join('`'+x+'`' for x in f['paths']) or '未找到同职生产实现'
 lines.append('| `'+f['original_file']+'` | '+owners+' | '+f['purpose']+' | **'+f['status']+'**：'+f['limit']+' |')
lines+=['','## 每条原函数记录','','以下每条均保留，不把闭包/单态合并成完成率；外部依赖记录也保留并排除出私有业务。具体调用角色无法由元数据裁定的条目明确写未决。实现候选只表示当前承载位置，非同名即等价。完整名称过长时仍原样保存在JSON。','']
for f in files:
 lines+=['### `'+f['original_file']+'`','','| ID / 原函数（含单态） | RVA | 功能/业务 | 当前承载 | 结论 |','|---|---|---|---|---|']
 for r in rows:
  if r['original_file']!=f['original_file']:continue
  name=r['original_symbol'];short=name.removeprefix('toolbar::')
  target='<br>'.join('['+x['path']+':'+str(x['line'])+']('+str(ROOT/x['path'])+':'+str(x['line'])+')' for x in r['current_candidates'][:3])
  if not target:target='；'.join('`'+x+'`' for x in r['current_owner_files']) or '无对应业务族'
  safe=lambda v:str(v).replace('|','&#124;').replace('\n',' ')
  lines.append('| '+r['id']+'<br>`'+safe(short)+'` | '+safe(r['original_rvas'] or '仅内联/声明，无独立RVA')+' | '+safe(r['purpose'])+' | '+target+' | '+safe(r['status'])+'；'+safe(r['finding'])+' |')
 lines.append('')
lines+=['## 后续收口顺序','','1. 收口Settings顶层、视频与键位及实际配置保存；复验已恢复的教学和光标消费者。','2. 完成NewSession/每日/保存错误原页面到真实副作用。Credits与许可证按用户要求删除。','3. 补摄影PROMPT及Demo提示。','4. 验证历史、保存、Reload和GPU卸载。真实OS输入仍按用户选择暂停，不据此提前宣布模块完成。','','原证、桥接与失败记录见STATUS和同目录native结果；生成脚本不构成语义判等。','']
(OUT/'B-FUNCTION-AUDIT.md').write_text('\n'.join(lines))
print(json.dumps(report['counts_not_progress'],ensure_ascii=False))
