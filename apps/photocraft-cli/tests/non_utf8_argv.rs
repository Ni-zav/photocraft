//! Invalid UTF-8 argv must fail normally, rather than trigger std::env::args()'s panic.

#[cfg(unix)]
#[test]
fn non_utf8_argument_reports_usage_error() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    use std::process::Command;

    let invalid_name = OsString::from_vec(b"image-\xff.png".to_vec());
    let output = Command::new(env!("CARGO_BIN_EXE_photocraft-cli"))
        .arg("info")
        .arg(invalid_name)
        .output()
        .expect("spawn photocraft-cli");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("valid UTF-8"), "stderr: {stderr}");
    assert!(!stderr.contains("panicked"), "stderr: {stderr}");
}
