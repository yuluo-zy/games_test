from pathlib import Path
import hashlib,json,difflib

ROOT=Path(__file__).resolve().parent
REPO=ROOT.parents[4]
changes={}
def read(rel):
    p=REPO/rel
    text=p.read_text(encoding='utf-8')
    changes[rel]={'base_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'base':text}
    return text
def save(rel,text):
    p=ROOT/'proposed'/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(text,encoding='utf-8')
    changes[rel]['proposed_sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
    changes[rel]['proposed']=text

rel='tiny-glade/crates/country-core/src/resources/audio_resources.rs'
s=read(rel)
a=s.index('/// `AudioCore::instantiate_loop` descriptor')
b=s.index('/// One-shot completion',a)
s=s[:a]+'''/// Original borrowed descriptor is {name, extra_instance_count, stop_delay_ms}.
/// Native140a59243 increments count before collecting owned instances.
#[derive(Clone, Debug)]
pub struct AudioLoopDesc {
    pub name: String,
    /// Additional ring instances; native total is count.wrapping_add(1).
    pub count: usize,
    pub threshold_ms: u32,
}
impl AudioLoopDesc {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), count: 3, threshold_ms: 50 }
    }
}

'''+s[b:]
a=s.index('    /// `AudioCore::instantiate_loop` (0x140a59220):')
b=s.index('    /// `AudioCoreInner::set_parameter`',a)
s=s[:a]+'''    /// Original140a59220 collects fallible creation, then probes the first real
    /// instance's "loop" parameter. Prefix instances release on failure.
    pub fn try_instantiate_loop(&self, desc: &AudioLoopDesc) -> anyhow::Result<AudioLoop<AudioInstance<S::Event>>> {
        let entries: Vec<_> = (0..desc.count.wrapping_add(1))
            .map(|_| {
                let instance = self.instantiate_audio(&desc.name);
                if let AudioInstance::Err(error) = &instance {
                    return Err(anyhow::anyhow!("{error}"));
                }
                Ok(instance)
            }).collect::<anyhow::Result<_>>()?;
        let first = entries.first().expect("original AudioLoop first instance bounds check");
        let uses_loop_parameter = match first.has_parameter("loop") {
            Ok(found) => found,
            Err(error) => {
                log::error!("Couldn't query audio loop parameter for {}: {error:?}", desc.name);
                false
            }
        };
        Ok(AudioLoop::new(entries, desc.threshold_ms, uses_loop_parameter, false))
    }

    /// Startup callers unwrap the original Result. This adapter keeps their
    /// existing return type while retaining creation and prefix-drop failures.
    pub fn instantiate_loop(&self, desc: &AudioLoopDesc) -> AudioLoop<AudioInstance<S::Event>> {
        self.try_instantiate_loop(desc).expect("instantiate original audio loop")
    }

    /// A player owns the same fully initialized ring; no timing/flags are lost.
    pub fn instantiate_loop_player(&self, desc: &AudioLoopDesc) -> AudioLoopPlayer<AudioInstance<S::Event>> {
        AudioLoopPlayer::from_audio(self.instantiate_loop(desc))
    }

'''+s[b:]
s=s.replace('fn audio_loop_desc_native_default_shape_is_count_3_max_50()', 'fn audio_loop_desc_native_default_shape_is_extra_count_3_delay_50()').replace('assert_eq!(desc.max, 50);','assert_eq!(desc.threshold_ms, 50);')
needle='    #[test]\n    fn audio_loop_desc_native_default_shape_is_extra_count_3_delay_50()'
idx=s.index(needle)
s=s[:idx]+'''    #[test]
    fn native_disabled_loop_constructor_corpus_retains_all_instances_and_delay() {
        let core = AudioCore::<FakeStudio> { studio: None, banks: HashMap::new() };
        // Actual original constructor+iterator outputs, native-loop-init-fixtures.json.
        for (descriptor_count, actual_count) in [(0,1),(1,2),(3,4),(7,8)] {
            let mut desc = AudioLoopDesc::new("event:/SFX/Player/Walking");
            desc.count = descriptor_count;
            let ring = core.try_instantiate_loop(&desc).unwrap();
            assert_eq!(ring.entries.len(), actual_count);
            assert_eq!(ring.threshold_ms, 50);
            assert!(!ring.uses_loop_parameter);
            assert!(!ring.stops_explicitly);
        }
    }
    #[test]
    fn original_parameter_probe_survives_player_construction() {
        let core = AudioCore { studio: Some(FakeStudio), banks: HashMap::new() };
        let player = core.instantiate_loop_player(&AudioLoopDesc::new("event:/x"));
        assert_eq!(player.audio.entries.len(), 4);
        assert_eq!(player.audio.threshold_ms, 50);
        assert!(player.audio.uses_loop_parameter);
        assert!(!player.audio.stops_explicitly);
        assert!(!player.should_play);
    }

'''+s[idx:]
s += '\n' + (ROOT/'canonical-init.tests.rs').read_text(encoding='utf-8')
save(rel,s)

rel='tiny-glade/crates/country-core/src/utils/audio_utils.rs'
s=read(rel)
at=s.index('    /// 调用方必须维持非空实例队列')
s=s[:at]+'''    /// Original noloop adaptor sets AudioLoop+36. Keep native parameter
    /// detection; this additionally requests explicit stop when a loop exists.
    pub fn noloop(mut self) -> Self {
        self.stops_explicitly = true;
        self
    }

'''+s[at:]
save(rel,s)

rel='tiny-glade/crates/country-core/src/resources/audio_instance.rs'
s=read(rel)
a=s.index('/// 循环播放器：一组实例')
b=s.index('\n#[cfg(test)]',a)
s=s[:a]+'''/// Original player owns one complete AudioLoop plus its should-play byte.
/// Ring timing, parameter detection, current index and entries have one owner.
#[derive(Clone, Debug)]
pub struct AudioLoopPlayer<I> {
    pub audio: crate::utils::audio_utils::AudioLoop<I>,
    pub should_play: bool,
}
impl<I> AudioLoopPlayer<I> {
    pub fn from_audio(audio: crate::utils::audio_utils::AudioLoop<I>) -> Self {
        Self { audio, should_play: false }
    }
}
impl<I> AudioLoopPlayer<I>
where I: LoopInstance + crate::utils::audio_utils::LoopEntry<Error=<I as LoopInstance>::Error> {
    /// Explicit low-level host adapter. Game startup uses from_audio on the
    /// fully initialized canonical producer, rather than discarding its state.
    pub fn new(instances: Vec<I>) -> Self {
        Self::from_audio(crate::utils::audio_utils::AudioLoop::new(instances,0,false,false))
    }
    pub fn current(&self) -> &I { self.audio.current() }
    pub fn request_play(&mut self, position:[f32;3], volume:f32, editing_speed:f32) -> Result<(), <I as LoopInstance>::Error> {
        self.should_play = true;
        let instance = self.audio.current_mut();
        LoopInstance::set_position(instance,position);
        LoopInstance::set_parameter(instance,EDITING_SPEED_PARAMETER,editing_speed);
        if LoopInstance::is_active(instance) { LoopInstance::set_volume(instance,volume)?; }
        Ok(())
    }
    fn frame_end_with_stop(&mut self, stop: impl FnOnce(&mut crate::utils::audio_utils::AudioLoop<I>) -> Result<(),<I as LoopInstance>::Error>) -> Result<(),<I as LoopInstance>::Error> {
        let play = std::mem::take(&mut self.should_play);
        if play { self.audio.start_if_not_playing() } else { stop(&mut self.audio) }
    }
    pub fn frame_end(&mut self) -> Result<(),<I as LoopInstance>::Error> {
        self.frame_end_with_stop(crate::utils::audio_utils::AudioLoop::stop_now)
    }
}
'''+s[b:]
# Existing focused tests are kept, with real ring ownership and delayed stop.
s=s.replace('player.instances[0]','player.audio.entries[0]').replace('player.current = 1;','player.audio.cursor = 1;')
at=s.index('    #[test]\n    fn start_is_skipped_for_playing')
s=s[:at]+'''    impl crate::utils::audio_utils::LoopEntry for Fake {
        type Error = &'static str;
        fn is_active(&self) -> bool { LoopInstance::is_active(self) }
        fn set_parameter(&mut self,name:&str,value:f32) { LoopInstance::set_parameter(self,name,value); }
        fn start(&mut self) -> Result<(),Self::Error> { LoopInstance::start(self) }
        fn stop(&mut self,_:crate::utils::audio_utils::StopMode) -> Result<(),Self::Error> { LoopInstance::stop(self) }
    }

'''+s[at:]
a=s.index('    #[test]\n    fn frame_end_starts_when_should_play_and_stops_otherwise()')
b=s.index('    #[test]',a+16)
s=s[:a]+'''    #[test]
    fn player_retains_native_delayed_stop_and_rotation_from_its_ring() {
        use core::time::Duration;
        let audio = crate::utils::audio_utils::AudioLoop::new(vec![Fake {active:true,..Default::default()},Fake::default()],50,false,false);
        let mut player = AudioLoopPlayer::from_audio(audio);
        player.request_play([0.;3],1.,1.).unwrap();
        player.frame_end_with_stop(|ring|ring.stop(Duration::ZERO)).unwrap();
        assert!(player.audio.started);
        assert_eq!(player.audio.entries[0].calls,["position","editing_speed=1","volume=1","start"]);
        player.frame_end_with_stop(|ring|ring.stop(Duration::ZERO)).unwrap();
        player.frame_end_with_stop(|ring|ring.stop(Duration::from_millis(50))).unwrap();
        assert!(player.audio.started);
        player.frame_end_with_stop(|ring|ring.stop(Duration::from_millis(51))).unwrap();
        assert!(!player.audio.started);
        assert_eq!(player.audio.cursor,1);
        assert_eq!(player.audio.entries[0].calls.last().unwrap(),"stop");
        assert!(!player.should_play);
    }

'''+s[b:]
save(rel,s)

rel='tiny-glade/crates/country-core/src/systems/audio/wall_audio_utils.rs'
s=read(rel)
s=s.replace('pub fn new_player<I: crate::resources::audio_instance::LoopInstance>(instances: Vec<I>) -> AudioLoopPlayer<I> {','pub fn new_player<I>(instances: Vec<I>) -> AudioLoopPlayer<I>\nwhere I: crate::resources::audio_instance::LoopInstance + crate::utils::audio_utils::LoopEntry<Error=<I as crate::resources::audio_instance::LoopInstance>::Error> {')
save(rel,s)
for rel in ['tiny-glade/crates/country-core/src/systems/audio/building_audio.rs','tiny-glade/crates/country-core/src/systems/audio/wall_audio_fmod.rs','tiny-glade/crates/country-core/src/systems/audio/playermade_stairs_audio.rs']:
    s=read(rel)
    s=s.replace('let current = loop_player.current;\n                let _ = loop_player.instances[current]','let _ = loop_player.audio.current_mut()')
    s=s.replace('let current = state.loops[loop_index].current;\n                let _ = state.loops[loop_index].instances[current]','let _ = state.loops[loop_index].audio.current_mut()')
    s=s.replace('let current = state.loops[loop_index].current;\n            let _ = state.loops[loop_index].instances[current]','let _ = state.loops[loop_index].audio.current_mut()')
    save(rel,s)

rel='tiny-glade/crates/country-core/src/systems/audio/path_audio.rs'
s=read(rel)
old='''loops: PATH_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),'''
assert old in s
s=s.replace(old,'''loops: std::array::from_fn(|index| {
                // Native1409d0987/a1f/ab7/b64/c05 set each returned ring+36.
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(PATH_LOOP_EVENTS[index])).noloop()
            }),''')
save(rel,s)

rel='tiny-glade/crates/country-core/src/systems/audio/terrain_audio.rs'
s=read(rel).replace('AudioLoopDesc::new(name).count,','AudioLoopDesc::new(name).count.wrapping_add(1),')
save(rel,s)

rel='tiny-glade/crates/country-core/src/systems/audio/sheep_audio.rs'
s=read(rel)
a=s.index('/// Kind 4 →')
b=s.index('pub const SHEEP_PITCH_PARAM:',a)
s=s[:a]+'''/// Original inlined SheepAudioState constructors140b6a92c/1414d9b82 use
/// event name142ac2338 (20 bytes), separate from startup sheep's Baah event.
pub const KIND4_LOOP_EVENT: &str = "event:/SFX/Sheep/Pet";
/// Sheep reaction pitch parameter.
'''+s[b:]
save(rel,s)

rel='tiny-glade/crates/country-core/src/systems/audio/terrain_stairs_audio.rs'
s=read(rel)
needle='pub const STAIR_DRAW_ONE_SHOT: &str = "event:/SFX/StairDraw_OneShot";'
assert needle in s
s=s.replace(needle,needle+'\n/// Native1417e3389 initializes the persistent ring with this distinct event.\npub const STAIR_DRAW_LOOP_EVENT: &str = "event:/SFX/StairDraw_LoopTrem";')
needle='AudioLoopDesc::new(STAIR_DRAW_ONE_SHOT)'
assert s.count(needle)==1
s=s.replace(needle,'AudioLoopDesc::new(STAIR_DRAW_LOOP_EVENT)')
save(rel,s)

rel='tiny-glade/crates/country-core/src/systems/audio/decorator_audio.rs'
s=read(rel)
needle='    "event:/SFX/Cosmetics/MoveGround",\n    "event:/SFX/Lantern/Amb",'
assert needle in s
s=s.replace(needle,'    "event:/SFX/Cosmetics/MoveGround",\n    "event:/SFX/Cosmetics/MoveBuilding",')
save(rel,s)

# Integration dependency: first apply the full203-case footsteps candidate.
rel='tiny-glade/crates/country-core/src/systems/audio/firstperson_steps.rs'
foot=ROOT.parent/'firstperson-full/proposed'/rel
s=foot.read_text(encoding='utf-8')
changes[rel]={'base_sha256':hashlib.sha256(foot.read_bytes()).hexdigest(),'base':s,'dependency':'firstperson-full proposed file must be integrated first'}
a=s.index('        // Both native init callers')
b=s.index('\n    }\n}',a)
s=s[:a]+'''        Self {
            walking_loop: core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(WALK_LOOP_EVENT)),
        }'''+s[b:]
save(rel,s)

manifest=[];patches=[]
for rel,info in changes.items():
    manifest.append({'path':rel,**{k:v for k,v in info.items() if k not in ['base','proposed']}})
    patches.append(''.join(difflib.unified_diff(info['base'].splitlines(True),info['proposed'].splitlines(True),fromfile='a/'+rel,tofile='b/'+rel)))
(ROOT/'candidate.patch').write_text(''.join(patches),encoding='utf-8')
(ROOT/'candidate-manifest.json').write_text(json.dumps({'candidate_only':True,'files':manifest,'validation':'not yet rustfmt or compiled; no Cargo; original initialization oracle reused separately'},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'candidate_files':len(manifest),'production_modified':False}))
