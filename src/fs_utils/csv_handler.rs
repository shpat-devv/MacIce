pub mod csv_handler_mod{
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Seek;
    use std::io::SeekFrom;

    use crate::enums::path_return::return_enum_mod::ReturnPath;

    
    pub struct CsvHandler{
        pub path: String,
        pub file: File
    }

    impl CsvHandler{
        pub fn new(csv_path: String) -> CsvHandler{
            let file_result = File::open(&csv_path);
            let file = match file_result {
                Ok(file) => file,
                Err(error) => panic!("Couldn't open file! {error}")
            };

            let csv_handler: CsvHandler = CsvHandler { path: (csv_path), file: (file) };
            csv_handler
        }

        pub fn read(&self, key: &String) -> ReturnPath{
            let mut file = &self.file;
            file.seek(SeekFrom::Start(0)).expect("Failed to seek to start of file"); //reset file every read since its a shared reference

            let mut csv_file: csv::Reader<&File> = csv::Reader::from_reader(file);
            let key_index = 0;
         
            for result in csv_file.records(){
                let record = match result {
                    Ok(rdr) => rdr,
                    Err(error) => panic!("Couldn't read records from csv file.. {}", error),
                };

                if let Some(field) = record.get(key_index) {
                    if field == key{ //key from first loop iteration
                        if let Some(path) = record.get(key_index + 1) { //path index
                            println!("Found key: {}, value: {}", key, path);
                            return ReturnPath::Path(String::from(path))
                        } else {
                            println!("Key found, but no associated value.");
                        }
                        
                    }
                }
            }
            return ReturnPath::None(false)
        }


        pub fn write(&mut self, key: &String, value: &String) {
            let file = match OpenOptions::new().append(true).open(&self.path) {
                Ok(file) => file,
                Err(error) => {
                    println!("Failed to open CSV file for appending: {}", error);
                    return;
                }
            };

            let mut csv_file = csv::Writer::from_writer(file);

            let record: [&String; 2] = [&key, &value];

            if let Err(error) = csv_file.write_record(&record) {
                println!("Failed to write record to CSV file: {}", error);
                return;
            }
            if let Err(error) = csv_file.flush() {
                println!("Failed to flush CSV writer: {}", error);
            }
        }
    }
}

//TODO: update the writer
//writer overwrites csv header