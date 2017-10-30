mod support;

use support::{assert, exe, fixture};


#[test]
fn test_help() {
    let out = exe::run(&["--help"]);

    assert::exit_ok(&out);
    assert::stdout_includes(&out, "USAGE");
}

#[test]
fn test_positional() {
    let path = &fixture::path("all_numeric_lines");

    assert::exit_fail(&exe::run(&[]));
    assert::exit_ok(&exe::run(&[path]));
    assert::exit_ok(&exe::run(&[path, path]));
    assert::exit_ok(&exe::run(&[path, path, path]));
}

#[test]
fn test_stdin() {
    let file = fixture::file("all_numeric_lines");
    let out = exe::run_with_stdin(file, &["-s"]);

    assert::exit_ok(&out);
}

#[test]
fn test_lax() {
    {
        let path = &fixture::path("all_numeric_lines");
        assert::exit_ok(&exe::run(&[path]));
        assert::exit_ok(&exe::run(&[path, "--lax"]));
    }
    {
        let path = &fixture::path("bad_lines");
        assert::exit_fail(&exe::run(&[path]));
        assert::exit_ok(&exe::run(&[path, "--lax"]));
    }
    {
        let path = &fixture::path("empty_lines");
        assert::exit_ok(&exe::run(&[path]));
        assert::exit_ok(&exe::run(&[path, "--lax"]));
    }
    {
        let path = &fixture::path("trailing_empty_lines");
        assert::exit_ok(&exe::run(&[path]));
        assert::exit_ok(&exe::run(&[path, "--lax"]));
    }
}

#[test]
fn test_comparison() {
    let path = &fixture::path("all_numeric_lines");
    let out = exe::run(&[path, path]);

    assert::exit_ok(&out);
    assert::stderr_is_empty(&out);
    assert::stdout_eq_file(&out, "comparison.out");
}

#[test]
fn test_comparison_plot() {
    let path1 = &fixture::path("normal_0_1");
    let path2 = &fixture::path("normal_5_2");
    let out = exe::run(&["-p", "-w", "90", path1, path2]);

    assert::exit_ok(&out);
    assert::stderr_is_empty(&out);
    assert::stdout_eq_file(&out, "comparison_plot.out");
}

#[test]
fn test_plot_one() {
    let path = &fixture::path("normal_0_1");
    let out = exe::run(&["-p", "-w", "90", path]);

    assert::exit_ok(&out);
    assert::stderr_is_empty(&out);
    assert::stdout_eq_file(&out, "plot_one.out");
}

#[test]
fn test_plot_one_long() {
    let path = &fixture::path("normal_0_1");
    let out = exe::run(&["--plot", "-w", "90", path]);

    assert::exit_ok(&out);
    assert::stderr_is_empty(&out);
    assert::stdout_eq_file(&out, "plot_one.out");
}

#[test]
fn test_plot_many() {
    let paths = vec![
        fixture::path("normal_0_1"),
        fixture::path("normal_5_2"),
        fixture::path("normal_3_1"),
    ];
    let out = exe::run(&["-p", "-w", "90", &paths[0], &paths[1], &paths[2]]);

    assert::exit_ok(&out);
    assert::stderr_is_empty(&out);
    assert::stdout_eq_file(&out, "plot_many.out");
}
