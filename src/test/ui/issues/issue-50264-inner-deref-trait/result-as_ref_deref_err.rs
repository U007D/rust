#![feature(inner_deref)]

// i32 does not implement `Deref`, and so `Result<_, i32>::Err(E)` should not
// have `as_ref_deref()/as_mut_deref_mut()` methods defined.
fn main() {
    let _ = Err(41).as_ref_deref_err();
//~^ ERROR no method named `as_ref_deref_err` found
    let _ = Err(41).as_mut_deref_mut_err();
//~^ ERROR no method named `as_mut_deref_mut_err` found
}
