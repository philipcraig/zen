fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    use cbindgen::*;

    let config = Config {
        language: Language::Cxx,
        macro_expansion: MacroExpansionConfig { bitflags: true },
        ..Default::default()
    };

    let license = "\
// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the \"hack\" directory of this source tree.";

    let warning = "\
// Warning: Don't edit this! This file is autogenerated by cbindgen.";

    Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .with_header(license)
        .with_autogen_warning(warning)
        .with_pragma_once(true)
        .with_namespace("HPHP")
        .with_namespaces(&["hackc", "hhbc", "ast"])
        .generate()
        .expect("Generation of \"hhbc_ast.h\" failed.")
        .write_to_file("hhbc_ast.h");
}
