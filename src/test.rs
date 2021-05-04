#[cfg(test)]
mod test {
    use crate::process::Reader;
    use crate::process::Writer;
    use crate::models::{ RifList ,SingleFile};
    use std::path::PathBuf;
    use std::collections::HashMap;

    #[test]
    fn write_test() -> Result<(), std::io::Error> {
        let file_path = PathBuf::from("testfile.md");
        let mut hash_map = HashMap::new();
        hash_map.insert(file_path.clone() ,SingleFile::new(file_path.to_str().unwrap().to_owned()));
        Writer::save(
            PathBuf::from("test/test.json"),
            RifList{files: hash_map}
        )?;
        Ok(())
    }

    fn read_test() -> Result<(), std::io::Error> {
        let rif_list = Reader::read(
            PathBuf::from("test/test.json")
        )?;
        println!("{:?}", rif_list);
        Ok(())
    }
}
