#![feature(inner_deref)]

// i32 does not implement `Deref`, and so `Option<i32>::Some(T)` should not
// have `as_ref_deref()/as_mut_deref_mut()` methods defined.
fn main() {
    let _ = Some(41).as_ref_deref();
//~^ ERROR no method named `as_ref_deref` found
}
