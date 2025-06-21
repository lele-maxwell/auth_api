#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_duration_sec: u32,
}

pub fn load_env() -> Config {
  dotenv::dotenv().ok();

  let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  let jwt_duration_sec = std::env::var("JWT_DURATION").expect("JWT_DURATION must be set");
  let jwt_duration_sec = jwt_duration_sec.parse::<u32>().expect("JWT_DURATION must be a number");

  return Config {
      jwt_secret,
      jwt_duration_sec,
  }
}
