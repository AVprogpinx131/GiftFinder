use chrono::{NaiveDate, Weekday, Datelike};
use inquire::DateSelect;
use inquire::Select;
use std::collections::HashMap;
use std::error::Error;

enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn from_u32(month: u32) -> Option<Month> {
        match month {
            1 => Some(Month::January),
            2 => Some(Month::February),
            3 => Some(Month::March),
            4 => Some(Month::April),
            5 => Some(Month::May),
            6 => Some(Month::June),
            7 => Some(Month::July),
            8 => Some(Month::August),
            9 => Some(Month::September),
            10 => Some(Month::October),
            11 => Some(Month::November),
            12 => Some(Month::December),
            _ => None,
        }
    }
}

pub fn choose_calendar_date() -> Result<i32, Box<dyn Error>> {
    let date = DateSelect::new("Your birthday")
        .with_starting_date(NaiveDate::from_ymd(2024, 1, 1))
        .with_min_date(NaiveDate::from_ymd(2024, 1, 1))
        .with_max_date(NaiveDate::from_ymd(2024, 12, 31))
        .with_week_start(Weekday::Mon)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt()?;

    let month_num = date.month();
    let month = Month::from_u32(month_num).expect("Invalid month");

    let salary = match month {
        Month::January => 100,
        Month::February => 200,
        Month::March => 300,
        Month::April => 400,
        Month::May => 500,
        Month::June => 600,
        Month::July => 700,
        Month::August => 800,
        Month::September => 900,
        Month::October => 1000,
        Month::November => 1100,
        Month::December => 1200,
    };

    println!("Your total salary is: {}$", salary);
    
    Ok(salary)
}

pub fn choose_birthday_item(mut salary: i32) {
    let mut gifts: HashMap<&str, i32> = HashMap::from([("laptop", 1000), ("smartphone", 500), ("tablet", 300), ("headphones", 200), 
                                    ("smartwatch", 150), ("earbuds", 100), ("keyboard", 75), 
                                    ("powerbank", 50), ("mouse", 20), ("mousepad", 5)]);

    let mut bought_gifts: Vec<&str> = Vec::new();
    
    loop {
        let gift_options: Vec<&str> = gifts.keys().cloned().collect();

        println!("Total gifts available: {:?}", gift_options.len());

        let selected_gift = Select::new("Choose a birthday gift:", gift_options.clone())
            .prompt()
            .unwrap();

        let gift_price = *gifts.get(selected_gift).unwrap();

        if salary < gift_price {
            println!("You don't have enough money to buy {} which costs {}$", selected_gift, gift_price);
            continue;
        }

        salary -= gift_price;

        println!("You have selected {}, which costs {}$", selected_gift, gift_price);

        bought_gifts.push(selected_gift);
        gifts.remove(selected_gift);

        if salary > 0 {
            println!("Your remaining salary is: {}$", salary);
            let continue_shopping = Select::new("Do you want to buy another gift?", vec!["Yes", "No"])
                .prompt()
                .unwrap();

            if continue_shopping == "No" {
                break;
            }
        } else {
            println!("You don't have enough money to buy any more gifts.");
            break;
        }
    }

    println!("Bought gifts:");

    for gift in bought_gifts {
        println!("{}", gift);
    }

}


