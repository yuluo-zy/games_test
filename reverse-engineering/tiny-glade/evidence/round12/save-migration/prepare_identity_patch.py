from pathlib import Path
import difflib, hashlib, json, subprocess

ROOT = Path('D:/game')
OUT = Path(__file__).resolve().parent
changes=[]
def edit(relative, transform):
    before=(ROOT/relative).read_text(encoding='utf-8')
    after=transform(before)
    target=OUT/'candidate'/relative
    target.parent.mkdir(parents=True,exist_ok=True)
    target.write_text(after,encoding='utf-8',newline='\n')
    changes.append((relative,before,after))

def identity(s):
    a=s.index('/// 解析失败回退（O：0x141673140）')
    b=s.index('/// 供外部使用的完整入口',a)
    s=s[:a]+'''/// B 141673140: hash the optional basename first, then hash the pair of
/// the previous result and file length, then the pair with modification time.
/// Metadata failures retain the already-computed prefix hash. This function
/// neither parses the Identity section nor uses the current clock.
pub fn derive_fallback_save_file_identity(path: &Path) -> SaveFileIdentity {
    let mut hash = crate::utils::calculate_hash(&path.file_name());
    if let Ok(metadata) = std::fs::metadata(path) {
        hash = crate::utils::calculate_hash(&(hash, &metadata.len()));
        if let Ok(modified) = metadata.modified() {
            hash = crate::utils::calculate_hash(&(hash, &modified));
        }
    }
    HistoryIdentity(hash)
}

'''+s[b:]
    a=s.index('/// 让 `SystemTime` 参与测试语义')
    b=s.index('#[cfg(test)]',a)
    s=s[:a]+s[b:]
    s=s.replace('    time::{SystemTime, UNIX_EPOCH},\n','')
    s=s.replace('//!   二次尝试、再由文件元数据派生（字段序 U，未决）。','//!   basename → file length → modification time 的逐次 tuple hash。')
    s=s.replace('//! - `derive_fallback_save_file_identity` 0x141673140：解析失败回退——取 SystemTime、','//! - `derive_fallback_save_file_identity` 0x141673140：解析失败回退——依次哈希')
    at=s.rfind('\n}')
    s=s[:at]+'''
    #[test]
    fn fallback_hashes_metadata_and_does_not_reparse_identity() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(br#"{\"Identity\":7}"#).unwrap();
        file.flush().unwrap();
        let metadata = std::fs::metadata(file.path()).unwrap();
        let basename = crate::utils::calculate_hash(&file.path().file_name());
        let sized = crate::utils::calculate_hash(&(basename, &metadata.len()));
        let expected = crate::utils::calculate_hash(&(sized, &metadata.modified().unwrap()));
        assert_eq!(derive_fallback_save_file_identity(file.path()), HistoryIdentity(expected));
        assert_eq!(parse_save_file_identity(file.path()), HistoryIdentity(7));
    }

    #[test]
    fn metadata_failure_returns_the_basename_hash() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("missing-history.json");
        assert_eq!(derive_fallback_save_file_identity(&path),
            HistoryIdentity(crate::utils::calculate_hash(&path.file_name())));
    }
'''+s[at:]
    return s

def transactions(s):
    needle='    pub fn ledger(&self) -> &ObservedHistoryLedger<M, S> {'
    assert needle in s
    return s.replace(needle,'''    /// Canonical native History+0x60. The old public ledger field name
    /// mutation_nonce refers to this same owner, not a separate RNG counter.
    pub fn identity(&self) -> super::HistoryIdentity {
        super::HistoryIdentity(self.ledger.mutation_nonce)
    }
    /// Load/move boundary for an already parsed Identity. Recording mutations
    /// continue to replace this same value through their existing RNG boundary.
    pub fn set_identity(&mut self, identity: super::HistoryIdentity) {
        self.ledger.mutation_nonce = identity.0;
    }

'''+needle,1)

edit('tiny-glade/crates/country-core/src/systems/save/save_file_identity.rs',identity)
edit('tiny-glade/crates/country-core/src/resources/history/transactions.rs',transactions)
def load_identity(s):
    old='''        // Native serde object @1415bd9f6..bda3d explicitly writes zero here.
        mutation_nonce: 0,'''
    new='''        // Parser writes parsed+0x48; load14146dd98..de95 moves it into
        // canonical History+0x60 after the serde object's temporary zero.
        mutation_nonce: parsed.identity.context("B parsed history identity missing")?.0,'''
    assert old in s
    return s.replace(old,new,1)
edit('tiny-glade/crates/country-core/src/systems/save/load_game.rs',load_identity)
patch=''.join(''.join(difflib.unified_diff(a.splitlines(True),b.splitlines(True),fromfile='a/'+p,tofile='b/'+p)) for p,a,b in changes)
(OUT/'identity-review.patch').write_text(patch,encoding='utf-8',newline='\n')
(OUT/'identity-source-baseline.json').write_text(json.dumps([
 {'path':p,'before_sha256':hashlib.sha256(a.encode()).hexdigest(),'candidate_sha256':hashlib.sha256(b.encode()).hexdigest()} for p,a,b in changes],indent=2),encoding='utf-8')
print(json.dumps({'files':len(changes),'patch':str(OUT/'identity-review.patch')}))
