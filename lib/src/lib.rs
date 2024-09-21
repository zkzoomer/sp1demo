use alloy_sol_types::sol;

sol! {
    struct PublicValuesStruct {
        uint32 x;
        uint32 y;
        uint32 z;
    }
}

/// `z = f(x, y) = x^2 + y^2`
pub fn f(x: u32, y: u32) -> u32 {
    x.saturating_pow(2).saturating_add(y.saturating_pow(2))
}
