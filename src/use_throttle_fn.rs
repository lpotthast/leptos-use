use crate::utils::{create_filter_wrapper, create_filter_wrapper_with_arg, throttle_filter};
use leptos::MaybeSignal;
use std::cell::RefCell;
use std::rc::Rc;

pub use crate::utils::ThrottleOptions;

/// Throttle execution of a function.
/// Especially useful for rate limiting execution of handlers on events like resize and scroll.
///
/// > Throttle is a spring that throws balls: after a ball flies out it needs some time to shrink back, so it cannot throw any more balls unless it's ready.
///
/// ## Demo
///
/// TODO
///
/// ## Usage
///
/// ```
/// use leptos::*;
/// use leptos_use::use_throttle_fn;
///
/// #[component]
/// fn Demo(cx: Scope) -> impl IntoView {
///     let mut throttled_fn = use_throttle_fn(
///         || {
///             // do something, it will be called at most 1 time per second
///         },
///         1000.0,
///     );
///     view! { cx,
///         <button on:click=move |_| { throttled_fn(); }>
///             "Smash me!"
///         </button>
///     }
/// }
/// ```
///
/// You can provide options when you use [`use_throttle_fn_with_options`].
///
/// ```
/// # use leptos::*;
/// # use leptos_use::{ThrottleOptions, use_throttle_fn_with_options};
///
/// # #[component]
/// # fn Demo(cx: Scope) -> impl IntoView {
///  let throttled_fn = use_throttle_fn_with_options(
///     || {
///         // do something, it will be called at most 1 time per second
///     },
///     1000.0,
///     ThrottleOptions {
///         leading: true,
///         trailing: true,
///     }
/// );
/// #    view! { cx, }
/// # }
/// ```
///
/// If your function that you want to throttle takes an argument there are also the versions
/// [`use_throttle_fn_with_args`] and [`use_throttle_fn_with_args_and_options`].
///
/// ## Recommended Reading
///
/// - [**Debounce vs Throttle**: Definitive Visual Guide](https://redd.one/blog/debounce-vs-throttle)
pub fn use_throttle_fn<F, R>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
) -> impl FnMut() -> Rc<RefCell<Option<R>>>
where
    F: FnMut() -> R + Clone + 'static,
    R: 'static,
{
    use_throttle_fn_with_options(func, ms, Default::default())
}

/// Version of [`use_throttle_fn`] with throttle options. See the docs for [`use_throttle_fn`] for how to use.
pub fn use_throttle_fn_with_options<F, R>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
    options: ThrottleOptions,
) -> impl FnMut() -> Rc<RefCell<Option<R>>>
where
    F: FnMut() -> R + Clone + 'static,
    R: 'static,
{
    create_filter_wrapper(throttle_filter(ms, options), func)
}

/// Version of [`use_throttle_fn`] with an argument for the throttled function. See the docs for [`use_throttle_fn`] for how to use.
pub fn use_throttle_fn_with_arg<F, Arg, R>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
) -> impl FnMut(Arg) -> Rc<RefCell<Option<R>>>
where
    F: FnMut(Arg) -> R + Clone + 'static,
    Arg: 'static,
    R: 'static,
{
    use_throttle_fn_with_arg_and_options(func, ms, Default::default())
}

/// Version of [`use_throttle_fn_with_arg`] with throttle options. See the docs for [`use_throttle_fn`] for how to use.
pub fn use_throttle_fn_with_arg_and_options<F, Arg, R>(
    func: F,
    ms: impl Into<MaybeSignal<f64>>,
    options: ThrottleOptions,
) -> impl FnMut(Arg) -> Rc<RefCell<Option<R>>>
where
    F: FnMut(Arg) -> R + Clone + 'static,
    Arg: 'static,
    R: 'static,
{
    create_filter_wrapper_with_arg(throttle_filter(ms, options), func)
}