// 只建立发行二进制身份与提供器绑定，不触发全程序反编译。
import { Client } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import { parse } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/smol-toml/dist/index.js';
import { readFileSync, writeFileSync } from 'node:fs';
const config = parse(readFileSync('C:/Users/liyu/.codex/config.toml', 'utf8')).mcp_servers.rea;
const client = new Client({ name: 'tiny-glade-baseline', version: '1' });
const transport = new StdioClientTransport({ command: config.command, args: config.args, env: { ...process.env, ...config.env }, stderr: 'pipe' });
transport.stderr?.on('data', data => process.stderr.write(data));
const report = { target: 'D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne', executed: false, deepAnalysisRequested: false, results: {} };
try {
  await client.connect(transport, { timeout: 30000 });
  for (const [name, args] of [['open_binary', { path: report.target, provider_id: 'ghidra' }], ['binary_session', {}], ['close_binary', {}]]) {
    const result = await client.callTool({ name, arguments: args }, { timeout: 60000 });
    report.results[name] = result;
    if (result.isError) throw new Error(`${name}: ${JSON.stringify(result)}`);
    console.log(JSON.stringify({ tool: name, ok: true }));
  }
  report.ok = true;
} catch (error) {
  report.ok = false;
  report.error = String(error);
  process.exitCode = 1;
} finally {
  await client.close();
  writeFileSync('D:/game/reverse-engineering/tiny-glade/evidence/rea-binding.json', JSON.stringify(report, null, 2));
}
