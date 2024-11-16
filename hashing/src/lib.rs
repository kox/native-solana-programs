use core::mem::MaybeUninit;

#[cfg(target_os = "solana")]
extern "C" {
    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut [u8;32]) -> u64;
}

// return a Sha256 has from the given data
#[inline(always)]
pub fn hashv(vals: &[&[u8]]) -> [u8;32] {
    #[cfg(target_os = "solana")]
    {
        let mut hash_result = MaybeUninit::<[u8;32]>::uninit();

        unsafe {
            sol_sha256(
                vals as *const _ as *const u8,
                vals.len() as u64,
                hash_result.as_mut_ptr(),
            );

            hash_result.assume_init()
        }
    }
}



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
