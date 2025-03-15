fn main() {
    glib_build_tools::compile_resources(
        &["content"],
        "content/org.gtk_rs.content.gresource.xml",
        "content.gresource",
    );
}