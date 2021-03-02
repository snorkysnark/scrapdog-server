use std::path::Path;

pub trait ProjectDirs {
    //fn config_dir(&self) -> &Path;
    fn data_dir(&self) -> &Path;
}

impl ProjectDirs for directories_next::ProjectDirs {
//    fn config_dir(&self) -> &Path {
//        self.config_dir()
//    }
//
    fn data_dir(&self) -> &Path {
        self.data_dir()
    }
}
