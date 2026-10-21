import json,pathlib,hashlib,subprocess,tempfile,shutil,os
root=pathlib.Path('/Users/yuluo/Desktop/game/games_test'); W=root/'reverse-engineering/tiny-glade/skeleton/task-index/work'; D=pathlib.Path(__file__).parent
nav=json.loads((D/'CORE_BUSINESS_CURRENT_REVIEWED_CANDIDATES_20261021.json').read_text())
peer=json.loads((W/'TG-10dd22b74858bd4c2e/decorator-chain-root-20261021/window-host-contract-v1/CANONICAL_WINDOW_CALLERS_20261021.json').read_text())
S=W/'TG-4b6d939d045d508224/clutter-startup-20261020/canonical-startup-call-20261021'; startup=json.loads((S/'delivery.json').read_text())
def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
selected=[]
for x in nav['files']:
 if x['group'] in ('canonical_loader_and_typed_callers','canonical_trap') or x['source'].endswith(('chimney_assemble_bricks.rs','assemble_dormer_roof_tiles.rs')):
  selected.append({k:x[k] for k in ('source','before_sha256','final_candidate_sha256','candidate','ordered_patches','review')})
for x in peer['window_sources']:
 selected.append(dict(source=x['source'],before_sha256=x['before_sha256'],final_candidate_sha256=x['candidate_sha256'],candidate=x['candidate'],ordered_patches=[x['patch']],review=peer['wall_corner_independent_review']['path'] if 'gothic' not in x['source'] else str(pathlib.Path(x['work_dir'])/'SOURCE_REVIEW_LEAD.md')))
selected.append(dict(source=startup['source'],before_sha256=startup['before_sha256'],final_candidate_sha256=startup['after_sha256'],candidate=startup['candidate'],ordered_patches=[startup['patch']],review=str(S/'SOURCE_REVIEW_LEAD.md'),apply_policy='root only method 2-hunk; never replace whole source if production changed'))
sandbox=pathlib.Path(tempfile.mkdtemp(prefix='tg-canonical-minimal-F0-',dir='/private/tmp')); checks=[]
for i,x in enumerate(selected):
 live=root/x['source']; dst=sandbox/x['source']; dst.parent.mkdir(parents=True,exist_ok=True); livehash=sha(live) if live.exists() else None
 assert livehash==x['before_sha256'],(x['source'],'live changed',livehash,x['before_sha256'])
 if live.exists(): shutil.copy2(live,dst)
 logs=[]
 for p in x['ordered_patches']:
  plus=next(l[4:].split('\t')[0] for l in pathlib.Path(p).read_text().splitlines() if l.startswith('+++ '))
  strip=0 if plus==x['source'] else 1
  assert plus=='b/'+x['source'] or plus==x['source'],(plus,x['source'])
  env=dict(os.environ,TMPDIR='/private/tmp')
  r=subprocess.run(['patch',f'-p{strip}','-F0','--batch','-i',p],cwd=sandbox,capture_output=True,text=True,env=env)
  logs.append(dict(path=p,sha256=sha(p),strip=strip,cwd_repo_relative='.',exit=r.returncode,stdout=r.stdout,stderr=r.stderr)); assert r.returncode==0,(p,r.stdout,r.stderr)
 assert sha(dst)==x['final_candidate_sha256']==sha(x['candidate']),x['source']
 x['order']=i+1; x['production_sha256_at_freeze']=livehash; x['ordered_patch_details']=logs
 checks.append(dict(source=x['source'],before=livehash,after=sha(dst),patches=logs))
