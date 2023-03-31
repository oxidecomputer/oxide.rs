// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

fn main() {
    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    match built::util::get_repo_head(src.as_ref()) {
        Ok(Some((_branch, _commit, _commit_short))) => (),
        Ok(None) => panic!("Error: Build script could not find git commit information"),
        Err(e) => panic!("Build script error: {}", e),
    };

    built::write_built_file().expect("Failed to acquire build-time information");
}
