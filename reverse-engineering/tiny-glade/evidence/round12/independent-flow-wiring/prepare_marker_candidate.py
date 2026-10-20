from pathlib import Path
import hashlib,json,difflib,datetime
root=Path('D:/game/tiny-glade');out=Path(__file__).parent/'loading-marker-candidate'
rel='crates/country-core/src/systems/save/load_game.rs';old=(root/rel).read_bytes();src=old
start=src.index(b'pub fn write_loading_tmp(');end=src.index(b'\n}',start)+2
nl=b'\r\n' if b'\r\n' in src[start:end] else b'\n'
body='''pub fn write_loading_tmp(session: &SessionId) {
    // B14146d750..7b0 sets write/create/truncate, then drops both success
    // handle and every error. It does not write payload bytes or warn.
    let _ = std::fs::File::create(session.save_folder_path().join("loading.tmp"));
}'''.encode().replace(b'\n',nl)
src=src[:start]+body+src[end:]
old_doc='''/// 写 `save_folder/loading.tmp`（O：load_game 0x14146d030 后置写标记；@0x142c6416e）。
/// 文件内容为读档进度标记；失败仅告警（宿主可据此判定正在读档）。'''.encode().replace(b'\n',nl)
assert old_doc in src
src=src.replace(old_doc,b'/// Original state0 creates/truncates an empty loading.tmp marker; errors are dropped.',1)
needle=b'        LoadGameState::Loading => {';pos=src.index(needle,src.index(b'pub fn try_load_game_system'))
endline=src.index(b'\n',pos);nl=b'\r\n' if src[endline-1:endline]==b'\r' else b'\n'
addition=('''
            // Original state0 creates the marker before parsing/applying the
            // requested source. Failure to create the marker is nonfatal.
            write_loading_tmp(&world.resource::<crate::resources::session::ActiveSession>().id);''').encode().replace(b'\n',nl)
src=src[:pos]+src[pos:].replace(needle,needle+addition,1)
p=out/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(src)
diff=''.join(difflib.unified_diff(old.decode().splitlines(True),src.decode().splitlines(True),'a/'+rel,'b/'+rel))
(out.parent/'loading-marker-candidate.patch').write_text(diff,encoding='utf-8',newline='')
(out.parent/'loading-marker-candidate-manifest.json').write_text(json.dumps({'created_at':datetime.datetime.now().astimezone().isoformat(),'file':rel,'base_sha256':hashlib.sha256(old).hexdigest(),'candidate_sha256':hashlib.sha256(src).hexdigest(),'status':'Evidence only; original full load15561B + exact608B observer retained; no Cargo executed'},indent=2),encoding='utf-8')
print('base',hashlib.sha256(old).hexdigest(),'candidate',hashlib.sha256(src).hexdigest())
