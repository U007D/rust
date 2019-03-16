#![feature(inner_deref)]

// i32 does not implement `Deref`, and so neither `Result<i32, i32>::Ok(T)` nor
// `::Err(E)` should have `as_ref_deref()/as_mut_deref_mut()` methods defined.
fn main() {
    let _ = Ok(41).as_ref_deref();
//~^ ERROR no method named `as_ref_deref` found
    let _ = Ok(41).as_mut_deref_mut();
//~^ ERROR no method named `as_mut_deref_mut` found
    let _ = Err(41).as_ref_deref();
//~^ ERROR no method named `as_ref_deref` found
    let _ = Err(41).as_mut_deref_mut();
//~^ ERROR no method named `as_mut_deref_mut` found

}
