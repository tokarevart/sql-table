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
fn braces_outside_interpolation() {
    assert_eq!(inject!("some {{ test }} text"), "some { test } text");
}

#[test]
fn only_injected_value_depth_1() {
    let forty_two = FortyTwo { n: 42 };
    assert_eq!(inject!("#{forty_two}#"), "42");
}

#[test]
fn depth_1() {
    let forty_two = FortyTwo { n: 42 };
    assert_eq!(inject!("the answer is #{forty_two}#!"), "the answer is 42!");
}

#[test]
fn front_and_back_seqs_iterpolation() {
    assert_eq!(
        inject!("front sequence is #{'#'}##{'{'}# and back sequence is #{'}'}##{'#'}#!"),
        "front sequence is #{ and back sequence is }#!"
    );

    // or

    let (f, b) = ("#{", "}#");
    assert_eq!(
        inject!("front sequence is #{f}# and back sequence is #{b}#!"),
        "front sequence is #{ and back sequence is }#!"
    );
}

#[test]
fn only_injected_value_depth_2() {
    assert_eq!(inject!("#{FortyTwo { n: 42 }}#"), "42");
}

#[test]
fn depth_2() {
    assert_eq!(
        inject!("the answer is #{FortyTwo { n: 42 }}#!"),
        "the answer is 42!"
    );
}
