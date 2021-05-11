#[cfg(test)]
mod test {
    use crate::process::Reader;
    use crate::process::Writer;
    use crate::checker::*;
    use crate::models::{ RifList ,SingleFile, RifError};
    use std::path::PathBuf;
    use std::collections::HashMap;

    #[test]
    fn check_test() -> Result<(), RifError> {
        let mut rif_list = Reader::read(
            PathBuf::from("test/test.json")
        )?;

        let mut checker = Checker::new();
        checker.add_rif_list(&rif_list)?;
        checker.check(&mut rif_list)?;

        Ok(())
    }

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
