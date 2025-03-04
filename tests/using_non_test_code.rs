#[cfg(test)]
mod tests {
    use mockcmd::Command;

    #[test]
    fn using_non_test_code() {
        let mut cmd = Command::new("echo");
        cmd.arg("hello");
        let output = cmd.output().unwrap();
        assert_eq!(output.status.success(), true);
        assert_eq!(output.stdout, b"hello\n");
    }
}
