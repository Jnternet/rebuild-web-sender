use walkdir::DirEntry;
const HOUR: i32 = 3600;
///按从新到旧的顺序展示文件名
pub fn show_name_and_time(v: &[DirEntry]) {
    sort_by_time_new_to_old(v)
        .iter()
        .enumerate()
        .for_each(|(n, d)| {
            println!(
                "{n}: {}\t最后修改时间:{}",
                d.file_name().to_str().unwrap(),
                get_modified_time_str(d)
            )
        });
}

fn get_modified_time_str<'a>(d: &DirEntry) -> String {
    let dura = chrono::Duration::from_std(
        d.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap(),
    )
    .unwrap();
    let d = chrono::DateTime::from_timestamp_nanos(dura.num_nanoseconds().unwrap());
    let bj_time = d.with_timezone(&chrono::FixedOffset::east_opt(8 * HOUR).unwrap());
    bj_time.format("%Y年%m月%d日 %H:%M:%S(北京时)").to_string()
}

fn sort_by_time_new_to_old(v: &[DirEntry]) -> Vec<DirEntry> {
    let mut v = v.to_vec();
    v.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&a.metadata().unwrap().modified().unwrap())
    });
    v
}
