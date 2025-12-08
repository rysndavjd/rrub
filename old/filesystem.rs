use core::marker::PhantomData;

pub enum FilesystemError {}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, FilesystemError>;

    fn seek_relative(&mut self, offset: i64) -> Result<(), FilesystemError> {
        self.seek(SeekFrom::Current(offset))?;
        Ok(())
    }
}

pub trait FilesytemBackend {}

pub struct File<B: FilesytemBackend> {
    _backend: PhantomData<B>,
}
