#[derive(Serialize, Deserialize)]
pub struct Conf {
    pub run_sbanken_fetch: bool,
    pub run_nynab_update: bool,
}

impl ::std::default::Default for Conf {
    fn default() -> Self { Self {
        run_sbanken_fetch: true,
        run_nynab_update: false,
    }}
}

pub fn load() -> Result<Conf, confy::ConfyError> {
    confy::load_path("./configfile")
}
