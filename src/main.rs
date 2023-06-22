

fn main() {
    let gifts = [
        String::from("partridge in a pear tree"),
        String::from("turtle doves, and"),
        String::from("french hens"),
        String::from("calling birds"),
        String::from("golden rings"),
        String::from("geese a laying"),
        String::from("swans a swimming"),
        String::from("maids a milking"),
        String::from("ladies dancing"),
        String::from("lords a sleeping"),
        String::from("pipers piping"),
        String::from("drummers drumming")
    ];

    println!("\n\n");

    for day_num in 0..12 {
        let day = get_day(day_num);
        println!("On the {day} day of Christmas my true love gave to me,");
        for current in (0..=day_num).rev() {
            let gift = gifts
                .get(current as usize)
                .expect("whoops");

            let gift_count = match current {
                0 => String::from("A"),
                num => (num + 1).to_string()
            };
            println!("{gift_count} {gift}");
        }
        println!("\n");
    }
}

fn get_day(round: u8) -> String {
    match round {
        0 => String::from("first"),
        1 => String::from("second"),
        2 => String::from("third"),
        3 => String::from("fourth"),
        4 => String::from("fifth"),
        5 => String::from("sixth"),
        6 => String::from("seventh"),
        7 => String::from("eighth"),
        8 => String::from("ninth"),
        9 => String::from("tenth"),
        10 => String::from("eleventh"),
        11 => String::from("twelfth"),
        _ => panic!("day: {round} is out of range")
    }
}
