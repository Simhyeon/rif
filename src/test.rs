#[cfg(test)]
mod test {
    use crate::process::Reader;
    use crate::process::Writer;
    use crate::models::{ RifList ,SingleFile};
    use std::path::PathBuf;

    fn write_test() -> Result<(), std::io::Error> {
        Writer::save(
            PathBuf::from("test/test.json"),
            RifList{files: vec![ SingleFile::new("testfile.md") ]}
        )
    }

    fn read_test() -> Result<(), std::io::Error> {
        let rif_list = Reader::read(
            PathBuf::from("test/test.json")
        )?;
        println!("{:?}", rif_list);
        Ok(())
    }
}
