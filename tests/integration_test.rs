// Integration test for SigmaOS-backup-application
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use backup_application as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
