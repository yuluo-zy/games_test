from pathlib import Path
import difflib,hashlib,json
root=Path('D:/game');here=Path(__file__).parent
rel='tiny-glade/crates/country-core/src/resources/walls/cached_decorator_transforms.rs'
source=(root/rel).read_text(encoding='utf-8');text=source
a=text.index('/// B CachedDecoratorTransforms：')
b=text.index('impl CachedDecoratorTransforms {',a)
text=text[:a]+'''/// Current authoritative transform map. Original cache_decorator_transforms
/// borrows its previous-frame scratch as a Local<CachedDecoratorTransformStorage>;
/// reads after insert observe this current map in the same frame.
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct CachedDecoratorTransforms {
    pub storage: CachedDecoratorTransformStorage,
}

'''+text[b:]
text=text.replace('    /// 读取册（上一帧完成册）。\n    fn read(&self) -> &CachedDecoratorTransformStorage {\n        &self.buffers[self.current_write ^ 1]\n    }','    fn read(&self) -> &CachedDecoratorTransformStorage {\n        &self.storage\n    }')
text=text.replace('self.buffers[self.current_write].insert(id, transform);','self.storage.insert(id, transform);')
text=text.replace('''    /// 0x140a1aeb0（O）：双缓冲互换 + 清一册供本帧重写。
    pub fn swap_buffers_and_clear(&mut self) {
        self.current_write ^= 1;
        self.buffers[self.current_write].clear();
    }''','''    /// B140a1aeb0: swap with the actual system-local previous map, then clear
    /// this current destination. The caller can read previous during rebuilding.
    pub fn swap_buffers_and_clear(&mut self, previous: &mut CachedDecoratorTransformStorage) {
        std::mem::swap(&mut self.storage, previous);
        self.storage.clear();
    }''')
text=text.replace('''        // 读取册 = 上一帧完成册；本帧写入册尚未 swap，读取册为空 → try 版 None。
        assert!(cache.try_position(7).is_none());
        cache.swap_buffers_and_clear();
        // swap 后写入册成为读取册，可读到。''','''        // The authoritative current map is readable immediately after insert.''')
needle='''        assert_eq!(cache.transform(7), transform(1.0, 2.0, 3.0));
    }'''
text=text.replace(needle,'''        assert_eq!(cache.transform(7), transform(1.0, 2.0, 3.0));
        let mut previous = CachedDecoratorTransformStorage::default();
        cache.swap_buffers_and_clear(&mut previous);
        assert!(cache.try_position(7).is_none());
        assert_eq!(previous.get(7).copied(), Some(transform(1.0, 2.0, 3.0)));
        cache.insert(7, transform(4.0, 5.0, 6.0));
        assert_eq!(cache.position(7), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(previous.get(7).copied(), Some(transform(1.0, 2.0, 3.0)));
    }''',1)
changes=[(rel,source,text)]
for rel in ['tiny-glade/crates/systems/decorator/tests/core_paths_integration.rs','tiny-glade/crates/systems/decorator/tests/audio_integration.rs','tiny-glade/crates/systems/decorator/src/cull_oob_decorators.rs']:
    old=(root/rel).read_text(encoding='utf-8');new=old
    new=new.replace('    assert!(cache.try_transform(7).is_none(), "write buffer not readable");\n    cache.swap_buffers_and_clear();\n','')
    new=new.replace('        // insert 写当前册；读取册是另一册 → 交换后才可查询。\n        cache.swap_buffers_and_clear();\n','')
    new='\n'.join(line for line in new.split('\n') if 'transforms.swap_buffers_and_clear();' not in line)
    changes.append((rel,old,new))
patch=''
manifest=[]
for rel,old,new in changes:
    candidate=here/'core-candidates'/rel
    candidate.parent.mkdir(parents=True,exist_ok=True);candidate.write_text(new,encoding='utf-8')
    patch+=''.join(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
    manifest.append({'path':rel,'before_sha256':hashlib.sha256((root/rel).read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256(candidate.read_bytes()).hexdigest()})
(here/'cache-owner-and-fixtures.patch').write_text(patch,encoding='utf-8')
(here/'cache-owner-and-fixtures-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf-8')
