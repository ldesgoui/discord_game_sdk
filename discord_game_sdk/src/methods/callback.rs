use crate::{utils, Discord};
use std::ffi::c_void;

impl<'d, E> Discord<'d, E> {
    pub(crate) fn one_param<A>(
        &self,
        callback: impl 'd + FnOnce(&Discord<'d, E>, A),
    ) -> (*mut c_void, Option<unsafe extern "C" fn(*mut c_void, A)>) {
        extern "C" fn one_param_from_c<F: FnOnce(A), A>(ptr: *mut c_void, a: A) {
            let _guard = utils::prevent_unwind();

            // SAFETY:
            // lifetime of F was ellided when it was turned into a raw pointer
            // in all method calls, F is bound to 'd, which is the lifetime of the Discord instance
            // this is a bit risky, but it seems like the SDK will send `Err(TransactionAborted)` to
            // all waiting callbacks if we destroy the instance, we're relying on this behavior
            let callback = unsafe { Box::from_raw(ptr as *mut F) };
            callback(a)
        }

        fn one_param_align_types<F: FnOnce(A), A>(
            callback: F,
        ) -> (*mut c_void, Option<unsafe extern "C" fn(*mut c_void, A)>) {
            (
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(one_param_from_c::<F, A>),
            )
        }

        let dref = self.ref_copy();
        one_param_align_types(move |a| callback(&*dref, a))
    }

    pub(crate) fn two_params<A, B>(
        &self,
        callback: impl 'd + FnOnce(&Discord<'d, E>, A, B),
    ) -> (*mut c_void, Option<unsafe extern "C" fn(*mut c_void, A, B)>) {
        extern "C" fn two_params_from_c<F: FnOnce(A, B), A, B>(ptr: *mut c_void, a: A, b: B) {
            let _guard = utils::prevent_unwind();

            // SAFETY: see `one_param`
            let callback = unsafe { Box::from_raw(ptr as *mut F) };
            callback(a, b)
        }

        fn two_params_align_types<F: FnOnce(A, B), A, B>(
            callback: F,
        ) -> (*mut c_void, Option<unsafe extern "C" fn(*mut c_void, A, B)>) {
            (
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(two_params_from_c::<F, A, B>),
            )
        }

        let dref = self.ref_copy();
        two_params_align_types(move |a, b| callback(&*dref, a, b))
    }

    pub(crate) fn three_params<A, B, C>(
        &self,
        callback: impl 'd + FnOnce(&Discord<'d, E>, A, B, C),
    ) -> (
        *mut c_void,
        Option<unsafe extern "C" fn(*mut c_void, A, B, C)>,
    ) {
        extern "C" fn three_params_from_c<F: FnOnce(A, B, C), A, B, C>(
            ptr: *mut c_void,
            a: A,
            b: B,
            c: C,
        ) {
            let _guard = utils::prevent_unwind();

            // SAFETY: see `one_param`
            let callback = unsafe { Box::from_raw(ptr as *mut F) };
            callback(a, b, c)
        }

        fn three_params_align_types<F: FnOnce(A, B, C), A, B, C>(
            callback: F,
        ) -> (
            *mut c_void,
            Option<unsafe extern "C" fn(*mut c_void, A, B, C)>,
        ) {
            (
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(three_params_from_c::<F, A, B, C>),
            )
        }

        let dref = self.ref_copy();
        three_params_align_types(move |a, b, c| callback(&*dref, a, b, c))
    }
}
