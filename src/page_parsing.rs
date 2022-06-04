
use crate::model::*;

use std::error::Error;
use reqwest::{ Client, header };
use scraper::{ ElementRef, Html, Selector };
use futures::future::*;

/// fetches full entries for specific query
pub async fn fetch_entries(query: &str) -> Result<Vec<Entry>, Box<dyn Error>> {
    let page = fetch_search_page(&query).await?;
    Ok(parse_entries_from_page(&page))
}

/// fetches search page HTML for specific query
pub async fn fetch_search_page(query: &str) -> Result<String, Box<dyn Error>> {
    let rq_future = build_client()
        .post("https://howlongtobeat.com/search_results")
        .query(&[("page", "1")])
        .form(&[
            ("page", "1"),
            ("queryString", query),
            ("t", "games"),
            ("sorthead", "popular"),
            ("sortd", "0"),
            ("length_type", "main"),
            ("randomize", "0")
        ])
        .send();

    let rsp = rq_future
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(rsp)
}

/// parse slim entries from search page HTML
pub fn parse_entries_from_page(page: &str) -> Vec<Entry> {
    Html::parse_document(page)
        .select(&Selector::parse("li.back_darkish").unwrap())
        .into_iter()
        .filter_map(parse_element)
        .take(5)
        .collect()
}

/// fetches full page-specific game data using provided link
pub async fn fetch_game_data(link: &str) -> Result<Option<GamePageData>, Box<dyn Error>> {
    let page = build_client()
        .get(link).send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    Ok(parse_game_page(page.as_str()))
}

/// fetches full page-specific game data for each entry in parallel
pub async fn fetch_full_entries(entries: Vec<Entry>) -> Vec<FullEntry> {
    let vec: Vec<FullEntry> = join_all(
        entries
            .into_iter()
            .map(|e| map_full_entry(e))
    )
        .await
        .into_iter()
        .filter_map(|e| e)
        .collect();

    vec
}


fn parse_game_page(page: &str) -> Option<GamePageData> {
    let document = Html::parse_document(page);
    let selector = Selector::parse("#global_site a.text_red").unwrap();
    let links = document.select(&selector);

    for link in links {
        match link.text().next() {
            None => continue,
            Some(text) => {
                if text == "Steam" {
                    let href = link.value().attr("href")?.to_string();
                    return Some(GamePageData::new(href))
                }
            }
        }
    }

    None
}

fn build_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert("Referer", header::HeaderValue::from_static("https://howlongtobeat.com/"));
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36"));

    Client::builder()
        .default_headers(headers)
        .http1_title_case_headers()
        .build().unwrap()
}

fn parse_element(li_el: ElementRef) -> Option<Entry> {
    let link_el = li_el
        .select(&Selector::parse("a").unwrap())
        .next()?;

    let name = link_el.value().attr("title")?;

    let href = link_el.value().attr("href")?;
    let link = std::format!("https://howlongtobeat.com/{}", href);

    let img_href = li_el.select(&Selector::parse("img").unwrap())
        .next()?
        .value()
        .attr("src")?;
    let img = format!("https://howlongtobeat.com{}", img_href);

    let descr = parse_description(&li_el);

    let entry = Entry::new(
        name.to_string(),
        link,
        img.to_string(),
        descr
    );

    Some(entry)
}

fn parse_description(li_el: &ElementRef) -> String {
    let mut descr = String::new();
    let mut t = 0;

    for titbit_el in li_el.select(&Selector::parse("div.search_list_details_block div.search_list_tidbit").unwrap()) {
        t += 1;
        let line = format!("{}{}",
                           titbit_el.text().next().unwrap(),
                           if &t % 2 == 0 { "\n" } else { ": " });
        descr.push_str(line.as_str());
    }

    descr
}

async fn map_full_entry(entry: Entry) -> Option<FullEntry> {
    let result = fetch_game_data(&entry.link).await;
    match result {
        Ok(data) => {
            log::info!("{}: {:?}", entry.name, data);
            Some(FullEntry::new(entry, data))
        }
        Err(e) => {
            log::info!("{}: {:?}", entry.name, e);
            None
        }
    }
}
