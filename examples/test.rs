fn main() {
    use kbdgen_init::ir::Layer;
    use kbdgen_init::*;

    update_cldr_repo();
    let locale = match select_base_locale() {
        Some(v) => v,
        None => {
            println!("No valid locale selected; aborting.");
            return;
        }
    };

    println!("Selected locale: '{}'", &locale.0);
    println!("Files: {:#?}", &locale.1);

    // let mut layers = vec![];
    for (key, value) in locale.1 {
        let mut v = value;
        v.sort();
        let last = v.last().unwrap();
        let xml = parse_path(&key, last);
        let maps = xml
            .key_maps
            .into_iter()
            .map(|key_map| {
                (
                    ir::parse_modifiers(key_map.modifiers.as_ref()),
                    Layer::from(&key_map, key != "mobile" && key != "ios" && key != "android"),
                )
            })
            .collect::<Vec<_>>();

        println!("{}:", &key);
        
        for v in maps.iter() {
            println!("\n{}:", v.0);
            println!("{}", String::from(&v.1));
        }

        // let x = match key.as_ref() {
        //     "chromeos" => ("chrome", xml),
        //     "windows" => ("win", xml),
        //     "osx" => ("mac", xml),
        //     "android" => ("android", xml),
        //     _ => continue,
        // };

        // layers.push(x);
    }

    // let k = locale.1.keys().next().unwrap();
    // let xml = parse_path(&k, &locale.1[k][0]);
    // let layer = Layer::from(&xml.key_maps[0]);

    // let mut map = std::collections::HashMap::new();
    // map.insert("test", serde_yaml::Value::from(&layer));
    // let v = serde_yaml::to_string(&map).unwrap();
    // println!("{}", v);
}
