use aws_sdk_s3 as s3;
use aws_sdk_s3::types::Object;
use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Clone, ValueEnum)]
enum ObjectType {

    #[value(alias = "f")]
    File,

    #[value(alias = "d")]
    Directory
}

#[derive(Parser)]
struct Arguments {

    #[arg(short = 'b', long = "bucket")]
    bucket: String,

    #[arg(short = 'n', long = "name", value_parser = Regex::new)]
    name: Regex,

    #[arg(short = 't', long = "type")]
    r#type: ObjectType
}

async fn is_valid_bucket(client: &s3::Client, bucket_name: &str) -> bool {
    let Ok(result) = client.list_buckets().send().await else {
        eprintln!("Failed to get the list of buckets from AWS");
        std::process::exit(1)
    };
    result
        .buckets
        .iter()
        .flatten()
        .map(|e| e.name().unwrap() )
        .find(|e| e.eq(&bucket_name))
        .is_some()
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let aws_sdk_behavior = aws_config::BehaviorVersion::latest();
    let config = aws_config::load_defaults(aws_sdk_behavior).await;
    let client = s3::Client::new(&config);

    if !is_valid_bucket(&client, &args.bucket).await {
        eprintln!("Bucket does not exist: {}", args.bucket);
        std::process::exit(1)
    }

    let obj_type_filter = match args.r#type {
        ObjectType::File => |e: &Object| e.size().ne(&Some(0)),
        ObjectType::Directory => |e: &Object| e.size().eq(&Some(0))
    };

    let mut paginator = client
        .list_objects_v2()
        .set_bucket(Some(args.bucket))
        .into_paginator()
        .page_size(50)
        .send();
    while let Some(response) = paginator.next().await {
        let Ok(objects) = response
        else {
            eprintln!("Failed to fetch object list");
            std::process::exit(1)
        };
        objects
            .contents()
            .into_iter()
            .filter(|e| obj_type_filter(e))
            .filter(|e| e.key().is_some())
            .filter(|e| args.name.is_match(e.key().unwrap()) )
            .map(|e| e.key.clone().unwrap() )
            .for_each(|e| println!("{e}"));
    };
}
