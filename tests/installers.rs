//! The installers, the package metadata and the README must all resolve to the same repository,
//! so installing from this repository installs this repository's release, and the installer
//! workflow must exercise all three documented paths: the default source, an explicit
//! NEOVAIN_VERSION pin and the NEOVAIN_REPO override (issue #4).

use std::fs;

/// The repository this fork installs from.
const CANONICAL: &str = "alfonsodg/neovain";

fn read(name: &str) -> String {
    fs::read_to_string(name).unwrap_or_else(|e| panic!("cannot read {name}: {e}"))
}

#[test]
fn installers_default_to_the_canonical_repository_and_allow_an_override() {
    let sh = read("install.sh");
    let ps1 = read("install.ps1");

    // The default is this repository, with NEOVAIN_REPO documented as the override.
    assert!(
        sh.contains(&format!("repo=\"${{NEOVAIN_REPO:-{CANONICAL}}}\"")),
        "install.sh must default to {CANONICAL}"
    );
    assert!(
        sh.contains(&format!(
            "raw.githubusercontent.com/{CANONICAL}/main/install.sh"
        )),
        "install.sh usage must point at {CANONICAL}"
    );
    assert!(
        !sh.contains("kbrock84/neovain"),
        "install.sh still points at upstream"
    );

    assert!(
        ps1.contains(&format!("}} else {{ '{CANONICAL}' }}")),
        "install.ps1 must default to {CANONICAL}"
    );
    assert!(
        ps1.contains("$env:NEOVAIN_REPO"),
        "install.ps1 must document NEOVAIN_REPO"
    );
    assert!(
        !ps1.contains("kbrock84/neovain"),
        "install.ps1 still points at upstream"
    );
}

#[test]
fn the_default_repository_is_the_one_every_document_names() {
    let cargo = read("Cargo.toml");
    let readme = read("README.md");

    assert!(
        cargo.contains(&format!("repository = \"https://github.com/{CANONICAL}\"")),
        "Cargo.toml must advertise {CANONICAL}"
    );
    assert!(
        readme.contains(&format!(
            "raw.githubusercontent.com/{CANONICAL}/main/install.sh"
        )),
        "README must install from {CANONICAL}"
    );
    assert!(
        readme.contains(&format!(
            "raw.githubusercontent.com/{CANONICAL}/main/install.ps1"
        )),
        "README must install from {CANONICAL}"
    );
    assert!(
        !readme.contains("kbrock84/neovain"),
        "README still points at upstream"
    );
}

#[test]
fn the_workflow_exercises_all_three_install_paths() {
    let wf = read(".github/workflows/installers.yml");

    // The default install step must not redirect the repository: it tests this fork's releases.
    for step in wf
        .split("name: Install neovain and Neovim into an empty directory")
        .skip(1)
    {
        let env = &step[..step
            .find("run:")
            .expect("the default install step must run something")];
        assert!(
            !env.contains("NEOVAIN_REPO"),
            "the default install step overrides the repository"
        );
    }

    // The explicit pin keeps its own coverage, and so does the override.
    assert!(
        wf.contains("NEOVAIN_VERSION"),
        "the workflow must test the explicit version path"
    );
    assert!(
        wf.contains("NEOVAIN_REPO: kbrock84/neovain"),
        "the workflow must keep the NEOVAIN_REPO override coverage"
    );
    assert!(
        !wf.contains("do not exist yet"),
        "the workflow still assumes this fork has no releases"
    );
}
