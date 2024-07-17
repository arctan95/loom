use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "loom")]
pub struct ServerConfig {
    /// Listen host
    #[structopt(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,
    /// Listen port
    #[structopt(short, env = "PORT", long, default_value = "8080")]
    pub port: u16,
    /// Database name
    #[structopt(short, env = "DATABASE", long, default_value = "loom")]
    pub database: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            database: "loom.db".to_string(),
        }
    }
}
