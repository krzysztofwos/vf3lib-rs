use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data")
}

pub fn fixture_path(name: &str) -> String {
    let path = fixtures_dir().join(name);
    assert!(path.exists(), "missing bundled fixture: {}", path.display());
    path.to_string_lossy().into_owned()
}

pub fn fixture_pair(pattern: &str, target: &str) -> (String, String) {
    (fixture_path(pattern), fixture_path(target))
}
