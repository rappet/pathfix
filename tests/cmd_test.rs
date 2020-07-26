extern crate assert_cmd;

use assert_cmd::Command;

#[test]
fn test_env_dedup() {
    let mut cmd = Command::cargo_bin("pathfix").unwrap();
    let assert = cmd
        .arg("-ed")
        .env("PATH", "/usr/bin:/usr/bin:/bin")
        .assert();
    assert
        .success()
        .stdout("/usr/bin:/bin\n");
}

#[test]
fn test_lines() {
    let mut cmd = Command::cargo_bin("pathfix").unwrap();
    let assert = cmd
        .arg("-el")
        .env("PATH", "/usr/bin:/usr/bin:/bin")
        .assert();
    assert
        .success()
        .stdout("/usr/bin\n/usr/bin\n/bin\n");
}

/// This tests the fallback path string if an error occurs.
/// Not giving any cli arguments is an error.
#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin("pathfix").unwrap();
    let assert = cmd
        .env("PATH", "/usr/bin:/usr/bin:/bin")
        .assert();
    assert
        .failure()
        .stdout("/usr/sbin:/usr/bin:/sbin:/bin\n");
}

