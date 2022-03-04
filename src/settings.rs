use std::collections::BTreeMap;

pub struct FileData {
    pub(crate) functions : BTreeMap<String,Vec<String>>
}
impl FileData{
    pub fn new() -> Self{
        FileData{
            functions: Default::default(),
        }
    }
}