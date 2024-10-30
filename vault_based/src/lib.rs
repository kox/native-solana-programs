pub const ID: [u8; 32] = [
    0x7b, 0x07, 0x5a, 0x4f, 0xca, 0x15, 0x61, 0x6e, 
    0xbe, 0x53, 0xc1, 0xa8, 0x43, 0x6f, 0x42, 0x89, 
    0x2b, 0x02, 0x1a, 0xb6, 0x62, 0x5a, 0x2a, 0x02, 
    0x2a, 0x68, 0x9a, 0xef, 0xbd, 0xed, 0x26, 0xef
];

#[allow(unused)]
extern "C" {
    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut [u8;32]) -> u64;
    fn sol_log_(input: *const u8, len: u64) -> u64;
}

#[no_mangle]
/// # Safety
/// Where we're going, we don't need memory safety.
pub unsafe extern "C" fn entrypoint(input: *mut u8) {
    use core::mem::MaybeUninit;

    if *(input as *const u64) != 2 {
        sol_log_("Invalid number of accounts".as_ptr(), 26);
        core::arch::asm!("lddw r0, 1");
        return;
    }

    // Ensure signer had 0 bytes data length
    if *(input.add(0x0058) as *const u64) != 0 {
        sol_log_("Invalid account length Signer".as_ptr(), 29);
        core::arch::asm!("lddw r0, 2");
        return;  
    };

    // Ensure signer is a mutable no-duplicate signer
    if *(input.add(0x0008) as *const u32) != 0x0101ff {
        sol_log_("Signer is not a mutable nodup signer".as_ptr(), 36);
        core::arch::asm!("lddw r0, 5");
        return;  
    }

    // // Ensure PDA has 0 bytes data length
    if *(input.add(0x28b8) as *const u64) != 0 {
        sol_log_("Invalid account length Vault".as_ptr(), 28);
        core::arch::asm!("lddw r0, 3");
        return;  
    }

    unsafe {
        // Get the signer key
        let signer: [u8; 32] = *(input.add(0x0010) as *const [u8; 32]);
    
        // Get the bump
        let bump = *input.add(0x50d8) as u8;

        let data = [
            signer.as_ref(),
            &[bump],
            ID.as_ref(),
            b"ProgramDerivedAddress",
        ];

        let mut pda = MaybeUninit::<[u8; 32]>::uninit();
        sol_sha256(
            &data as *const _ as *const u8,
            4,
            pda.as_mut_ptr(),
        );

        // Check PDA address
        if *(input.add(0x2870) as *const [u8; 32]) != *pda.as_ptr() {
            {
                sol_log_("Invalid PDA address".as_ptr(), 29);
                core::arch::asm!("lddw r0, 4");
            }
            return;  
        };
    }
    
    let lamports: u64 = unsafe { *(input.add(0x50d0) as *const u64) };

    // Deduct lamports from PDA
    *(input.add(0x28b0) as *mut u64) -= lamports;
    // Add lamports to Signer
    *(input.add(0x0050) as *mut u64) += lamports;
}
