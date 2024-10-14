use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct File {
    name: String,
    path: String,
}

#[derive(Deserialize)]
pub(crate) struct Files {
    files: Vec<File>,
}

impl File {
    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}

impl Files {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &File> {
        self.files.iter()
    }
}
