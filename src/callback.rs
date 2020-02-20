use std::ffi::c_void;

pub(crate) fn one_param<F: FnOnce(A), A>(
    callback: F,
) -> (
    *mut std::ffi::c_void,
    Option<unsafe extern "C" fn(*mut c_void, A)>,
) {
    extern "C" fn one_param<F: FnOnce(A), A>(ptr: *mut c_void, a: A) {
        prevent_unwind!();

        // SAFETY:
        // lifetime of F was ellided when it was turned into a raw pointer
        // in all method calls, F is bound to 'd, which is the lifetime of the Discord instance
        // this is a bit risky, but it seems like the SDK will send `Err(TransactionAborted)` to
        // all waiting callbacks if we destroy the instance, we're relying on this behavior
        let callback = unsafe { Box::from_raw(ptr as *mut F) };
        callback(a)
    }

    (
        Box::into_raw(Box::new(callback)) as *mut _,
        Some(one_param::<F, A>),
    )
}

pub(crate) fn two_params<F: FnOnce(A, B), A, B>(
    callback: F,
) -> (
    *mut std::ffi::c_void,
    Option<unsafe extern "C" fn(*mut c_void, A, B)>,
) {
    extern "C" fn two_params<F: FnOnce(A, B), A, B>(ptr: *mut c_void, a: A, b: B) {
        prevent_unwind!();

        // SAFETY: see `one_param`
        let callback = unsafe { Box::from_raw(ptr as *mut F) };
        callback(a, b)
    }

    (
        Box::into_raw(Box::new(callback)) as *mut _,
        Some(two_params::<F, A, B>),
    )
}

pub(crate) fn three_params<F: FnOnce(A, B, C), A, B, C>(
    callback: F,
) -> (
    *mut std::ffi::c_void,
    Option<unsafe extern "C" fn(*mut c_void, A, B, C)>,
) {
    extern "C" fn three_params<F: FnOnce(A, B, C), A, B, C>(ptr: *mut c_void, a: A, b: B, c: C) {
        prevent_unwind!();

        // SAFETY: see `one_param`
        let callback = unsafe { Box::from_raw(ptr as *mut F) };
        callback(a, b, c)
    }

    (
        Box::into_raw(Box::new(callback)) as *mut _,
        Some(three_params::<F, A, B, C>),
    )
}
