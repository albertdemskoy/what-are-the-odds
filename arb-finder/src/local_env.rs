pub struct LocalEnv<'a> {
    pub odds_api_key: &'a str,
    pub discord_api_key: &'a str,
}

pub const MY_ENV: LocalEnv = LocalEnv {
    discord_api_key: "My Discord API Key",
    odds_api_key: "My Odds API Key",
};
