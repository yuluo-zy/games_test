//! 原定义列表保留重复项；文件键将每个字符串对的 DefaultHasher 结果异或。
use std::hash::{DefaultHasher, Hash, Hasher};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShaderDefines(pub Vec<(String, String)>);
impl ShaderDefines {
    /// 不排序、不去重；空列表为零，同一对出现两次会抵消。
    pub fn artifact_hash(&self) -> u64 {
        self.0.iter().fold(0, |hash, pair| {
            let mut hasher = DefaultHasher::new();
            pair.hash(&mut hasher);
            hash ^ hasher.finish()
        })
    }
}
