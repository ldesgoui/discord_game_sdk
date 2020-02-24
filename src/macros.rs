macro_rules! with_manager {
    ($with_manager: ident, $get_manager: ident, $type: ty) => {
        pub(crate) fn $with_manager<T>(&self, callback: impl FnOnce(&mut $type) -> T) -> T {
            self.with_core(|core| {
                let ptr = unsafe { core.$get_manager.unwrap()(core) };
                utils::with_tx(ptr, callback)
            })
        }
    };
}
