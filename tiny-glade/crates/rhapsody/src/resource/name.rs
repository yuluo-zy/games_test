//! 资源语义名。
//!
//! 内部是驻留字符串 [`ustr::Ustr`]：同名文本全局只存一份，[`ResourceName`] 保持 `Copy`
//! 且比较是 64 位整数比较，适合作为 temporal 资源键、导出名与调试名的轻量载体。驻留表
//! 不回收，因此只用于数量有限的命名资源（图内资源名、temporal 名），不要拿它给每帧的
//! 临时对象起动态名字。

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceName(ustr::Ustr);

impl ResourceName {
    /// 按文本驻留得到语义名。
    pub fn new(name: &str) -> Self {
        Self(ustr::Ustr::from(name))
    }

    pub fn from_ustr(inner: ustr::Ustr) -> Self {
        Self(inner)
    }

    pub fn as_ustr(self) -> ustr::Ustr {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.0.as_str().is_empty()
    }
}

impl From<&str> for ResourceName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for ResourceName {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<ustr::Ustr> for ResourceName {
    fn from(value: ustr::Ustr) -> Self {
        Self(value)
    }
}

impl fmt::Display for ResourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Debug for ResourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourceName({:?})", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_text_is_interned_and_copyable() {
        let a = ResourceName::new("taa.history");
        let b = ResourceName::from("taa.history");
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(a.as_str(), "taa.history");
        assert_eq!(a.to_string(), "taa.history");
        assert!(!a.is_empty());
    }

    #[test]
    fn different_text_is_distinct() {
        assert_ne!(
            ResourceName::new("main.color"),
            ResourceName::new("main.depth")
        );
    }

    #[test]
    fn empty_name_reports_empty() {
        assert!(ResourceName::new("").is_empty());
    }

    #[test]
    fn ustr_round_trip_preserves_name() {
        let inner = ustr::Ustr::from("bindless.heap");
        let name = ResourceName::from_ustr(inner);
        assert_eq!(name.as_ustr(), inner);
    }
}
