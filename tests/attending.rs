use hochzeitsfest::{AttendingField, GuestList};

#[test]
fn values_are_valid() {
    let input = include_str!("../gästeliste.toml");

    let guest_list: GuestList = toml::from_str(input).unwrap();

    for guest in guest_list.guests {
        let Some(attending) = guest.attending else {
            // guest has not decided yet
            continue;
        };

        let name = guest.name;

        let sub_guests: Vec<&str> = name.split(" und ").flat_map(|g| g.split(", ")).collect();

        match attending {
            AttendingField::Single(_) => assert_eq!(
                sub_guests.len(),
                1,
                "please specify 'attending' for each person individually"
            ),
            AttendingField::Table(table) => {
                for name in table.keys() {
                    assert!(
                        sub_guests.contains(&name.as_str()),
                        "unexpected guest: {name}"
                    )
                }
                for name in sub_guests {
                    assert!(table.contains_key(name), "missing guest: {name}")
                }
            }
        }
    }
}