gothic=next(a for a in peer['window_sources'] if 'gothic' in a['source']); G=pathlib.Path(gothic['work_dir'])
(G/'SOURCE_REVIEW_LEAD.md').write_text('''# Gothic canonical直接调用迁移独立审查

通过此最窄接口迁移的源码独审。完整哥特窗主body仍是旧部分实现，禁止据此挂载visual宿主。

逐hunk核对：仅增加同一DecoratorMeshLibrary资源；保留0x39类型门及required subtype/cache；真实Info.rank构造唯一key，两个required record查询。Arc只clone同owner，不由bbox造geometry；insert_for保真实Wall地址和raw flags，positioned colliders保真实缓存translation/rotation并set_cached_transform。旧数值helpers及其余body未变，未增默认成功/并行表。审查仅证明typed API与已审canonical schema对接及本次副作用，不能把旧无洞/未成形窗算法记为恢复。

before75da019bd17e76d5e9204b62fad91c206688a05c06228af50ce13cd21d2be2bc；after4ed2f8bc6027b2fdcbe9ee9c875f8df02fa3b03249bc67ebd254d781b8806df7。独立临时patch -p1 -F0逐字hash相等。未执行Cargo/runtime；用户目标C未做等价证明。
''')
(S/'SOURCE_REVIEW_LEAD.md').write_text('''# 26-loader真实启动调用独立审查

通过最窄source/接口/注册合同复核。原DecoratorPlugin140342c10→140344de6→140170fb0完整typed PreStartup wrapper证明三资源身份与phase，复用已审1407b66e0完整producer。实际run_game/bridge先装同一AssetMeshLibrary；MeshWatch按live_link创建。直接GPU App入口没走DecoratorPlugin，幂等init两个真实canonical空owner保持既有值；不清库，不造geometry。root/demo捕获已有配置；错误继续panic，不加ready标记、visualhost或无原证调度边。

两个hunk位于root/path移交前，config仅Copy bool读取后结束借用；同一owner实际Asset库直接消费。先canonical+loader五参、最后此2hunk，避免旧签名错配。root只按方法hunk合并，全文before变化时保留root音频修复并重新核hunk。独立F0重放after272bffb28cf9f683fc640fb999164c5918f713c6ed915a787c043ba8f2b226e4。

未实际Cargo或原资产运行；注册不等于加载成功。full52/demo34 keys/同Arc/triangles/bounds/outline以及缺资产真实失败待root执行。
''')
# 全Rust源码只读overlay扫描，不输出无关全体源码。
patterns=['LegacyDerivedDecoratorInfoRecord','.collision_mesh(','.interaction_mesh(']; scan={p:[] for p in patterns}; overrides={x['source']:pathlib.Path(x['candidate']).read_text() for x in selected}
for p in (root/'tiny-glade/crates').rglob('*.rs'):
 rel=p.relative_to(root).as_posix(); src=overrides.get(rel,p.read_text())
 for pat in patterns:
  for n,line in enumerate(src.splitlines(),1):
   if pat in line: scan[pat].append(dict(source=rel,line=n,text=line.strip()))
manifest=dict(scope='Canonical four maps + original26-way PreStartup loader + subtype ID producer + all direct typed callers; no visual host mount',source_count=len(selected),ordered_sources=selected,source_closure_declared=True,cargo_compilation_verified=False,production_changed=False,patch_reproduction=dict(sandbox=str(sandbox),all_fuzz0=True,all_candidate_hash_match=True,source_count=len(checks)),no_new_dependencies=True,dependencies_note='Canonical/window deps already pinned/present. Stairs bitvec/core-smallvec EXCLUDED. Root integrated SharedShape world7699 required by trap.',already_production_prerequisites=peer['already_present_production_dependencies'],world_prerequisite=dict(source='tiny-glade/crates/country-core/src/systems/collision/world.rs',actual_sha256=sha(root/'tiny-glade/crates/country-core/src/systems/collision/world.rs'),root_actual_tests='native-physics collision65/65; log /private/tmp/tg-sharedshape-collision-tests-20261021.log; no business-group runtime execution'),explicit_exclusions=['requests/cache/stairs assembly owners and deps','structure_outline/stairs_state/stairs lazy flag','Curve/WallSpace f32 group','storage niche pair pending independent source review','validator/location/display/Circle.closest84 pending source','any visual host mount'],remaining_algorithm_gaps='Gothic and some retained visual bodies partial. This closes schema/API and actual loader registration, not full visual/cache/history correctness.',root_validation=['locked cargo check current target without visual host mount','relevant system-decorator/country-core tests','real original install full/demo exact52/34 keys, same owner Arc, triangles/bounds/outline','actual gameplay PreStartup required asset failure'],overlay_legacy_scan=scan)
(D/'CANONICAL_MINIMAL_COMPILE_20261021.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n'); (D/'CANONICAL_MINIMAL_F0_REPRODUCTION_20261021.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n')
print('source_count',len(selected),'manifest',sha(D/'CANONICAL_MINIMAL_COMPILE_20261021.json'),'reproduction',sha(D/'CANONICAL_MINIMAL_F0_REPRODUCTION_20261021.json'));print('legacy_overlay',json.dumps(scan,ensure_ascii=False));print('reviews',sha(G/'SOURCE_REVIEW_LEAD.md'),sha(S/'SOURCE_REVIEW_LEAD.md'))
