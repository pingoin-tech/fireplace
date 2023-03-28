pub fn format_mac(mac: String) -> String {
    let mut lower = mac.to_lowercase();
    if lower.len() == 12 {
        lower.insert(2, ':');
        lower.insert(5, ':');
        lower.insert(8, ':');
        lower.insert(11, ':');
        lower.insert(14, ':');
    }
    lower
}
