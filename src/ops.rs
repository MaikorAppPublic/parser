pub fn expects_bytes(command: &str) -> bool {
    let cmd = command.to_ascii_lowercase();

    cmd.ends_with(".b")
        || cmd.contains("mcpy")
        || cmd.contains("mswp")
        || cmd.contains("jbc")
        || cmd.contains("jbs")
        || cmd.contains("jrf")
        || cmd.contains("jrb")
}
