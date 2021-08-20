use std::path::PathBuf;

use clap::{AppSettings, Clap};
use feed_bouncer_database::Database;
use feed_bouncer_database::Error as DbError;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Print recent updates
    #[clap(short, long)]
    recent: bool,

    #[clap(short, long)]
    storage_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), DbError> {
    let opts = Opts::parse();

    let mut db = Database::init(opts.storage_path);
    db.import().await;
    db.update_feeds().await;
    db.save();

    if opts.recent {
        println!();
        println!("Recent updates:");
        let mut feeds = db.get_items_ordered_by_time();
        feeds.reverse();
        let feeds = &feeds[0..(feeds.len().min(10))];
        for (feed, item) in feeds {
            println!(
                "  [{}] {}",
                feed.display_name(),
                item.display_title_without_prefix(&feed.display_name())
                    .unwrap_or("???")
            );
        }
    }

    Ok(())
}
