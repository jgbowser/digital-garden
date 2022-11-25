use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use color_eyre::eyre::Result;

#[test]
/// make sure help runs. this indicates the binary works
fn test_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
fn test_write_help() -> Result<()> {
    let mut cmd = Command::cargo_bin("garden")?;
    let assert = cmd.arg("write").arg("--help").assert();
    assert.success().stderr("");
    Ok(())
}

#[test]
fn test_write() {
    let temp_dir = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("garden").unwrap();
    let fake_editor_path = std::env::current_dir()
        .expect("expect to be in a dir")
        .join("tests")
        .join("fake_editor.sh");
    if !fake_editor_path.exists() {
        panic!("fake editor shell script not found")
    }
    let assert = cmd
        .env("EDITOR", fake_editor_path.into_os_string())
        .env("GARDEN_PATH", temp_dir.path())
        .arg("write")
        .arg("-t")
        .arg("atitle")
        .write_stdin("N\n".as_bytes())
        .assert();

    assert.success();

    temp_dir.child("atitle.md")
    .assert(predicate::path::exists());
}
