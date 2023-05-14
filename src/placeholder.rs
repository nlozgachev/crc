pub fn replace_placeholder(content: &str, replace_to: &str) -> String {
    content.replace("COMPONENT", replace_to)
}
