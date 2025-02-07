// build.rs
fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native=$HOME/miniconda3/envs/rust-python/lib"); // Replace with the actual path to Python's lib directory in your conda env

    // Link against libpythonXY.so
    println!("cargo:rustc-link-lib=python3.13"); // Adjust if python version is different
}
