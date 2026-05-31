use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

const TARGET_ARCH: &str = "riscv64gc-unknown-none-elf";
const USER_BASE: u64 = 0x8600_0000;
const APP_STEP: u64 = 0x20_0000;
const CH5_BASE_APPS: &[&str] = &[
    "00hello_world",
    "01store_fault",
    "02power",
    "03priv_inst",
    "04priv_csr",
    "05write_a",
    "06write_b",
    "07write_c",
    "08power_3",
    "09power_5",
    "10power_7",
    "12forktest",
    "13forktree",
    "14forktest2",
    "15matrix",
    "fork_exit",
    "forktest_simple",
    "sbrk",
    "ch5b_usertest",
    "user_shell",
    "initproc",
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TG_USER_DIR");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch != "riscv64" {
        return;
    }

    let user_root = user_root();
    println!(
        "cargo:rerun-if-changed={}",
        user_root.join("Cargo.toml").display()
    );
    println!("cargo:rerun-if-changed={}", user_root.join("src").display());

    let user_target_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("target/user");
    let target_dir = user_target_dir.join(TARGET_ARCH).join("debug");
    let mut bins = Vec::with_capacity(CH5_BASE_APPS.len());
    for name in CH5_BASE_APPS.iter() {
        let base = USER_BASE;
        build_user_app(&user_root, &user_target_dir, name, base);
        let elf = target_dir.join(name);
        bins.push(objcopy_to_bin(&elf));
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let app_asm = out_dir.join("apps.S");
    write_app_asm(&app_asm, &bins);
    println!("cargo:rustc-env=APP_ASM={}", app_asm.display());
}

fn user_root() -> PathBuf {
    if let Ok(path) = env::var("TG_USER_DIR") {
        let path = PathBuf::from(path);
        if path.join("Cargo.toml").exists() {
            return path;
        }
    }

    PathBuf::from("../../user")
}

fn build_user_app(user_root: &Path, user_target_dir: &Path, name: &str, base: u64) {
    let status = Command::new("cargo")
        .args([
            "build",
            "--manifest-path",
            user_root.join("Cargo.toml").to_string_lossy().as_ref(),
            "--bin",
            name,
            "--target",
            TARGET_ARCH,
        ])
        .env("CARGO_TARGET_DIR", user_target_dir)
        .env("BASE_ADDRESS", base.to_string())
        .env("CHAPTER", "-5")
        .env("RUSTFLAGS", "-Aunsafe_op_in_unsafe_fn")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .status()
        .unwrap_or_else(|err| panic!("failed to build user app {name}: {err}"));

    if !status.success() {
        panic!("user app build failed for {name}");
    }
}

fn objcopy_to_bin(elf: &Path) -> PathBuf {
    let bin = elf.with_extension("bin");
    let status = Command::new("rust-objcopy")
        .args([
            elf.to_string_lossy().as_ref(),
            "--strip-all",
            "-O",
            "binary",
            bin.to_string_lossy().as_ref(),
        ])
        .status()
        .unwrap_or_else(|err| panic!("failed to objcopy {}: {err}", elf.display()));

    if !status.success() {
        panic!("rust-objcopy failed for {}", elf.display());
    }

    bin
}

fn write_app_asm(path: &Path, bins: &[PathBuf]) {
    let mut file = fs::File::create(path)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", path.display()));

    writeln!(
        file,
        "\
.global apps
.section .data
.align 3
apps:
    .quad {:#x}
    .quad {:#x}
    .quad {}",
        USER_BASE,
        APP_STEP,
        bins.len()
    )
    .unwrap();

    for index in 0..bins.len() {
        writeln!(file, "    .quad app_{index}_start").unwrap();
    }
    writeln!(file, "    .quad app_{}_end", bins.len() - 1).unwrap();

    for (index, bin) in bins.iter().enumerate() {
        writeln!(
            file,
            "\
app_{index}_start:
    .incbin {bin:?}
app_{index}_end:"
        )
        .unwrap();
    }
}
