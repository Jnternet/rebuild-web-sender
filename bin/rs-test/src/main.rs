fn main() {
    a();
}
use std::fs;
use std::time;

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
