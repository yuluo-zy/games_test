import { Client } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/index.mjs';
import { StdioClientTransport } from 'file:///C:/Users/liyu/AppData/Roaming/npm/node_modules/rea-agents/node_modules/@modelcontextprotocol/client/dist/stdio.mjs';
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const root = 'D:/game/reverse-engineering/tiny-glade';
const output = root + '/evidence/render-recovery/parallel-batch-02/graph-compiler';
const target = root + '/workspace/input/tiny-glade.exe';
const sha256 = createHash('sha256').update(readFileSync(target)).digest('hex');
if (sha256 !== 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03') throw Error('Identity mismatch');
const configuration = JSON.parse(readFileSync('D:/tools/reverse-engineering/verification/server-config.json', 'utf8')).re_mcp_ghidra;
const transport = new StdioClientTransport({ ...configuration, env: { ...process.env, ...configuration.env }, stderr: 'pipe' });
transport.stderr?.on('data', data => writeFileSync(output + '/mcp-stderr.log', data, { flag: 'a' }));
const client = new Client({ name: 'tiny-glade-graph-compiler-r2', version: '2' });
const report = { sha256, target, target_execution: false, database_mutation_requested: false, actual_connection: false, results: [] };
let opened = false;
const call = async (name, arguments_) => {
  const result = await client.callTool({ name, arguments: arguments_ }, { timeout: 180000 });
  report.results.push({ name, arguments: arguments_, result });
  writeFileSync(output + '/mcp-report.json', JSON.stringify(report, null, 2));
  return result;
};
try {
  await client.connect(transport, { timeout: 60000 });
  report.actual_connection = true;
  writeFileSync(output + '/mcp-catalog.json', JSON.stringify(await client.listTools(), null, 2));
  const databases = await call('list_databases', {});
  console.log(JSON.stringify(databases));
  const open = await call('open_database', { file_path: target, database_id: 'tg_render_arch', run_auto_analysis: false });
  if (open.isError) throw Error(JSON.stringify(open));
  opened = true;
  await call('wait_for_analysis', { database: 'tg_render_arch' });
  const identity = await call('get_database_info', { database: 'tg_render_arch' });
  console.log(JSON.stringify(identity));
  const identityText = JSON.stringify(identity);
  if (!identityText.includes(sha256) && !identityText.includes('tiny-glade.exe')) throw Error('Database identity not established');
  for (const address of ['0x140be78b0', '0x1402c3d00', '0x1402c1d40']) {
    const result = await call('decompile_function', { database: 'tg_render_arch', address });
    let data = result.structuredContent;
    if (!data) for (const item of result.content ?? []) if (item.type === 'text') {
      try { data = JSON.parse(item.text); } catch {}
    }
    if (data?.decompiled_code) writeFileSync(output + '/' + address + '.c', data.decompiled_code);
    console.log(JSON.stringify({ address, isError: result.isError ?? false }));
  }
} catch (error) {
  report.error = String(error);
  console.error(String(error));
  process.exitCode = 1;
} finally {
  if (opened) try { await call('close_database', { database: 'tg_render_arch', save: false }); } catch (error) { report.close_error = String(error); }
  await client.close();
  writeFileSync(output + '/mcp-report.json', JSON.stringify(report, null, 2));
}
