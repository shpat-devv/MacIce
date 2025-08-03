pub mod csv_handler_mod{
    use std::fs::File;
    
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
        pub fn read(&self, key: String){
            let mut csv_file: csv::Reader<&File> = csv::Reader::from_reader(&self.file);
            let key_index = 0;

            for result in csv_file.records(){
                let record = match result {
                    Ok(rdr) => rdr,
                    Err(error) => panic!("Couldn't read records from csv file.. {}", error),
                };

                if let Some(field) = record.get(key_index) {
                    if field == key {
                        if let Some(path) = record.get(key_index + 1) { //path index
                            println!("Found key: {}, value: {}", key, path);
                        } else {
                            println!("Key found, but no associated value.");
                        }
                    }
                }
            }   
        }


        pub fn write(&mut self, key: String, value: String) {
            let mut csv_file = match csv::Writer::from_path(&self.path) {
                Ok(writer) => writer,
                Err(error) => {
                    eprintln!("Failed to open CSV file for writing: {}", error);
                    return;
                }
            };
            let record = [key, value];
            if let Err(error) = csv_file.write_record(&record) {
                eprintln!("Failed to write record to CSV file: {}", error);
                return;
            }
            if let Err(error) = csv_file.flush() {
                eprintln!("Failed to flush CSV writer: {}", error);
            }
        }
    }
}
