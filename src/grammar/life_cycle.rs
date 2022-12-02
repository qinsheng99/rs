#[allow(dead_code)]
pub fn max_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 生命周期取参数中短的那个
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
