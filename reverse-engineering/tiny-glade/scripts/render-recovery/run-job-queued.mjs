// 已被 run-lanes.mjs 取代，仅保留用于复现 2026-10-12 及之前的既有证据报告。
//
// 本文件的"并发 agent 共用一个 database/session_key"前提已经不成立：RE-MCP 现在按
// database_id 给每个 worker 独立进程与独立 Ghidra 项目，mcp-evidence-job.mjs 里按
// session_key 的整作业互斥锁也已删除，因此这里不会再等到 'busy' 事件，退避重试分支
// 成为死路。新作业用 run-lanes.mjs（每条 lane 一个 database_id，N 路并行）。
//
// 原说明：共享 Ghidra 会话的排队执行器：并发 agent 共用一个 database/session_key。
// Ghidra 项目同一时刻只允许一个打开会话，因此这里在 "session lock busy" 上退避重试。
// 只读语义完全沿用 mcp-evidence-job.mjs；本文件不扩大工具白名单。
// Usage: node run-job-queued.mjs JOB.json REPORT.json [--config CONFIG] [--max-wait-ms N]
import {readFileSync} from 'node:fs';
import {parse} from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import {run} from './mcp-evidence-job.mjs';

const [jobFile, output, ...flags] = process.argv.slice(2);
if (!jobFile || !output) throw new Error('Usage: JOB.json REPORT.json [--config CONFIG] [--max-wait-ms N]');
let configPath = 'C:/Users/liyu/.codex/config.toml';
let maxWaitMs = 30 * 60 * 1000;
for (let i = 0; i < flags.length; i += 2) {
  if (flags[i] === '--config') configPath = flags[i + 1];
  if (flags[i] === '--max-wait-ms') maxWaitMs = Number(flags[i + 1]);
}
const parsed = configPath.endsWith('.json') ? JSON.parse(readFileSync(configPath, 'utf8')) : parse(readFileSync(configPath, 'utf8'));
const server = 're_mcp_ghidra';
const config = parsed.mcp_servers?.[server] ?? parsed[server];
if (!config) throw new Error('Configured MCP server not found: ' + server);
const job = JSON.parse(readFileSync(jobFile, 'utf8'));

const started = Date.now();
let attempt = 0;
for (;;) {
  attempt += 1;
  const report = await run(server, job, output, config);
  const busy = (report.lock_events ?? []).some(e => e.event === 'busy');
  if (report.ok === true) {
    console.log(JSON.stringify({ok: true, attempt, waited_ms: Date.now() - started, results: report.results.length}));
    process.exitCode = 0;
    break;
  }
  if (!busy) {
    console.error(JSON.stringify({ok: false, attempt, error: report.error ?? 'unknown', results: report.results.map(r => ({tool: r.tool, ok: r.ok, error: r.error}))}));
    process.exitCode = 1;
    break;
  }
  if (Date.now() - started > maxWaitMs) {
    console.error(JSON.stringify({ok: false, attempt, error: 'gave up waiting for the shared Ghidra session lock'}));
    process.exitCode = 1;
    break;
  }
  const backoff = Math.min(15000 + attempt * 5000, 45000);
  console.log(JSON.stringify({waiting: 'session lock busy', attempt, backoff_ms: backoff}));
  await new Promise(resolve => setTimeout(resolve, backoff));
}
