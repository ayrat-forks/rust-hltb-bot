use crate::model::*;

pub fn format_msg_initial(entries: &Vec<Entry>) -> String {
    if entries.len() == 0 {
        return "not found".to_string();
    }

    let mut str = String::new();
    try_add_preview_img(&mut str, &entries);
    for entry in entries {
        str.push_str(&format!("*{}* [🗗]({}) [ ](_{}_)\n{}\n",
            clean_md(&entry.name), entry.link, entry.link, clean_md(&entry.descr)
        ))
    }

    str
}

pub fn populate_page_data(msg: &str, full_entries: &Vec<FullEntry>) -> String {
    let mut msg = String::from(msg);

    for full_entry in full_entries {
        let tmp = &format!(" [ ](_{}_)", full_entry.entry.link);

        msg = match &full_entry.page_data {
            None =>
                msg.replace(tmp, ""),

            Some(page_data) =>
                msg.replace(tmp, &format!(" [🗗Steam]({})", page_data.steam))
        }
    }

    msg
}


fn try_add_preview_img(msg: &mut String, entries: &Vec<Entry>) {
    let img_entry = entries.iter()
        .filter(|e| e.img.len() > 0)
        .next();

    match img_entry {
        None => {}
        Some(img_entry) => {
            let escaped_url = escape_url(&img_entry.img);
            let image_link = format!("[ ]({})", escaped_url);
            msg.push_str(image_link.as_str())
        }
    };
}

fn escape_url(str: &str) -> String {
    str
        .replace("(", "%28")
        .replace(")", "%29")
}

fn clean_md(str: &str) -> String {
    str
        .replace("_", "\\_")
        .replace("*", "\\*")
}
