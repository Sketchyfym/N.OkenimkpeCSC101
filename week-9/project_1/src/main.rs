use::std::io::Write;

fn main() {

    let lager = "33 Exports, Desperados, Goldberg, Gulder, Heineken, Star\n";
    let stout = "Legend, Turbo King, Williams\n";
    let non_alcoholic = "Maltina, Amstel Malta, Malta Gold, Fayrouz\n";

    let mut file = std::fs::File::create("drinks.txt").expect("create failed");
    file.write_all("List of high quality drink categories\n"
        .as_bytes()).expect("write failed");
    file.write_all("Lager:\n"
        .as_bytes()).expect("Write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all("Stout:\n"
        .as_bytes()).expect("Write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all("Non-Alcogolic:\n"
        .as_bytes()).expect("Write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");
    print!("Write Complete");
}
