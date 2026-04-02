$env:RUST_BACKTRACE = 1
cargo test --lib tests::test_basic_sovereign_operations -- --nocapture *>&1 | Out-File -FilePath test_result.log
Get-Content test_result.log
