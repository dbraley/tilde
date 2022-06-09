fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        // skip header row and empty lines
        if i == 0 || record.trim().len() == 0 {
            if cfg!(debug_assertions) {
                eprintln!("Skipping Record ({:?}, {:?})", i, record)
            }
            continue;
        }

        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields)
        }

        let name = fields[0];
        // parse will return an error for the record "Invalid,data"
        // if we include a record "Invalid_data", `fields[1]` will cause a panic
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
