use structopt::StructOpt;

const URL: &str = "http://onefile.clowzed.ru:8081";

#[derive(StructOpt)]
#[structopt(name = "onefile", about = "Cli for uploading and oening files")]
struct Args {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Upload { path: std::path::PathBuf },
    Open { key: String },
}

fn main() {
    let opt = Args::from_args();

    match opt.command {
        Command::Upload { path } => {
            let form = reqwest::blocking::multipart::Form::new()
                .file("file", &path)
                .unwrap();

            let client = reqwest::blocking::Client::new();

            match client
                .post(URL.to_string() + "/upload")
                .multipart(form)
                .send()
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!(
                            "{}",
                            resp.text().unwrap_or_else(|_| "no key available".into())
                        );
                    } else {
                        println!(
                            "error: {:?}",
                            resp.text().unwrap_or_else(|_| "unknown error".into())
                        );
                    }
                }
                Err(err) => {
                    println!("error: {:?}", err.to_string());
                }
            }
        }
        Command::Open { key } => {
            match webbrowser::open(&(URL.to_string() + &format!("/get/{}", key))) {
                Ok(()) => {}
                Err(err) => {
                    println!("error: {:?}", err.to_string());
                }
            }
        }
    }
}
