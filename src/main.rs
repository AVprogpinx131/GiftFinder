
mod cli_game;

fn main() {
    let salary = cli_game::choose_calendar_date().unwrap();
    
    cli_game::choose_birthday_item(salary);
}
