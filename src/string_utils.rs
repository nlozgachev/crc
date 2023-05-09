pub fn replace_placeholder(content: &str, new: &str) -> String {
    content.replace("COMPONENT", new)
}
