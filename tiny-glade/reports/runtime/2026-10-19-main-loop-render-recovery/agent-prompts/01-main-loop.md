# 主循环与异步加载生命周期

先读取同目录 COMMON.md。本文件覆盖此前创建6名子agent的所有指令；你现在是单一执行者。

目标：让explicit本机宿主的OS事件→输入→App update→后台App发布→render→错误/退出形成真实闭包。
输入：当前 game_loop.rs、event_loop.rs、run_game.rs::GameplayPreparation；原GameLoop/main_loop和loader保留完整RNE。
你的唯一候选写面：game_loop.rs，以及必要event_loop.rs最窄调用hunk；GameplayPreparation具体实现由启动agent负责。core render_frame归世界图agent，production_renderer归主图agent，不双写。
必须完成：核OS事件/egui/捕获确认/每帧边沿清除顺序；同App/同HostTables暖机交接；错误必须到event_loop真实Result/cleanup；失败不得增加temporal index或记首帧；resize/minimize与OOM重试保持原分类；退出时真实等待worker结束并安全收GPU。当前普通Steam工厂None不是你用假成功替换的理由，独立记录。
接口：与04启动协调cancel_and_join和实际warmup状态；与03世界图协调render_frame的Result/错误类型；与02主图协调本帧submission/present结果。
验收：真实loading→gameplay切换多帧；缺producer/非OOM错误保留原因且不降尺度；取消/加载异常/正常退出不遗留worker或GPU在用资源；原输入顺序回归。验收agent负责独立核查。
