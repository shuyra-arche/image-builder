use super::*;

#[test]
fn replacing_an_output_symlink_does_not_change_its_lock_identity() {
    let root = std::env::temp_dir().join(format!("vanllia-output-lock-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let target = root.join("old-image");
    let output = root.join("initrd.img");
    std::fs::write(&target, b"old").unwrap();
    std::os::unix::fs::symlink(&target, &output).unwrap();
    let held = leases(&[&output]).unwrap();
    let prepared = root.join("prepared");
    std::fs::write(&prepared, b"complete").unwrap();
    std::fs::rename(prepared, &output).unwrap();
    assert!(
        leases(&[&output]).is_err(),
        "a still-owned output acquired a second lock after rename"
    );
    assert_eq!(std::fs::read(target).unwrap(), b"old");
    drop(held);
    std::fs::remove_dir_all(root).unwrap();
}
