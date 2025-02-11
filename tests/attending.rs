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

        let name = guest["name"].as_str().unwrap();

        let sub_guests: Vec<&str> = name.split(" und ").flat_map(|g| g.split(", ")).collect();

        fn validate_attending(name: &str, attending: &[toml::Value]) {
            static ALLOWED_VALUES: &[&str] = &["afternoon", "dinner", "hike"];
            for val in attending {
                let val = val.as_str().unwrap();
                assert!(ALLOWED_VALUES.contains(&val), "{name} violated the law");
            }
        }

        if sub_guests.len() > 1 {
            for sub_guest in attending.as_table().unwrap() {
                let name = sub_guest.0.as_str();
                assert!(sub_guests.contains(&name), "unexpected guest: {name}");
                let attending = sub_guest.1.as_array().unwrap();
                validate_attending(name, attending);
            }
        } else {
            let attending = attending.as_array().unwrap();
            validate_attending(name, attending);
        }
    }
}
