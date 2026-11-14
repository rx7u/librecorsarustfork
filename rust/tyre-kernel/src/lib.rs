//! Small, allocation-free kernels used by the VF26 tyre model.
//!
//! ABI: callers write packed f64 inputs into exported linear memory and call
//! `magic_formula_batch(ptr, count)`. Each input is [x, B, C, E], each output
//! is one f64. Keeping the boundary batched avoids a JS↔WASM call per wheel.

#[no_mangle]
pub extern "C" fn magic_formula(x: f64, b: f64, c: f64, e: f64) -> f64 {
    let bx = b * x;
    (c * (bx - e * (bx - bx.atan()))).sin()
}

/// Evaluate N packed [x, B, C, E] tuples in-place as N results.
/// Returns 0 for a null pointer; otherwise returns the number of results.
#[no_mangle]
pub unsafe extern "C" fn magic_formula_batch(ptr: *mut f64, count: usize) -> usize {
    if ptr.is_null() { return 0; }
    let input = core::slice::from_raw_parts_mut(ptr, count.saturating_mul(4));
    // Read each tuple before overwriting its first slot with the result.
    for i in 0..count {
        let base = i * 4;
        let x = input[base];
        let b = input[base + 1];
        let c = input[base + 2];
        let e = input[base + 3];
        let bx = b * x;
        input[base] = (c * (bx - e * (bx - bx.atan()))).sin();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_slip_is_zero() {
        assert_eq!(magic_formula(0.0, 1.0, 1.6, 0.2), 0.0);
    }

    #[test]
    fn batch_matches_scalar() {
        let mut values = [0.2, 1.25, 1.6, 0.3, 0.8, 1.25, 1.6, 0.3];
        let expected = [
            magic_formula(values[0], values[1], values[2], values[3]),
            magic_formula(values[4], values[5], values[6], values[7]),
        ];
        unsafe { magic_formula_batch(values.as_mut_ptr(), 2); }
        assert!((values[0] - expected[0]).abs() < 1e-14);
        assert!((values[4] - expected[1]).abs() < 1e-14);
    }
}
