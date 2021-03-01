use clap::App;
use indicatif::ProgressBar;
use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("Async Copy")
        .version("1.0")
        .author("Aaryamann Challani")
        .about("Asynchronously copy files")
        .arg("-s, --src=[FILE] 'Set the source file'")
        .arg("-d, --des=[FILE] 'Set the destination file'")
        .arg("-r, --recursive 'Copy recursively'")
        .get_matches();

    match (matches.value_of("src"), matches.value_of("des")) {
        (Some(s), Some(d)) => {
            println!("Source File: {}", s);
            println!("Destintation File: {}", d);
            let mut src = match File::open(s).await {
                Ok(s) => s,
                _ => {
                    eprintln!("Source File Not Found!");
                    std::process::exit(1);
                }
            };
            let len = src.metadata().await?.len();
            let mut des = File::create(d).await?;
            io::copy(&mut src, &mut des).await?;
            let pb = ProgressBar::new(len);
            for _ in 0..len {
                pb.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
            pb.finish_with_message("Done 🚀")
        }
        (Some(_), None) => println!("Missing Destination File Path!"),
        (None, Some(_)) => println!("Missing Source File Path!"),
        (None, None) => panic!("No Arguments Passed In. Exiting"),
    };
    Ok(())
}
