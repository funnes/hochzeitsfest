#[test]
fn values_are_valid() {
    let input = include_str!("../gästeliste.toml");
    let root_table: toml::Value = toml::from_str(input).unwrap();
    let guests = root_table["guests"].as_array().unwrap();
    for guest in guests {
        let guest = guest.as_table().unwrap();
        let Some(attending) = guest.get("attending") else {
            // guest has not decided yet
            continue;
        };
        let attending = match attending.as_array() {
            Some(array) => array.clone(),
            None => attending
                .as_table()
                .unwrap()
                .into_iter()
                .flat_map(|(_subguest, subattending)| subattending.as_array().unwrap())
                .cloned()
                .collect(),
        };

        let name = guest["name"].as_str().unwrap();

        for val in attending {
            let val = val.as_str().unwrap();

            static ALLOWED_VALUES: &[&str] = &["afternoon", "dinner", "hike"];

            assert!(ALLOWED_VALUES.contains(&val), "{name} violated the law");
        }
    }
}
