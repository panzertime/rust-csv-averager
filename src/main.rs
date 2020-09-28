use std::io;
use std::fs::File;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct Application {
    count: u32,
    divisor: u32
}

impl Application {
    fn new(count: u32, divisor: u32) -> Self { Self { count, divisor } }

    fn averager(&self) -> u32 {
        &self.count / &self.divisor
    }

    fn cumulator(&mut self, new_count: u32) {
        self.count += new_count;
        self.divisor += 1;
    }
}

fn main() {
//    print!("Starting up\n");

    
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap();
//    print!("Opening path {}\n", path);
    let input = File::open(path).unwrap();
    let buffered = io::BufReader::new(input);

    let mut list: HashMap<String, Application> = HashMap::new();

    let mut reader = csv::Reader::from_reader(buffered);
    for result in reader.records() {
        let record = result.expect("a CSV record");
//        print!("Got record {:#?}\n", &record);
        let mut key = record[0].trim().to_string();
        key.make_ascii_lowercase();
        let count = record[1].to_string().parse::<u32>().unwrap();
        if list.contains_key(&key) {
            if let Some(row) = list.get_mut(&key) {
//                print!("Updating application {:#?}\n", &row);
                row.cumulator(count);
            }
        }
        else {
            let mut app = Application::new(count.to_owned(), 1);
//            print!("Inserting application {:#?}\n", &app);
            list.insert(
                key.to_owned(),
                app,
            );
        }
    }

    /*
    let total = list.len();
    print!("\n\nTotal applications: {}\n\n\n", total);

    for (app, stats) in list {
        print!("Application :\n\t{}\n\t{}\n", app, stats.averager())
    }
    */
    for (app, stats) in list {
        print!("{},{}\n", app, stats.averager());
    }
}