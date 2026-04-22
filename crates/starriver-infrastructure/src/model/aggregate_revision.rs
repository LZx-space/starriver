pub struct Revision<T: Clone> {
    /// 变更前（原始、旧版本）
    original: T,
    /// 变更后（当前、新版本）
    modified: T,
}

impl<T> Revision<T>
where
    T: Clone,
{
    pub fn new(original: T, modified: T) -> Self {
        Self { original, modified }
    }

    pub fn dissolve(self) -> (T, T) {
        (self.original, self.modified)
    }
}
