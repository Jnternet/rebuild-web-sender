pub fn choose_file(v: &[walkdir::DirEntry]) -> anyhow::Result<walkdir::DirEntry> {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    eprint!("输入你要选择的文件序号:");
    stdin.read_line(&mut buf)?;

    //parse之前记得先trim
    let n = buf.trim().parse::<usize>()?;
    let entry = v.get(n);
    match entry {
        Some(s) => anyhow::Ok(s.to_owned()),
        None => Err(anyhow::Error::msg("范围错误")),
    }
}
