# 原B RT owner / worker / set44

原RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；PDB来源procs.tsv。父任务只读请求，本轮不改RT源码/宿主source。

* run_game_thread @14059dbef 分配0xb0 align8，@dc07/0e写Arc strong/weak1，@dc16锁0，@dc1e poisonfalse，@dc22 None niche8000000000000000，@dc26 state2，@dc2d busyfalse，@dc34保存[rbx+158]。Inactive payload不得凭空解释成有意义的零字段。
* @1405a0066 从该同一owner取Arc并inc；@a00b6 new=ICF identity14067dd80；@a0100调用真正App::insert_resource<RtBvhBuilderGlueResource>@140138aa0。
* @1405a3fba读rdata1429ffaf0前16B并追加er，原18B名字为bvh_builder_worker。@a4004把[rbx+158]作为closure输入R8，@a403c调用std::thread::Builder::spawn<run_game_thread::closure_env$7>@14028ae90，失败@a6e1d unwrap_failed。PDB唯一原RNE worker call @1400d9914，由15B wrapper1400d9910进入bvh_builder_worker140b24d50。
* 此worker在gameplayApp装配之后、独立loadingApp装配之前启动。不能每App各建线程，也不能用输入参数[rbp+37b8]代替此owner。
* runthread@14059fc70 DL=0x2c，@fc72调用in_set1403c7a30。PDB完整3tuple为prepare_frame_constants、update_rt_world、upload_rt_world，统一set44；tuple非chain，无额外condition（OS独立原RNE已核）。
* 此前消息把@14059f05f/insert140127e40解释成RT是错的；PDB该插入类型是PendingEguiContextResource，已原始指令反证并明确撤销。请只采用上述同[rbx+158]链。

当前重构库/资源架构保持不变；原证据决定单owner、线程和调度语义，不用于复制原底层ABI。
