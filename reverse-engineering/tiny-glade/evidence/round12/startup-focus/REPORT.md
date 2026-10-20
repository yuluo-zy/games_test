# 集中启动链检查点

## 2026-10-19 最新优先级：集中完成启动→装配→主循环

用户明确要求把并行力量集中到这一条主流程。T16/归档等编辑候选冻结留存，不丢弃既有交付；8专项转为会话初始请求/CLI、卸载与Reload、预置树、真实环境/森林/墙drawable startup、相机与步行音频、装配及独立二次会话验收。中央单一Cargo owner。任务记录见 evidence/round12/startup-focus/dispatch.json。T13继续排除。

已中央合入：v23墙候选除stale run_game（21文件）、preplaced15、完整GCC10、会话lifecycle14、LoadUI36/Parsed缺主history修复、真实Started/Finished及Vermeer持久locals、原check_load_completion sticky/unlink36、真实resource-only装配。候选SHA逐一审计，现正fresh all-targets check，不计验收通过。旧v20实际exe运行241s在临时Decorator完整UI装配处B0002失败；其结构stub不得仅改ParamSet后冒充恢复，改为保真实资源并只挂已恢复叶。v20后因Load test类型/私有字段失败已结束，120源变化使它不是新source的验收。

新发现真实启动gap：实际host缺初始NextSession writer；首次intro是v16，必须经过legacy SpawnPreplacedTrees；原墙7全局drawable未生成；init_clearing完整6913B/forest80B producer仍缺；Walk footsteps真实8参数/rock/raster/input算法当前简化，专项恢复中。原Reveal writers分别来自MainLoop交接和Vermeer完成，不在load_game，不能凭request_reveal新造等待。首次非空+同主题二次+Summer→Winter验收未完成。


## 集成审计

- integration-result.json: 46首批文件按候选SHA精确匹配。
- lifecycle-integration-result.json: 14文件；gameplay_startup仅混合EOL不同，归一换行后代码一致，其余精确匹配。
- validation-startup-check-v1.log: 新source首次五package all-targets类型检查，进行中。
- startup-baseline-v20-result.json: 旧已链接exe真实运行失败，不算新source验收。


## 初始请求审计纠正

SessionFlowPlugin.build实际调用于run_game，并在session.rs构造NextSession::LoadSavedGame。仅在tiny-glade/src搜索构造名会漏掉该跨crate嵌套writer；此前“无初始请求producer”判断撤销。保留原单一请求，不额外插入第二份。
