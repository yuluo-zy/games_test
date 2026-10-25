import { Client } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import { readFileSync, writeFileSync } from 'node:fs';

const base = 'D:/game/reverse-engineering/tiny-glade/evidence/render-recovery/parallel/registry-pool';
const config = JSON.parse(readFileSync('D:/tools/reverse-engineering/verification/server-config.json', 'utf8')).re_mcp_ghidra;
const client = new Client({ name: 'registry-pool-evidence', version: '1' });
const transport = new StdioClientTransport({ ...config, env: { ...process.env, ...config.env }, stderr: 'pipe' });
transport.stderr?.on('data', data => writeFileSync(base + '/mcp-stderr.log', data, { flag: 'a' }));
const report = { actual_connection: false, target_execution: false, results: [] };
const reportName = process.env.TG_REGISTRY_REPORT ?? 'mcp-report.json';
const database = 'tg_render_arch';
async function call(name, args) {
  const result = await client.callTool({ name, arguments: args }, { timeout: 180000 });
  report.results.push({ name, args, result });
  writeFileSync(base + '/' + reportName, JSON.stringify(report, null, 2));
  console.log(JSON.stringify({ name, address: args.address, isError: result.isError ?? false }));
  return result;
}
try {
  await client.connect(transport, { timeout: 60000 });
  report.actual_connection = true;
  writeFileSync(base + '/mcp-catalog.json', JSON.stringify(await client.listTools(), null, 2));
  await call('list_databases', {});
  await call('open_database', { file_path: 'D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe', run_auto_analysis: false, database_id: database });
  await call('wait_for_analysis', { database });
  for (const address of process.argv.slice(2)) {
    const result = await call('decompile_function', { database, address });
    const data = result.structuredContent ?? JSON.parse(result.content.find(item => item.type === 'text').text);
    if (data.decompiled_code) writeFileSync(base + '/' + address + '.c', data.decompiled_code);
  }
} catch (error) {
  report.error = String(error);
  process.exitCode = 1;
} finally {
  try { await call('close_database', { database, save: false }); } catch (error) { report.cleanup_error = String(error); }
  await client.close();
  writeFileSync(base + '/' + reportName, JSON.stringify(report, null, 2));
}
