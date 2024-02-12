mod xml_parse;

use super::{CourtPrecedent, CourtType};
use worker::{Fetch, Method, Request as WorkerRequest, RequestInit};
use std::error::Error as StdErr;

const PREC_LIST_CACHE_TTL: u32 = 60 * 15;               // 15 min
const PREC_DESCRIPTION_CACHE_TTL: u32 = 60 * 60 * 24;   // 1 day

pub async fn get_court_precedent_list(
    auth: &String,
    court_type: CourtType,
    count: u8,
) -> Vec<CourtPrecedent> {
    let url: String = {
        let org: &str = match court_type {
                CourtType::SupremeCourt => "400201",
                CourtType::Court(_) => "400202"
        };

        format!(
            "https://www.law.go.kr/DRF/lawSearch.do?OC={oc}&target=prec&org={org}&curt={curt}&display={display}&sort=ddes",
            oc = auth, 
            curt = court_type, 
            display = count
        )
    };

    let xml_text: String = _fetch(&url, PREC_LIST_CACHE_TTL).await.unwrap();

    let mut items = xml_parse::parse_court_precedent_list(&xml_text);

    for item in &mut items {
        item.description = Some(get_court_description(&auth, item.id).await);
    }

    items
}

async fn get_court_description(auth: &String, id: u32) -> String {
    let url = format!("http://www.law.go.kr/DRF/lawService.do?OC={auth}&target=prec&ID={id}&type=XML");
    let xml_text = _fetch(&url, PREC_DESCRIPTION_CACHE_TTL).await.unwrap();

    xml_parse::parse_court_description(&xml_text)
}

async fn _fetch(url: &String, cache_ttl: u32) -> Result<String, Box<dyn StdErr>> {
    let request_init: RequestInit = {
        let mut i: RequestInit = RequestInit::new();
        i.method = Method::Get;
        i.cf.cache_everything = Some(true);
        i.cf.cache_ttl = Some(cache_ttl);

        i
    };

    let req: WorkerRequest = WorkerRequest::new_with_init(&url, &request_init)?;
    let resp: Fetch = Fetch::Request(req);
    let mut response = resp.send().await?;
    let content_type = response.headers().get("Content-Type").unwrap();

    if !content_type.is_some_and(|x| x.contains("xml")) {
        panic!("XML이 아님.");
    }

    Ok(response.text().await?)
}
