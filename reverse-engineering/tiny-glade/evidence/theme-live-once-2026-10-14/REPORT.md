# 主题服务一次真实请求

用户明确授权一次请求，用实际 Rust reqwest/ReqwestTransport 向原 EVENTS_URL 发起单次 GET，禁用重定向，没有 retry/run 循环，没有请求 blob。

结果：HTTP 200，Content-Length 与实收 body 均为123字节；Date、ETag 和 Cache-Control 可读取，max-age=3600。保存的原响应通过 theme_server_proto::EventsResponse 解析，events有1项；使用同一响应离线回放 ThemeFetcher::fetch_events 也成功，返回Replaced和3600秒刷新间隔。

首次主程序在收到/处理响应后写汇总时，旧 serde_json 1.0.85 不支持 elapsed_ms 的u128，导致报告写入panic。已改为u64，没有重发请求；原响应与headers此前已保存，随后仅运行`--replay`验证和落盘。此处不把离线回放冒充第二次真实请求或完整原游戏运行。

原响应：[events-response.json](events-response.json)；实际头与内容校验：[headers.json](headers.json)；解析结果：[result.json](result.json)；源/响应SHA：[manifest.json](manifest.json)。显式执行示例在实际theme-fetcher/examples/probe_events_once.rs；默认测试不会发请求，同一目录的request-attempted标记阻止重复运行live模式。

这只验证当前events接口可达和当前响应可被恢复协议/producer接受，不代表活动当前激活、blob下载成功或完整主题游戏消费验收。
