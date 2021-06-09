use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct BucketsFolder {
    path: PathBuf,
}

impl BucketsFolder {
    pub fn from_path(path: PathBuf) -> Self {
        BucketsFolder { path }
    }

    pub fn get_path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn make_bucket_path(&self, bucket_id: i32) -> PathBuf {
        self.path.join(bucket_id.to_string())
    }

    pub fn make_bucket_path_create(&self, bucket_id: i32) -> io::Result<PathBuf> {
        let path = self.make_bucket_path(bucket_id);
        fs::create_dir(&path)?;
        Ok(path)
    }
}
