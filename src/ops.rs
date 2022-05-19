pub fn expects_bytes(command: &str) -> bool {
    command.ends_with(".b") || command.ends_with(".B")
}
