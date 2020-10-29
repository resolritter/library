pub fn format_test_name(raw: &str) -> String {
    let prunned_name = if let Some(idx) = raw.find("::{{closure}}") {
        &raw.as_bytes()[0..idx]
    } else {
        raw.as_bytes()
    };
    let filtered_name = prunned_name
        .iter()
        .map(|c| if *c == b':' { b'_' } else { c.to_owned() })
        .collect::<Vec<u8>>();
    let sliced = filtered_name.as_slice();
    String::from_utf8_lossy(sliced).to_string()
}
