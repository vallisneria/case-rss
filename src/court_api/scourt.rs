use super::{CourtPrecedent, CourtPrecedentDetail, CourtPrecedentOverview};
use serde_json::Value;
use std::error::Error as StdErr;
use wasm_bindgen::JsValue;
use worker::{
    console_log, Cf, Fetch, FormData, Headers, Method, Request as WorkerRequest, RequestInit,
};

const PREC_LIST_CACHE_TTL: u32 = 60 * 15; // 15 min
const PREC_DETAIL_CACHE_TTL: u32 = 60 * 60 * 24; // 1 day

pub async fn get_scourt_precedent_list(
    list_size: u8,
) -> Result<Vec<CourtPrecedent>, Box<dyn StdErr>> {
    let overview_list = get_scourt_case_overview_list(list_size).await?;

    let mut result: Vec<CourtPrecedent> = Vec::new();

    for i in overview_list {
        let detail = match i.detail_file.as_ref() {
            Some(filename) => get_scourt_case_detail(filename).await?,
            None => CourtPrecedentDetail::default(),
        };

        result.push(CourtPrecedent::new(i, detail));
    }

    Ok(result)
}

/// 대법원 판례공보 목록을 가져온다.
async fn get_scourt_case_overview_list(
    size: u8,
) -> Result<Vec<CourtPrecedentOverview>, Box<dyn StdErr>> {
    let url = format!(
        "https://library.scourt.go.kr/api/decisionSearch/result\
            ?&search_category=1&search_kind=2&facet_search_yn=Y&param_orderby_item=DECI_DATE\
            &param_lib_orderby=DESC&param_lib_display={size}&param_pageNo=1"
    );
    let request_init = {
        let mut init = RequestInit::new();
        init.method = Method::Get;
        init.headers = {
            let mut headers = Headers::new();
            headers.append("User-Agent", "Mozilla/5.0")?;
            headers.append("Accept", "application/json")?;
            headers
        };
        init.cf.cache_everything = Some(true);
        init.cf.cache_ttl = Some(PREC_LIST_CACHE_TTL);

        init
    };

    let request = WorkerRequest::new_with_init(&url, &request_init)?;
    let mut response = Fetch::Request(request).send().await?;
    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;
    let result = serde_json::from_value(response_json["resultListData"].to_owned())?;
    Ok(result)
}

/// 대법원 판례공보 세부내용을 가져온다.
async fn get_scourt_case_detail(
    filename: &String,
) -> Result<CourtPrecedentDetail, Box<dyn StdErr>> {
    let url = "https://library.scourt.go.kr/api/decisionSearch/textxml/info".to_string();
    let request_init: RequestInit = {
        let mut init = RequestInit::new();
        init.method = Method::Post;
        init.headers = {
            let mut headers = Headers::new();
            headers.append("Content-Type", "application/x-www-form-urlencoded")?;
            headers
        };

        init.cf.cache_everything = Some(true);
        init.cf.cache_ttl = Some(PREC_DETAIL_CACHE_TTL);

        let year = &filename[0..=1];
        let body = format!("filePath=case_xml%2f20{year}%2f{filename}");
        init.body = Some(JsValue::from(body));

        init
    };

    let request = WorkerRequest::new_with_init(&url, &request_init)?;
    let mut response = Fetch::Request(request).send().await?;
    let response_text = response.text().await?;
    let response_json: Value = serde_json::from_str(&response_text)?;
    let result = serde_json::from_value(response_json["contents"].to_owned())?;

    Ok(result)
}
