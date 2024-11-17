#[derive(Debug, PartialEq, Eq)]
pub enum CurveError {
    Overflow
}

// Static Invariant calculation
#[inline]
pub fn k_from_xy(x: u64, y: u64) -> Result<u128, CurveError> {
    assert_ne!(x, 0);
    assert_ne!(y, 0);
    Ok((x as u128).checked_mul(y as u128).ok_or(CurveError::Overflow)?)
}

// Get spot price for a token in its opposing token
#[inline]
pub fn spot_price_from_pair(x: u64, y: u64, precision: u32) -> Result<u64, CurveError> {
    assert_ne!(x, 0);
    assert_ne!(y, 0);
    Ok(
        u64::try_from(
            (x as u128)
            .checked_mul(precision as u128).ok_or(CurveError::Overflow)?
            .checked_div(y as u128).ok_or(CurveError::Overflow)?
            .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        ).map_err(|_| CurveError::Overflow)?
    )
}

// Get amount of X and Y to deposit from liquidity token amount
#[inline]
pub fn xy_deposit_amounts_from_l(x: u64, y: u64, l: u64, a: u64, precision: u32) -> Result<(u64, u64), CurveError> {
    let ratio = (l as u128)
        .checked_add(a as u128).ok_or(CurveError::Overflow)?
        .checked_mul(precision as u128).ok_or(CurveError::Overflow)?
        .checked_div(l as u128).ok_or(CurveError::Overflow)?;
    let deposit_x = (x as u128)
        .checked_mul(ratio).ok_or(CurveError::Overflow)?
        .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        .checked_sub(x as u128).ok_or(CurveError::Overflow)? as u64;
    let deposit_y = (y as u128)
        .checked_mul(ratio).ok_or(CurveError::Overflow)?
        .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        .checked_sub(y as u128).ok_or(CurveError::Overflow)? as u64;
    Ok((
        deposit_x,
        deposit_y
    ))
}

// Get amount of X and Y to withdraw from liquidity token amount
#[inline]
pub fn xy_withdraw_amounts_from_l(x: u64, y: u64, l: u64, a: u64, precision: u32) -> Result<(u64, u64), CurveError> {
    let ratio = ((l - a) as u128)
    .checked_mul(precision as u128).ok_or(CurveError::Overflow)?
    .checked_div(l as u128).ok_or(CurveError::Overflow)?;

    let withdraw_x = (x as u128)
        .checked_sub((x as u128)
            .checked_mul(ratio).ok_or(CurveError::Overflow)?
            .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        ).ok_or(CurveError::Overflow)? as u64;

    let withdraw_y = (y as u128)
        .checked_sub((y as u128)
            .checked_mul(ratio).ok_or(CurveError::Overflow)?
            .checked_div(precision as u128).ok_or(CurveError::Overflow)?
        ).ok_or(CurveError::Overflow)? as u64;

    Ok((
        withdraw_x, 
        withdraw_y
    ))
}

// Calculate new value of X after depositing Y
// When we swap amount A of Y for X, we must calculate the new balance of X from invariant K
// Y₂ = Y₁ + Amount
// X₂ = K / Y₂
#[inline]
pub fn x2_from_y_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    let k = k_from_xy(x, y)?;
    let x_new = (y as u128).checked_add(a as u128).ok_or(CurveError::Overflow)?;
    Ok(k.checked_div(x_new).ok_or(CurveError::Overflow)? as u64)
}

// Calculate new value of Y₂ after depositing X
// When we swap amount A of X for Y, we must calculate the new balance of Y from invariant K
// X₂ = X₁ + Amount
// Y₂ = K / X₂
#[inline]
pub fn y2_from_x_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    x2_from_y_swap_amount(y,x,a)
}

// Calculate the withdraw amount of X from swapping in Y
// ΔX = X₁ - X₂
#[inline]
pub fn delta_x_from_y_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    Ok(x.checked_sub(x2_from_y_swap_amount(x,y,a)?).ok_or(CurveError::Overflow)?)
}

// Calculate difference in Y from swapping in X
// ΔY = Y₁ - Y₂ 
#[inline]
pub fn delta_y_from_x_swap_amount(x: u64, y: u64, a: u64) -> Result<u64, CurveError> {
    delta_x_from_y_swap_amount(y,x,a)
}

// Calculate the withdraw amount of X from swapping in Y
// ΔX = X₁ - X₂
#[inline]
pub fn delta_x_from_y_swap_amount_with_fee(x: u64, y: u64, a: u64, fee: u16) -> Result<(u64, u64), CurveError> {
    let raw_amount = x.checked_sub(x2_from_y_swap_amount(x,y,a)?).ok_or(CurveError::Overflow)?;
    let amount = raw_amount.checked_mul((10_000 - fee).into()).ok_or(CurveError::Overflow)?.saturating_div(10_000);
    Ok((amount, raw_amount - amount))
}

// Calculate difference in Y from swapping in X
// ΔY = Y₁ - Y₂ 
#[inline]
pub fn delta_y_from_x_swap_amount_with_fee(x: u64, y: u64, a: u64, fee: u16) -> Result<(u64, u64), CurveError> {
    delta_x_from_y_swap_amount_with_fee(y,x,a, fee)
}

#[cfg(test)]
mod tests {
    use crate::delta_y_from_x_swap_amount_with_fee;
    #[test]
    fn swap() {
        let (amount_out, fee) = delta_y_from_x_swap_amount_with_fee(20, 30, 5, 0).unwrap();
        assert_eq!(amount_out, 6);
        assert_eq!(fee, 0);
        let (amount_out, fee) = delta_y_from_x_swap_amount_with_fee(25, 24, 5, 0).unwrap();
        assert_eq!(amount_out, 4);
        assert_eq!(fee, 0);
    }

    #[test]
    fn swap_with_fee() {
        let (amount_out, fee) = delta_y_from_x_swap_amount_with_fee(20, 30, 5, 100).unwrap();
        assert_eq!(amount_out, 5);
        assert_eq!(fee, 1);
    }
}