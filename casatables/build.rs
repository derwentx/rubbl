extern crate gcc;

const FILES: &[&str] = &[
    "src/glue.cc",
];

fn main() {
    gcc::Build::new()
        .cpp(true)
        .warnings(true)
        .include("../casatables_impl")
        .include("src")
        .files(FILES)
        .compile("libcasatables_glue.a");

    for file in FILES {
        println!("cargo:rerun-if-changed={}", file);
    }
}
