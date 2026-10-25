from pathlib import Path
import gzip,json,re,hashlib,datetime,shutil
R=Path('/Users/yuluo/Desktop/game');C=R/'games_test/tiny-glade/crates';P=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/history-snapshot-api-review';D=P/'sources';D.mkdir(parents=True,exist_ok=True)
occ=[];snap=[];unit=[];bare=[];mix=[]
for f in C.rglob('*.rs'):
 if 'recovery' in f.parts or 'target' in f.parts:continue
 s=f.read_text()
 if not re.search(r'\bHistory\b|SnapshotCaptureState|HistorySectionJson|ResolvedHistorySnapshot',s):continue
 rel=str(f.relative_to(C))
 if rel.split('/')[0] not in ['country-core','tiny-glade','systems']:continue
 externaltest='/tests/' in '/'+rel or f.name=='tests.rs'
 masked=re.sub(r'/\*.*?\*/|//[^\n]*|r(?P<hashes>\#*)".*?"(?P=hashes)|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\'', lambda m:' '*len(m.group(0)),s,flags=re.S)
 test_ranges=[]
 for hit in re.finditer(r'\#\[cfg\(test\)\]\s*(?:\#\[[^\]]+\]\s*)*(?:pub\s+)?mod\s+\w+\s*\{|\#\[test\]\s*(?:\#\[[^\]]+\]\s*)*(?:pub\s+)?fn\s+\w+[^\{]+\{',masked):
  start=hit.end()-1;depth=1;k=start+1
  while depth and k<len(masked):
   depth+=(masked[k]=='{')-(masked[k]=='}');k+=1
  test_ranges.append((hit.start(),k))
 line=lambda pos:s.count('\n',0,pos)+1
 def test(pos):return externaltest or any(a<=pos<b for a,b in test_ranges)
 us=[{'line':line(m.start()),'text':m.group(0),'test':test(m.start())} for m in re.finditer(r'\bHistory\s*(?:::)?\s*<\s*\(\)\s*>',s)]
 bs=[]
 for m in re.finditer(r'\bHistory\b(?=\s*[>,;)])',s):
  st=s.rfind('\n',0,m.start())+1;en=s.find('\n',m.end());txt=s[st:en if en>=0 else len(s)].strip()
  if txt.startswith(('//','use ')) or 'use ' in txt or 'history::{History,' in txt or ' History,' in txt:continue
  bs.append({'line':line(m.start()),'text':txt,'test':test(m.start())})
 if us:unit.append({'path':str(f),'occurrences':us})
 if bs:bare.append({'path':str(f),'occurrences':bs})
 if us and any(x['test'] for x in bs):mix.append({'path':str(f),'unit_lines':[x['line'] for x in us if x['test']],'bare_test_lines':[x['line'] for x in bs if x['test']]})
 if not(us or bs or 'History<' in s or 'Snapshot' in s and 'systems/save/' in rel or '/resources/history/' in '/'+rel or 'src/startup/run_game.rs' in rel):continue
 dst=D/rel;dst.parent.mkdir(parents=True,exist_ok=True);dst.write_text(s);snap.append({'source':str(f),'snapshot':str(dst),'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'bytes':f.stat().st_size})
I=R/'games_test/reverse-engineering/tiny-glade/skeleton/task-index';claims=[]
for l in gzip.open(I/'files.jsonl.gz','rt'):
 j=json.loads(l);path=j['path']
 if j['version']!='B' or not any(path.endswith(x) for x in ['resources/history/mod.rs','systems/mode_manager.rs','systems/save/load_game.rs','systems/save/save_game.rs','systems/save/save_thread.rs','systems/save/snapshot.rs','startup/mod.rs']):continue
 ident=j['task_id'];fp=I/'claims'/f'{ident}.json';cl=json.loads(fp.read_text()) if fp.exists() else None;claims.append({'task_id':ident,'path':path,'claim':None if cl is None else {k:cl.get(k) for k in ['owner','status','work_dir']}})
j={'at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'production_changed':False,'cargo_executed':False,'native_executed':False,'sources':snap,'source_drift_after_capture':[x['source'] for x in snap if hashlib.sha256(Path(x['source']).read_bytes()).hexdigest()!=x['sha256']],'explicit_unit_History':unit,'bare_default_History':bare,'unit_fixture_and_default_query_mix':mix,'claims_at_review':claims,'test_classifier_boundary':'Lexically balanced cfg(test) modules/test functions plus tests file/path; comments/strings masked. Rows remain textual source inventory, not Rust AST/compiler results; imports can appear as references.','canonical_payload_expected_from_native_owner':'MaybePendingSnapshotId (Arc<AtomicU64>), HistorySnapshotId u64; parent candidate handles exact schema','HistorySnapshots_name_boundary':'Only private snapshot_gc DeserializeSeed plus unrelated utils SaveHistorySnapshots selector; no current runtime ECS HistorySnapshots Resource'}
(P/'api-propagation-inventory.json').write_text(json.dumps(j,indent=2)+'\n');print({'captured_sources':len(snap),'unit_production':[{'path':x['path'],'lines':[y['line'] for y in x['occurrences'] if not y['test']]} for x in unit if any(not y['test'] for y in x['occurrences'])],'mixed_fixture_files':len(mix),'bare_production':[{'path':x['path'],'lines':[y['line'] for y in x['occurrences'] if not y['test']]} for x in bare if any(not y['test'] for y in x['occurrences'])],'drift':j['source_drift_after_capture']})
