// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() -> anyhow::Result<()> {
    embuild::build::LinkArgs::output_propagated("ESP_IDF")
}
