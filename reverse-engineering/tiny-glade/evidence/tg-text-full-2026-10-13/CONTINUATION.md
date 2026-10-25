# 后续闭合：anchor、opacity、font init、ECS

原rect的第4参数是float opacity，调用原utils私有scale helper，只改最低8位alpha、RGB高24位不变。新color.rs复用标准float clamp/cast，仅恢复该私有调用策略，未实现pow或通用色彩库。rect_with_opacity复用同一helper。原发行memcpy IAT桥接后20项原rect指令成功，候选逐word全匹配；保留初始两次失败为历史，不再以它们判断功能未恢复。另原96B scalar88项bit边界/NaN/Inf/颜色保留执行成功，新增候选测试待根统一门禁。

原render第3参数asm r8/movsd表明Vec2 extent，不是整数align。真实offset为center-[extent.x,fragment.height]*0.5。新增render_in_box保持该center/extent语义；bounds.y在box重排后更新。原prepare闭包先align=None、WrapWord、size(max_width,FLT_MAX)、shape(false)，generate闭包当alignSome时改line attrs和size(box_extent)，随后shape(true)。这些控制流已恢复，top-left render单列独立适配。

原init_font_system locale缺失字符串为en-US，db为fontdb::Database::new，对应泛型族TimesNewRoman/Arial/ComicSansMS/Impact/CourierNew。保持new_with_locale_and_db，不改为Cosmic::new_with_fonts（后者会更改generic defaults/系统字体行为）。原Cosmic fork强制平台fallback/unix，同源依赖直接执行default-family→script(locale)→common→other顺序；真实发行CJK与color-emoji测试均验证glyph_id非0、NotoSansSC/NotoColorEmoji face。

TextFragment派生Component、TextRenderer派生Resource由原版本Bevy0.16直接提供；独立World spawn/get/despawn+resource测试通过。原分叉repo/commit仍unknown，不把vanilla trait运行说成分叉完整等价；没有手写engine。原register函数bounded decomp已取，涉及600B tg_text::TextFragment自身注册（类型name地址142ced778），未据调用列表凭空增加required类型。

恢复GPU对象的独立扩展reset_glyph_cache清缓存/atlas generation，rerender重新产生全部pending上传；测试验证。old CPU 6+新9项已实际通过；第16项原opacity边界候选刚新增，最后Cargo等待target锁超时，不算通过，由根统一重跑。孤儿PID32584经启动时间/命令行重查已退出，未停止无关进程。

## RTL 外部验证补充

C:/Windows/Fonts/arial.ttf仅作外部验证字体，未改原初始化或fallback。新增Hebrewשלום用例由原Cosmic完整整形/字形/atlas链处理，4个非zero glyph、RTL段、UTF8逻辑cluster顺序对应x严格下降均通过。Cargo shell超时后发现最新test exe已编译完成，--list含新RTL用例；直接执行最新11业务tests全部通过，executable/source/font SHA见cpu-business-11-direct.result.json与rtl-direct-test.result.json。总测试现为17（原6+新11），根门禁另跑Clippy/GPU。未找到存活的本agent cargo/rustc，无需清理无关任务。
