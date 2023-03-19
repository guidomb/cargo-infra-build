use std::path::PathBuf;

#[test]
fn route() {
    let base_path = PathBuf::from_iter(["tests", "route"]);
    let t = trybuild::TestCases::new();
    t.pass(base_path.join("basic.rs"));
    t.compile_fail(base_path.join("too_few_route_args.rs"));
    t.compile_fail(base_path.join("too_many_route_args.rs"));
    t.compile_fail(base_path.join("invalid_http_method.rs"));
}
