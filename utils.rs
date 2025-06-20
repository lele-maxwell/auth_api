pub struct Config {
    pub jwt_secret: String,
    pub jwt_duration: String,
}

pub fn load_env() -> Config {
  dotenv::dotenv().ok();

  let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  let jwt_duration = std::env::var("JWT_DURATION").expect("JWT_DURATION must be set");

 return Config {
      jwt_secret,
      jwt_duration,
  }
}
