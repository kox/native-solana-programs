#![cfg(feature = "test-sbf")]

use solana_program_test::*;
use hello_id_program;

#[tokio::test]
async fn test_hello() {
    let program_id = hello_id_program::id();
    let program_test = ProgramTest::new("hello_id_program", program_id, None);

    let mut ctx = program_test.start_with_context().await;

}