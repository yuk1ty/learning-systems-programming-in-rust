use std::os::unix::fs::PermissionsExt;
use std::{
    env::temp_dir,
    fs::{OpenOptions, Permissions},
    io::Write,
};
use std::{fs::File, io::Read, str};

use memmap2::MmapMut;

const UTF8_ERROR_MSG: &str = "should\'ve safely converted into Utf-8";

fn main() -> std::io::Result<()> {
    let mut test_data = "0123456789ABCDE".as_bytes();
    let mut test_path = temp_dir();
    test_path.push("test_data");
    let mut file = File::create(test_path.clone())?;
    file.write_all(&mut test_data)?;

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(test_path)
        .unwrap();
    f.set_permissions(Permissions::from_mode(0o644))?;

    // mmap-go の代わりに memmap2 というクレートを利用できる。
    let mut m = unsafe { MmapMut::map_mut(&mut f)? };

    // Go 言語の場合、下記のように `defer m.Unmap()` が記述されているが、
    // `MmapMut` が中に持つ `MmapInner` に `Drop` トレイトが実装されており、
    // drop されるタイミングで munmap(2) が走るようになっている。
    // よって、明示的に何か unmap に関する処理を書く必要はない。
    // defer m.Unmap()

    // `MmapMut` は添字アクセスができる。
    m[9] = "X".as_bytes()[0];
    // あるいは、下記のように書くこともできる。
    // if let Some(elem) = m.get_mut(9) {
    //     *elem = "X".as_bytes()[0];
    // }

    m.flush().unwrap();

    // 読み込んでみる
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;

    println!(
        "original: {}",
        str::from_utf8(test_data).expect(UTF8_ERROR_MSG)
    );
    println!(
        "mmap:     {}",
        str::from_utf8(&m[..m.len()]).expect(UTF8_ERROR_MSG)
    );
    println!(
        "file:     {}",
        str::from_utf8(buf.as_ref()).expect(UTF8_ERROR_MSG)
    );

    Ok(())
}
