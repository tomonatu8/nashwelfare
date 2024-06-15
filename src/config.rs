pub struct Config {
    pub num_agents: usize,
    pub num_items: usize,
    pub max_utility: i32,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let num_agents = match args[1].parse() {
            Ok(num) => num,
            Err(_) => return Err("invalid value for n_agents"),
        };

        let num_items = match args[2].parse() {
            Ok(num) => num,
            Err(_) => return Err("invalid value for num_items"),
        };

        let max_utility = match args[3].parse() {
            Ok(num) => num,
            Err(_) => return Err("invalid value for max_utility"),
        };

        Ok(Config {
            num_agents,
            num_items,
            max_utility,
        })
    }
}
