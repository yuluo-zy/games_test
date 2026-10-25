from pathlib import Path
import json,re,hashlib,collections,difflib
root=Path('/Users/yuluo/Desktop/game');out=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
base=root/'games_test/tiny-glade'; reports=base/'reports/core-systems-resources-recovery-2026-10-18'
manifest_path=reports/'audio-registration-lifecycle.json';manifest=json.loads(manifest_path.read_text())
before=reports/'before-audio-registration-lifecycle.rs';after=base/'crates/country-core/src/systems/audio/mod.rs'
beforetext=before.read_text();aftertext=after.read_text()
function=lambda s:re.search(r'pub fn add_audio_systems\b.*?\n}',s,re.S).group(0)
refs=lambda s:collections.Counter(re.findall(r'([A-Za-z_][A-Za-z_0-9]*::[A-Za-z_][A-Za-z_0-9]*)::<(?:S|M::Event)>',function(s)))
assert refs(beforetext)==refs(aftertext)
checks=[]
for mod,init,update,state in manifest['domain_pairs']:
 p=base/'crates/country-core/src/systems/audio'/f'{mod}.rs';s=p.read_text()
 ini=re.search(r'pub fn '+init+r'\b.*?\n}',s,re.S).group(0)
 upd=re.search(r'pub fn '+update+r'\b.*?\n\)',s,re.S).group(0)
 pattern=r'\(\s*'+mod+'::'+init+r'::<S>\s*\.run_if\(not\(resource_exists::<'+mod+r'::'+state+r'<S::Event>>\)\),\s*'+mod+'::'+update+r'::<S>,\s*\)\s*\.chain\(\)'
 registration_matches=re.findall(pattern,function(aftertext))
 inserted=(state+'::<S::Event>::new(&core)')in ini and ini.count('commands.insert_resource(')==1
 consumed=('ResMut<'+state+'<S::Event>>')in upd
 resource=bool(re.search(r'#\[derive\(Resource[^\]]*\)\]\s*pub struct '+state+r'<',s))
 assert inserted and consumed and resource and len(registration_matches)==1
 checks.append({'module':mod,'source':str(p),'sha256':sha(p.read_bytes()),'init':init,'condition_state':state+'<S::Event>','inserted_owner_matches':inserted,'consumer_owner_matches':consumed,'resource_derive':resource,'registration_pair_matches':len(registration_matches),'init_source':ini,'consumer_signature':upd})
bevy=Path('/Users/yuluo/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/schedule')
bevyfiles=[bevy/'config.rs',bevy/'condition.rs',bevy/'auto_insert_apply_deferred.rs',bevy/'schedule.rs']
report={'scope':'read-only current registration adapter review; not original global schedule','manifest':{'path':str(manifest_path),'sha256':sha(manifest_path.read_bytes())},'before':{'path':str(before),'sha256':sha(before.read_bytes()),'matches_root_manifest':sha(before.read_bytes())==manifest['before_sha256']},'after':{'path':str(after),'sha256':sha(after.read_bytes()),'matches_root_manifest':sha(after.read_bytes())==manifest['after_sha256']},'generic_system_reference_multisets_equal':True,'generic_system_reference_count':sum(refs(aftertext).values()),'non_generic_ivy_reference_equal':function(beforetext).count('ivy_audio::ivy_audio_init_system')==function(aftertext).count('ivy_audio::ivy_audio_init_system'),'pairs':checks,'bevy_ecs_version':'0.16.0','bevy_local_sources':[{'path':str(p),'sha256':sha(p.read_bytes())}for p in bevyfiles],'production_source_edited':False,'cargo_run':False}
(out/'review-evidence.json').write_text(json.dumps(report,indent=2)+'\n')
(out/'registration-only.diff').write_text(''.join(difflib.unified_diff(function(beforetext).splitlines(True),function(aftertext).splitlines(True),fromfile='retained-before',tofile='current-after'))+'\n')
print({'pairs_checked':len(checks),'system_reference_multiset_equal':True,'generic_references':report['generic_system_reference_count'],'before_matches':report['before']['matches_root_manifest'],'after_matches':report['after']['matches_root_manifest']})
