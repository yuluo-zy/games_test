from pathlib import Path
import hashlib,json,difflib
ROOT=Path('D:/game');OUT=Path(__file__).resolve().parent;candidate=OUT/'load-audio-candidate';rows=[];patch=[]
def add(rel,after):
 p=ROOT/rel;before=p.read_text(encoding='utf8');q=candidate/rel;q.parent.mkdir(parents=True,exist_ok=True);q.write_text(after,encoding='utf8');rows.append({'path':rel,'base_raw_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'candidate_raw_sha256':hashlib.sha256(q.read_bytes()).hexdigest()});patch.extend(difflib.unified_diff(before.splitlines(True),after.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
rel='tiny-glade/crates/country-core/src/startup/load_audio.rs';add(rel,(OUT/'load_audio_corrected.rs').read_text(encoding='utf8'))
rel='tiny-glade/crates/country-core/src/systems/audio/game_loading_audio.rs';s=(ROOT/rel).read_text(encoding='utf8');needle='impl<E: EventInstance> GameLoadingAudioState<E> {';assert s.count(needle)==1;s=s.replace(needle,needle+'\n    /// Original load_audio inserts this empty owner before the separate\n    /// init_game_loading_audio Startup consumer starts the snapshot.\n    pub fn empty_after_samples() -> Self {\n        Self { snapshot:None, loading:false, timer:0.0 }\n    }\n');add(rel,s)
rel='tiny-glade/crates/country-core/src/systems/audio/terrain_stairs_audio.rs';s=(ROOT/rel).read_text(encoding='utf8');needle='#[derive(Debug, Default, Clone, Copy)]\npub struct StairsCounter';assert s.count(needle)==1;add(rel,s.replace(needle,'#[derive(Resource, Debug, Default, Clone, Copy)]\npub struct StairsCounter'))
rel='tiny-glade/crates/country-core/src/startup/run_game.rs';s=(ROOT/rel).read_text(encoding='utf8');needle='    // batch1/PreStartup 的 load_audio：追加 game_loading_audio 两个系统。\n    // promise 由音频域在真正的样本载入点提供；这里不伪造已载入状态。\n    crate::startup::load_audio::load_audio::<S>(None, startup_schedule);\n';assert s.count(needle)==1;add(rel,s.replace(needle,'    // The host runs the real sample-completion resource commands at\n    // PreStartup; load_audio does not register Schedule systems.\n'))
rel='tiny-glade/crates/tiny-glade/src/audio_host.rs';s=(ROOT/rel).read_text(encoding='utf8');a=s.index('fn receive_samples(world: &mut World) {');b=s.index('\nfn photo_audio(',a);replacement='''fn receive_samples(
    samples: Option<Res<AudioSamplesLoadedPromise<()>>>,
    mut commands: Commands,
) {
    if let Some(samples) = samples {
        country_core::startup::load_audio::load_audio::<NativeEvent>(&samples, &mut commands);
    }
}''';add(rel,s[:a]+replacement+s[b:])
(OUT/'load-audio-candidate.patch').write_text(''.join(patch),encoding='utf8');(OUT/'load-audio-candidate-manifest.json').write_text(json.dumps({'production_modified':False,'files':rows,'patch_sha256':hashlib.sha256((OUT/'load-audio-candidate.patch').read_bytes()).hexdigest()},indent=2),encoding='utf8');print(json.dumps(rows,indent=2))
