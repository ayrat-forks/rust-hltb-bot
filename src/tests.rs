#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;

    use crate::page_parsing::*;
    use crate::formatting::*;

    #[tokio::test]
    async fn formatting_flow() {
        // let page = fetch_page("Skyrim").await?;
        let page = fs::read_to_string("./page.html").unwrap();

        let entries = parse_entries_from_page(&page);
        log::info!("found {} entries", entries.len());

        let msg = format_msg_initial(&entries);
        log::info!("{}", msg);

        let vec = fetch_full_entries(entries).await;
        log::info!("{}", populate_page_data(&msg, &vec));
    }

    #[tokio::test]
    async fn fetch_entries_flow() -> Result<(), Box<dyn Error>> {
        let entries = fetch_entries(&"skyrim").await?;
        assert_eq!(entries.len(), 5);

        let full_entries = fetch_full_entries(entries).await;
        assert_eq!(full_entries.len(), 5);

        Ok(())
    }
}