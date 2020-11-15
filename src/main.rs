extern crate log;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    social_dice_roller::rocket().launch();
}
