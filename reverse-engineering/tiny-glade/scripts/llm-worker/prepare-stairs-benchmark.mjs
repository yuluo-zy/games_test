import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const out = path.join(root, 'evidence/llm-worker-2026-10-19');
fs.mkdirSync(out, { recursive: true });
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
const input = path.join(root, 'evidence/country-core-deep/history/decomp/decomp-0x140af75b0.txt');
const raw = fs.readFileSync(input);
const original = raw.toString('utf8').split(/\r?\n/);
const lineMap = [];
const lines = original.flatMap((line, i) => {
  if (!line.trim()) return [];
  lineMap.push({ compact: lineMap.length + 1, original: i + 1 });
  return [line.replace(/\r$/, '')];
});
const symbolsPath = path.join(root, 'evidence/country-core-deep/history/public-symbols-history.json');
const syms = JSON.parse(fs.readFileSync(symbolsPath, 'utf8'));
const addresses = new Set([...lines.join('\n').matchAll(/(?:FUN_|func_0x000)([0-9a-f]{9})/g)].map(m => '0x' + m[1]));
addresses.add('0x140af75b0');
const symbols = syms.filter(s => addresses.has(s.va.toLowerCase()));
function pe(filename) {
  const b = fs.readFileSync(filename), pe = b.readUInt32LE(0x3c), opt = pe + 24;
  if (b.toString('ascii', pe, pe + 4) !== 'PE\0\0' || b.readUInt16LE(opt) !== 0x20b) throw Error('Expected PE32+');
  const imageBase = Number(b.readBigUInt64LE(opt + 24));
  const table = opt + b.readUInt16LE(pe + 20), n = b.readUInt16LE(pe + 6);
  const sections = Array.from({length:n}, (_,i) => { const s = table + i*40; return {name:b.toString('ascii',s,s+8).replace(/\0/g,''), va:b.readUInt32LE(s+12), size:b.readUInt32LE(s+16), offset:b.readUInt32LE(s+20)}; });
  function at(va, length) { const rva=va-imageBase; const s=sections.find(s=>rva>=s.va&&rva+length<=s.va+s.size); if(!s) throw Error('VA outside file-backed section'); return b.subarray(s.offset+rva-s.va,s.offset+rva-s.va+length); }
  const pdata=sections.find(s=>s.name==='.pdata');
  let range;
  for(let o=pdata.offset;o+12<=pdata.offset+pdata.size;o+=12) {
    if(b.readUInt32LE(o)+imageBase===0x140af75b0) {range={start:0x140af75b0,end:b.readUInt32LE(o+4)+imageBase};break;}
  }
  if(!range) throw Error('Function not found in exception directory');
  return {b,at,range,filename};
}
const exe = pe('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.exe');
const rne = pe('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');
const functionBytes = exe.at(exe.range.start, exe.range.end-exe.range.start);
const originalBytes = rne.at(rne.range.start, rne.range.end-rne.range.start);
if(!functionBytes.equals(originalBytes)) throw Error('Patched/original function differs; resolve before dispatch');
const constant = rne.at(0x142930d70,4);
const sLines = [...new Set(symbols.map(s => `${s.va}: ${s.demangled}`))];
const cLines = [`Original RNE bytes at VA 0x142930d70: ${constant.toString('hex')}; IEEE754 little-endian f32 = ${constant.readFloatLE(0)}.`, `Function VA 0x${exe.range.start.toString(16)}..0x${exe.range.end.toString(16)} (${functionBytes.length} bytes): original RNE and patched EXE bytes equal; sha256=${sha(functionBytes)}. This is a static identity check, not a runtime behavior test.`];
const packet = {
  schema_version:1, task_id:'B-replay-stairs-long-v1', task:'Recover the bounded behavioral contract of replay_stairs, including all control-flow outcomes and side effects. Propose independently dispatchable evidence/review work, not shared-state integration.',
  target:{build:'B', program:'tiny-glade.exe', function_va:'0x140af75b0', scope:'original game-owned country_core::systems::history::replay_stairs', ownership_evidence:'S1'},
  limits:['No candidate Rust, human algorithm digest, or grading checklist supplied.', 'Only retained decompilation, retained PDB public labels and fresh static original-byte observations are supplied.', 'No original execution has been observed in this task; decompilation may contain artifacts.'],
  sources:[{id:'D1',kind:'retained_decompilation',path:path.relative(root,input).replaceAll('\\','/'),sha256:sha(raw),line_count:lines.length,text:lines.map((s,i)=>`D1:L${i+1} ${s}`).join('\n')},{id:'S1',kind:'retained_PDB_public_symbols',path:path.relative(root,symbolsPath).replaceAll('\\','/'),sha256:sha(fs.readFileSync(symbolsPath)),line_count:sLines.length,text:sLines.map((s,i)=>`S1:L${i+1} ${s}`).join('\n')},{id:'C1',kind:'fresh_original_bytes',line_count:cLines.length,text:cLines.map((s,i)=>`C1:L${i+1} ${s}`).join('\n')}]
};
fs.writeFileSync(path.join(out,'stairs-packet.json'),JSON.stringify(packet,null,2)+'\n');
fs.writeFileSync(path.join(out,'stairs-input-manifest.json'),JSON.stringify({decompilation_sha256:sha(raw),line_map:lineMap,binaries:[exe,rne].map(p=>({path:p.filename,sha256:sha(p.b),range:p.range,function_sha256:sha(p.at(p.range.start,p.range.end-p.range.start))})),constant:{va:'0x142930d70',hex:constant.toString('hex'),f32:constant.readFloatLE(0)},limitations:packet.limits},null,2)+'\n');
fs.writeFileSync(path.join(out,'jobs-v1.json'),JSON.stringify([{id:packet.task_id,packet:'stairs-packet.json',prompt:'../../scripts/llm-worker/review-v1.txt',output:'stairs-v1-result.json',depends_on:[]}],null,2)+'\n');
console.log(JSON.stringify({packet:path.join(out,'stairs-packet.json'),lines:lines.length,original_function_bytes:functionBytes.length,constant:constant.readFloatLE(0),binary_span_equal:true}));
