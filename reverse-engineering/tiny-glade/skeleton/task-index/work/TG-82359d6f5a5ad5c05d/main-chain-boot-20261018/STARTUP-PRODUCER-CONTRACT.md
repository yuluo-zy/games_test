# B 原版启动载入 producer：2026-10-18 独立复核

目标仅B，原oracle tiny-glade.rne SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。索引虽含A/C，本合同不迁移它们。生产源码正为真实GPU执行冻结；candidate未应用。

## 原始事实

- insert_resources@140940d80：140940e35写AppMode tag0，140940e45调用具名insert@14153d770；140940e4a写PrevAppMode tag0，140940e5a调用insert@141536030。当前canonical Editing(EditSubject::None)对应0。
- AppLoadingState::fmt@140308a00读取长度表1429b8448={6,4,6}与相对串表1429b8460，得0=Splash、1=Game、2=Reload。init_state@1403258c0插State值0；原PostStartup闭包14032b1a0写NextState byte1（Pending(Game)；NextState Unchanged niche3）。不能以布尔loading替代这个状态身份。
- SessionFlowPlugin@140b47840插空SavedSessionList与NextSession::create(nilUUID,kind_niche=8000000000000001)，不插ActiveSession。Startup刷新SavedSessionList。NextSession的kind是启动请求类型，**不同于LoadGameManager.source**，不能把同一niche值视为同一业务枚举。
- startup::load_game_session@14166e7d0真实13参（PDB与fresh函数体双证），无Local<bool>。开头入队Commands.remove_resource<NextSession>，memcpy请求0xa0B并take其owner。原完整body18791B；Ghidra函数metadata误为399B，fresh反编译仍覆盖后续body，必须用RNE范围核。
- NextSession请求tag1读取最近session；tag2指定session；缺最新档或失败后尝试new，默认主题Autumn(3)。原attempt队列可以按失败路径继续，不是发一个告警即返回所有请求。
- new session生成UUID，create_dir失败与write meta失败均panic。不是best-effort复制；NewSessionStarted事件在进入真实new路径时产生，已有会话分支产生SessionLoaded，不能无条件同时发两事件。
- GameProfile.new_glade_count是usize，serialize@140a658a0最后为serialize_field<usize>；startup@14166f893从+1d8读qword。当前u32会截断原值。
- new默认模板来源：@14166f861判断demo选择assets/starting-builds/{full|demo}。count==0→join intro(@14166fd5f)。count>0→read_dir(root)，只收目录，排除UTF8名称intro(@14166fef8..ff1b)，升序排序后取(count-1)%len(@14167016f开始)。原空目录panic。
- 后续构造选中build/history.json为LoadGameSource path。copy_dir_all@141740c00调用@1416710dd用于选中来源的snapshots到session/snapshots。copy helper本身没有排序/intro过滤，过滤发生于调用者；旧恢复把整棵full目录复制到session不等价。
- startup系统将AppMode写14=LoadGame(@141670前半对应fresh L1290)，替换LoadGameManager owner，state@+48=0；flags@49来自NextSession.ignore_snapshot，@4a/@4b来自attempt，+50清0。ActiveSession原先不存在时Commands.insert_resource，存在时就地替换；当前旧源None时不插是缺口。

## LoadGameManager ctor/consumer 勘误

add_save_load_events_and_resources@1409447e0先开真实Async save线程/channel/UnsavedFileList；它们随后属于AutoSaveState，不能从相邻栈槽推到LoadGameManager。

ctor @140944b6c..bfc建立Manager：

| 偏移 | 已证含义 |
|---|---|
| +00..18 | Vec<PostGameLoadCommand>，len1/cap1，分配32B，首项tag8000000000000000，AppMode payload tag0。即初始SwitchAppMode Editing(None) |
| +18..48 | LoadGameSource，tag8000000000000000表示None（inactive payload不要求全零） |
| +48 | state2，空闲/处理postcommands阶段 |
| +49..4b | ctor不初始化；只有startup重建后才能读取有效flags |
| +4c | 有符号应用计数差；ctor inactive不初始化 |
| +50 | 后续owned值，startup清0；精确类型待定位 |

fresh load_game@14146d030：AppMode!=14早退。state0读source并load/upgrade/snapshot/apply，成功后state1并写有符号计数差；state1按绝对计数发对应输入事件，再state2返回；state2消费postcommands并返回。当前旧Loading0→Loaded2再开始解析的模型不符，首次就发LoadingGameComplete更是伪完成。该事件必须位于真实域恢复及后续完成链之后。

## 当前候选与边界

candidate/startup_session.rs新增原模板选择函数，candidate/game_profile.rs（另一合法claim）修usize，candidate/app_loading_state.rs（另一合法claim）恢复canonical三态。尚未修改生产或宣称它们已接通。

继续工作：把原完整startup请求/attempt及Manager.source具体字段恢复后接OnEnter(Game)，接真实原load consumer。现有snapshot::init_game_state_from_snapshot有已恢复部分，但Roofs/Trees/颜色等仍未完整，不能用空值或忽略Apply发ready。
