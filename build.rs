fn main() {
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::path::PathBuf;
        use std::process::Command;

        println!("cargo:rerun-if-changed=assets/windows/gruntia.rc");
        println!("cargo:rerun-if-changed=assets/icons/gruntia.ico");

        let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
        let res_path = out_dir.join("gruntia.res");

        let status = Command::new("rc.exe")
            .args([
                "/nologo",
                "/fo",
                res_path.to_str().expect("resource path is not valid UTF-8"),
                "assets/windows/gruntia.rc",
            ])
            .status()
            .expect(
                "failed to run rc.exe; install Windows SDK or run through scripts/cargo-msvc.ps1",
            );

        if !status.success() {
            panic!("rc.exe failed with status {status}");
        }

        println!(
            "cargo:rustc-link-arg={}",
            res_path.to_str().expect("resource path is not valid UTF-8")
        );
    }
}
