use arxiv_api::*;
use reqwest;
use feed_rs::parser;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut q = Query::default();

    q.search_query = Some(
        SearchCriteria::new()
            .pure(SearchField::All("nikolas tapia".to_string()))
            .and(SearchField::Title("some paper".to_string())),
    );
    let res = reqwest::get(q.to_string()).await?.text().await?;
    println!("{:#?}", parser::parse(res.as_bytes()).unwrap());
    Ok(())
}
