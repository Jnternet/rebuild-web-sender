pub fn set_str_to_clipboard(s: &str) -> anyhow::Result<()> {
    let mut cb = arboard::Clipboard::new()?;
    cb.set_text(s)?;
    anyhow::Ok(())
}
