pub mod csv_handler_mod{
    pub struct CsvHandler{
        pub path: String,
    }

    impl CsvHandler{
        pub fn new(csv_path: String) -> CsvHandler{
            let csv_handler = CsvHandler { path: (csv_path) };
            csv_handler
        }
        pub fn read(&self){

        }
        pub fn write(&mut self, key: String, value: String){

        }
    }
}