use procfs::process::Stat;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::read,
    io::{Cursor, Error as IoError},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct UnixError {
    kind: UnixErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl UnixError {
    pub const fn kind(&self) -> &UnixErrorType {
        &self.kind
    }

    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    pub fn into_parts(self) -> (UnixErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }

    fn system_call(source: IoError) -> Self {
        Self {
            kind: UnixErrorType::SystemCall,
            source: Some(Box::new(source)),
        }
    }

    fn file_read(source: IoError, path: PathBuf) -> Self {
        Self {
            kind: UnixErrorType::FileRead(path),
            source: Some(Box::new(source)),
        }
    }

    fn file_contents_malformed() -> Self {
        Self {
            kind: UnixErrorType::FileContentsMalformed,
            source: None,
        }
    }
}

impl Display for UnixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            UnixErrorType::FileRead(ref path) => {
                f.write_str("failed to read from file ")?;
                Display::fmt(&path.display(), f)
            }
            UnixErrorType::FileContentsMalformed => {
                f.write_str("file contents are in unexpected format")
            }
            UnixErrorType::SystemCall => f.write_str("call to system-native API errored"),
        }
    }
}

impl Error for UnixError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

#[derive(Debug, Clone)]
pub enum UnixErrorType {
    FileRead(PathBuf),
    FileContentsMalformed,
    SystemCall,
}

pub fn get_memory() -> Result<u64, UnixError> {
    let bytes_per_page = procfs::page_size().map_err(UnixError::system_call)?;

    let path = Path::new("/proc/self/stat");
    let file_contents = read(path).map_err(|err| UnixError::file_read(err, path.to_path_buf()))?;

    let readable_string = Cursor::new(file_contents);
    let stat_file =
        Stat::from_reader(readable_string).map_err(|_| UnixError::file_contents_malformed())?;

    Ok((stat_file.rss as u64) * (bytes_per_page as u64))
}
