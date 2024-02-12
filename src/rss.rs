pub trait Rss {
    fn get_title(&self) -> String;
    fn get_link(&self) -> String;
    fn get_description(&self) -> String;
    fn get_guid(&self) -> String;
    fn get_author(&self) -> String;
    fn get_category(&self) -> String;
}

pub struct RssChannelConfig<'a> {
    pub title: &'a str,
    pub link: &'a str,
    pub description: &'a str,
    pub language: Option<&'a str>,
}

pub fn generate_rss<T: Rss>(config: &RssChannelConfig, items: &Vec<T>) -> String {
    let mut result: String =
        r#"<?xml version="1.0" encoding="UTF-8"?><rss version="2.0"><channel>"#.to_string();

    result =
        format!(
        "{result}<title>{title}</title><link>{link}</link><description>{description}</description>",
        title = config.title, link = config.link, description = config.description
    );

    match config.language {
        Some(lang) => result = format!("{result}<language>{}</language>", lang),
        None => (),
    }

    for item in items {
        result = format!(
            "{result}
<item>
<title><![CDATA[{title}]]></title>
<link>{link}</link>
<description><![CDATA[{description}]]></description>
<guid>{guid}</guid>
<author>{author}</author>
<category>{category}</category>
</item>",
            title = item.get_title(),
            link = item.get_link(),
            description = item.get_description(),
            guid = item.get_guid(),
            author = item.get_author(),
            category = item.get_category()
        );
    }

    format!("{}</channel>", result.replace("\n", ""))
}
