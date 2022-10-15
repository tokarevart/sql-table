use inject::inject;

#[test]
fn no_injection() {
    assert_eq!(inject!("test"), "test");
}

struct FortyTwo {
    n: i64,
}
impl std::fmt::Display for FortyTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.n))
    }
}

#[test]
fn only_injected_value_depth_1() {
    let forty_two = FortyTwo { n: 42 };
    assert_eq!(inject!("#{forty_two}"), "42");
}

#[test]
fn depth_1() {
    let forty_two = FortyTwo { n: 42 };
    assert_eq!(inject!("the answer is #{forty_two}!"), "the answer is 42!");
}

#[test]
fn only_injected_value_depth_2() {
    assert_eq!(inject!("#{FortyTwo { n: 42 }}"), "42");
}

#[test]
fn depth_2() {
    assert_eq!(
        inject!("the answer is #{FortyTwo { n: 42 }}!"),
        "the answer is 42!"
    );
}
