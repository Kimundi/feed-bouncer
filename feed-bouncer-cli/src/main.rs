use std::path::PathBuf;

use clap::Parser;
use feed_bouncer_database::Database;
use feed_bouncer_database::Error as DbError;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
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
    let tasks = db.update_feeds_task();
    let results = tasks.run().await;
    db.commit_from(results).await;
    db.save();

    if opts.recent {
        println!();
        println!("Recent updates:");
        let mut feeds = db.get_items_ordered_by_time();
        feeds.reverse();
        let feeds = &feeds[0..(feeds.len().min(10))];
        for (_feed_id, feed, item) in feeds {
            println!(
                "  [{}] {}",
                feed.display_name(),
                item.item
                    .display_title_without_prefixes(&feed)
                    .unwrap_or("???")
            );
        }
    }

    Ok(())
}
