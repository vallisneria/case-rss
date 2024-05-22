use rss::{CategoryBuilder, ChannelBuilder, EnclosureBuilder, GuidBuilder, Item, ItemBuilder};

pub trait Rss {
    fn get_title(&self) -> String;
    fn get_link(&self) -> String;
    fn get_description(&self) -> String;
    fn get_guid(&self) -> String;
    fn get_author(&self) -> String;
    fn get_category(&self) -> String;
    fn get_pubdate(&self) -> String;
}

pub struct RssChannelConfig<'a> {
    pub title: &'a str,
    pub link: &'a str,
    pub description: &'a str,
    pub language: Option<&'a str>,
    pub generator: Option<&'a str>,
}

pub fn generate_rss<T: Rss>(config: &RssChannelConfig, items: &Vec<T>) -> String {
    let mut rss_items: Vec<Item> = Vec::new();

    for item in items {
        let guid = GuidBuilder::default()
            .value(item.get_guid())
            .permalink(false)
            .build();

        let category = CategoryBuilder::default().name(item.get_category()).build();

        let rss_item = ItemBuilder::default()
            .title(item.get_title())
            .link(item.get_link())
            .description(item.get_description())
            .author(item.get_author())
            .guid(Some(guid))
            .categories(vec![category])
            .pub_date(item.get_pubdate())
            .build();

        rss_items.push(rss_item);
    }

    ChannelBuilder::default()
        .title(config.title)
        .link(config.link)
        .description(config.description)
        .items(rss_items)
        .build()
        .to_string()
}
