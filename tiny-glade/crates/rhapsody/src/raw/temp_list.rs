//! 帧内临时对象的拥有容器；不重写通用 Vec 分配实现。
//! 原版类型（PDB：`rhapsody::raw::temp_list::TempList<T>::add`、`TempListInner<CString/Sampler>`），
//! 当前重建侧暂无调用点，仍按恢复类型保留。
#[derive(Default)]
pub struct TempList<T> {
    items: Vec<T>,
}
impl<T> TempList<T> {
    pub fn new() -> Self {
        Self { items: vec![] }
    }
    pub fn add(&mut self, item: T) -> &T {
        self.items.push(item);
        self.items.last().unwrap()
    }
    pub fn clear(&mut self) {
        self.items.clear()
    }
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }
}
