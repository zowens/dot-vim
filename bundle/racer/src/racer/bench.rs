extern crate test;

use std::old_io::fs::PathExtensions;
use std::os::getenv;
use std::old_io::File;
use self::test::Bencher;
use racer::codecleaner::code_chunks;
use racer::codeiter::iter_stmts;

fn get_rust_file_str(path: &[&str]) -> String {

    let mut src_path = match getenv("RUST_SRC_PATH") {
        Some(env) => { Path::new(&env[]) },
        None => panic!("Cannot find $RUST_SRC_PATH")
    };
    for &s in path.iter() { src_path.push(s); }

    File::open(&src_path).read_to_string().unwrap()
}

#[bench]
fn bench_code_chunks(b: &mut Bencher) {
    let src = &get_rust_file_str(&["libcollections", "bit.rs"])[];
    b.iter(|| {
        let chunks = code_chunks(src).collect::<Vec<_>>();
    });
}

#[bench]
fn bench_iter_stmts(b: &mut Bencher) {
    let src = &get_rust_file_str(&["libcollections", "bit.rs"])[];
    b.iter(|| {
        let chunks = iter_stmts(src).collect::<Vec<_>>();
    });
}

