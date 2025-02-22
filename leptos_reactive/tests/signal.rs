#[cfg(not(feature = "stable"))]
use leptos_reactive::{create_scope, create_signal};

#[cfg(not(feature = "stable"))]
#[test]
fn basic_signal() {
    create_scope(|cx| {
        let (a, set_a) = create_signal(cx, 0);
        assert_eq!(a(), 0);
        set_a(5);
        assert_eq!(a(), 5);
    })
    .dispose()
}

#[cfg(not(feature = "stable"))]
#[test]
fn derived_signals() {
    create_scope(|cx| {
        let (a, set_a) = create_signal(cx, 0);
        let (b, set_b) = create_signal(cx, 0);
        let c = move || a() + b();
        assert_eq!(c(), 0);
        set_a(5);
        assert_eq!(c(), 5);
        set_b(1);
        assert_eq!(c(), 6);
    })
    .dispose()
}
