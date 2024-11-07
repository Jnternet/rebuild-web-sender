fn main() {
    show_name_and_time();
}
use std::fs;
use std::time;

#[allow(dead_code)]
fn a() {
    let mdata = fs::metadata("test.txt").unwrap();
    let modify = mdata.modified().unwrap();
    // let modify = mdata.modified().unwrap();
    let e = modify.duration_since(time::UNIX_EPOCH).unwrap();
    let du = dbg!(chrono::Duration::from_std(e).unwrap());
    let t = chrono::DateTime::from_timestamp_nanos(du.num_nanoseconds().unwrap());
    dbg!(t);
    let 北京时 = t.with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap());
    let v = 北京时.format("%Y年%m月%d日 %H:%M:%S(北京时)");
    println!("v: {v}")
}

fn show_name_and_time() {
    show_time::show_name_and_time(&entry::get_direntry_with_suffix("./", ".yaml"));
}
