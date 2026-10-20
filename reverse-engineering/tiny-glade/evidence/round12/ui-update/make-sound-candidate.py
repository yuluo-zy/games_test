from pathlib import Path
import difflib,json,hashlib,re,subprocess
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round12/ui-update'
p=root/'tiny-glade/crates/country-core/src/systems/ui/resources/widget_params.rs';src=p.read_text(encoding='utf8');a=src.index('/// UI 声音播放事件');z=src.index('/// UI 导航状态',a)
replacement='''/// Original PlayUiSound business payload is 24B. UiSounds stores four such
/// values. Native Events<EventInstance<PlayUiSound>> appends its own 8B EventId;
/// that counter is not part of the game event and must not be stored here.
#[repr(C)]
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayUiSound {
    pub kind: u32,
    /// Native +4..24, including nested enum tags at +4. Never drop this word.
    pub _payload: [u8; 20],
}
impl PlayUiSound {
    pub fn from_native_slot(slot: [u8; 24]) -> Self {
        Self { kind: u32::from_le_bytes(slot[..4].try_into().unwrap()),
            _payload: slot[4..].try_into().unwrap() }
    }
    pub fn native_slot(self) -> [u8; 24] {
        let mut slot = [0; 24];
        slot[..4].copy_from_slice(&self.kind.to_le_bytes());
        slot[4..].copy_from_slice(&self._payload);
        slot
    }
    pub fn variant4() -> Self { Self { kind: 4, ..Default::default() } }
}

'''
changes={p:src[:a]+replacement+src[z:]};inventory=[]
paths=subprocess.check_output(['rg','-l','PlayUiSound','tiny-glade/crates','-g','*.rs'],cwd=root,text=True).splitlines()
for rel in paths:
 path=root/rel;s=path.read_text(encoding='utf8');n=changes.get(path,s)
 if path!=p:
  n=re.sub(r'PlayUiSound::variant4\([^)]*\)', 'PlayUiSound::variant4()',n)
  def fix(m):
   t=m.group(0)
   t=re.sub(r'(_payload:\s*\[0;\s*)(?:0x10|16)(\])',r'\g<1>20\2',t)
   t=re.sub(r'\n\s*id:\s*0,','',t)
   return t
  n=re.sub(r'PlayUiSound\s*\{[^{}]*\}',fix,n)
 readers=[i+1 for i,l in enumerate(s.splitlines()) if 'EventReader<PlayUiSound>' in l]
 inventory.append(dict(path=rel,sha256=hashlib.sha256(s.encode()).hexdigest(),writer_count=s.count('EventWriter<PlayUiSound>'),reader_lines=readers,literals=s.count('PlayUiSound {'),changed=n!=s))
 if n!=s:changes[path]=n
patch=''
for p,n in changes.items():
 old=p.read_text(encoding='utf8');rel=p.relative_to(root).as_posix();patch+=''.join(difflib.unified_diff(old.splitlines(True),n.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
(out/'play-ui-sound.patch').write_text(patch,encoding='utf8');(out/'sound-type-fragment.rs').write_text(replacement,encoding='utf8');(out/'sound-writer-migration.json').write_text(json.dumps(inventory,indent=2),encoding='utf8')
print('changes',len(changes),'references',len(inventory),'readers',sum(len(v['reader_lines'])for v in inventory))
for x in inventory:
 if x['changed']:print(x['path'],x['literals'])
