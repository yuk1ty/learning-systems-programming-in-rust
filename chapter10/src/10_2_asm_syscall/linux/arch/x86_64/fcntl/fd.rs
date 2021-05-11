/// ファイルディスクリプタ
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct Fd(i32);

// TODO Drop で close
