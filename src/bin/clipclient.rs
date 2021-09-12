use clipstash::domain::clip::field::{Content, Expires, Password, ShortCode, Title};
use clipstash::service::ask::{GetClip, NewClip, UpdateClip};
use clipstash::web::api::{ApiKey, API_KEY_HEADER};
use clipstash::Clip;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
    Update {
        shortcode: ShortCode,
        clip: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clipclient", about = "ClipStash API Client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLIPSTASH_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: ApiKey,
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get { shortcode, password } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                shortcode,
            };
            todo!()
        }
        Command::New { clip, password, expires, title } => {
            let req = NewClip {
                content: Content::new(clip.as_str())?,
                title: title.unwrap_or_default(),
                expires: expires.unwrap_or_default(),
                password: password.unwrap_or_default(),
            };
            todo!()
        }
        Command::Update { clip, password, expires, title, shortcode } => {
            todo!()
        },
    }
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e);
    }
}