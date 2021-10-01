use procfs::process::Stat;
use std::{
    convert::TryFrom,
    fs::read,
    io::{Cursor, Error as IoError},
    num::TryFromIntError,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
pub enum UnixError {
    #[error("failed to read from file {}", .path.display())]
    FileRead {
        #[source]
        error: IoError,
        path: PathBuf,
    },
    #[error("file contents are in an unexpected format")]
    FileContentsMalformed,
    #[error("call to system-native API errored")]
    SystemCall(#[from] IoError),
    #[error(transparent)]
    ConversionError(#[from] TryFromIntError),
}

pub fn get_memory() -> Result<u64, UnixError> {
    let bytes_per_page = procfs::page_size().map_err(UnixError::from)?;

    let path = Path::new("/proc/self/stat");
    let file_contents = read(path).map_err(|err| UnixError::FileRead {
        error: err,
        path: path.to_path_buf(),
    })?;

    let readable_string = Cursor::new(file_contents);
    let stat_file =
        Stat::from_reader(readable_string).map_err(|_| UnixError::FileContentsMalformed)?;

    Ok(u64::try_from(stat_file.rss)? * u64::try_from(bytes_per_page)?)
}
