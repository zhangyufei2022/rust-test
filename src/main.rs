mod async_await;
mod self_reference;
mod thread;
mod var;
mod weak_and_circle_reference;
mod basic_types;

fn main() {
    println!("Hello, world!");

    greet_world();

    print_info();
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";

    let records = [southern_germany, chinese, english];
    for record in records.iter() {
        println!("{}", &record);
    }
}

fn print_info() {
    let data = "
    zhangsan,160
    lisi,155
    wangwu,165
    invalid,data
    ";

    let records = data.lines();

    for record in records {
        if record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(height) = fields[1].parse::<f32>() {
            println!("name:{}, height:{}", name, height);
        }
    }
}
