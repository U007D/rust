#![feature(inner_deref)]

// i32 does not implement `Deref`, and so `Result<_, i32>::Err(E)` should not have
// `as_ref_deref_ok()/as_mut_deref_mut_ok()` methods defined.
fn main() {
    let _ = Ok(41).as_ref_deref_ok();
//~^ ERROR no method named `as_ref_deref_ok` found
    let _ = Ok(41).as_mut_deref_mut_ok();
//~^ ERROR no method named `as_mut_deref_mut_ok` found
}
