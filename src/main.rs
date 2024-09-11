use crate::messenger::MessengerConversation;

use crate::base::adapter::{ConversationConverter, ConversationLoader, MergeImportFiles};
use crate::base::model::ConversationMarker;
use clap::{arg, Parser};
use console::{style, Emoji};
use futures::future::join_all;
use indicatif::ProgressBar;
use itertools::Itertools;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::Error;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;

mod messenger;
mod base;

static PARTICIPANT: Emoji<'_, '_> = Emoji("🙋‍♂️  ", "");
static MESSAGE: Emoji<'_, '_> = Emoji("✉️  ", "");
static REACTION: Emoji<'_, '_> = Emoji("😆  ", "");


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of the messages (messenger)
    #[arg(short, long, default_value_t = String::from("messenger"))]
    app: String,

    /// URL of the Postgres database holding the schema</br>postgres://user:password@address:\[port]/database
    #[arg(short, long)]
    destination: String,

    /// Max connections of the connection pool
    #[arg(short, long, default_value_t = 1)]
    max_connections: u32,

    /// Chunk size of each task importing to database
    #[arg(short, long, default_value_t = 4294967295)]
    chunk_size: u32,

    /// Folder containing files to import
    #[arg(short, long)]
    input: String,
}

fn parse_file<'a, T>(path: String, dest: &'a mut String) -> T
where
    T: Sized,
    T: ConversationMarker,
    T: Deserialize<'a>,
{
    dest.insert_str(0, fs::read_to_string(path).expect("Cannot read file").as_str());
    serde_json::from_str::<T>(dest).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(args.max_connections)
        .connect(&args.destination).await?;

    let paths: Vec<String> = fs::read_dir(args.input)?
        .map(|path| path.unwrap().path().to_str().unwrap().to_string())
        .collect();


    println!("Parsing conversations... {} files", paths.len());
    let conversation = paths.into_iter()
        .map(|path| {
            let mut destination = String::new();
            match args.app.as_str() {
                "messenger" => parse_file::<MessengerConversation>(path, &mut destination),
                _ => panic!("Unknown application name")
            }
        })
        .collect_vec()
        .into_iter()
        .merge_import_files()
        .map(ConversationConverter::convert)
        .unwrap();

    println!(
        "{} {}Importing participants...",
        style("[2/4]").bold().dim(),
        PARTICIPANT
    );
    conversation.load_participants(&pool).await?;

    println!(
        "{} {}Importing messages...",
        style("[3/4]").bold().dim(),
        MESSAGE
    );


    let message_jobs = conversation.load_messages(&pool, Some(args.chunk_size as usize));
    let m = Arc::new(Mutex::new(ProgressBar::new(message_jobs.len() as u64)));

    let messages_with_pb = message_jobs.into_iter().map(|job| async {
        job.await.expect("An error has occurred inserting messages");
        let m_locked = m.lock().await;
        (*m_locked).inc(1);
    });

    join_all(messages_with_pb).await;

    println!(
        "{} {}Importing reactions...",
        style("[4/4]").bold().dim(),
        REACTION
    );
    conversation.load_reactions(&pool).await?;

    Ok(())
}
