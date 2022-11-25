use predicates::str::contains;

mod helpers;
use helpers::*;

const DIR: &str = "tests/workspace.in";

#[test]
fn test_workspace_run_without_build() {
    let dir = setup(DIR, Project::Workspace);
    cargo_codspeed(&dir)
        .arg("run")
        .assert()
        .failure()
        .stderr(contains("Error No benchmarks found."));
    teardown(dir);
}

#[test]
fn test_workspace_build_without_package_spec() {
    let dir = setup(DIR, Project::Workspace);
    cargo_codspeed(&dir)
        .arg("build")
        .assert()
        .failure()
        .stderr(contains(
            "Error No package found. If working with a workspace please use the -p option to specify a member.",
        ));
    teardown(dir);
}

#[test]
fn test_workspace_build_subpackage_and_run_subpackage() {
    let dir = setup(DIR, Project::Workspace);
    cargo_codspeed(&dir)
        .arg("build")
        .args(["--package", "package-a"])
        .assert()
        .success()
        .stderr(contains("Finished built 1 benchmark suite(s)"));
    cargo_codspeed(&dir)
        .arg("run")
        .args(["--package", "package-a"])
        .assert()
        .success()
        .stderr(contains("Finished running 1 benchmark suite(s)"));
    teardown(dir);
}

#[test]
fn test_workspace_build_subpackage_and_run_other() {
    let dir = setup(DIR, Project::Workspace);
    cargo_codspeed(&dir)
        .arg("build")
        .args(["--package", "package-a"])
        .assert();
    cargo_codspeed(&dir)
        .arg("run")
        .args(["--package", "package-b"])
        .assert()
        .failure()
        .stderr(contains("Error No benchmarks found."));
    teardown(dir);
}

#[test]
fn test_workspace_build_both_and_run_submodule() {
    let dir = setup(DIR, Project::Workspace);
    cargo_codspeed(&dir)
        .arg("build")
        .args(["--package", "package-a"])
        .assert();

    cargo_codspeed(&dir)
        .arg("build")
        .args(["--package", "package-b"])
        .assert();

    cargo_codspeed(&dir)
        .arg("run")
        .args(["--package", "package-a"])
        .assert()
        .success()
        .stderr(contains("Finished running 1 benchmark suite(s)"));
    teardown(dir);
}

#[test]
fn test_workspace_build_both_and_run_all() {
    let dir = setup(DIR, Project::Workspace);
    cargo_codspeed(&dir)
        .arg("build")
        .args(["--package", "package-a"])
        .assert();

    cargo_codspeed(&dir)
        .arg("build")
        .args(["--package", "package-b"])
        .assert();

    cargo_codspeed(&dir)
        .arg("run")
        .assert()
        .success()
        .stderr(contains("Finished running 3 benchmark suite(s)"));
    teardown(dir);
}
