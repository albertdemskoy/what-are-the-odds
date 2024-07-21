pub struct LocalEnv<'a> {
    pub odds_api_key: &'a str,
    pub discord_api_key: &'a str,
}

pub const MY_ENV: LocalEnv = LocalEnv {
    discord_api_key: "MTI2MjM3NDY1NTQ5MTMxMzc5NQ.GJBvsj.vrSJAg2c--AtHNRhGgk_nDL56ZVGJXGv9vzuiA",
    odds_api_key: "74b4c29eb501524fc2d16ca5310de51c",
};
