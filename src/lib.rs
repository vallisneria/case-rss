mod court_api;
mod rss;

use crate::court_api::CourtType;
use chrono::{NaiveDate, TimeZone};
use chrono_tz::Asia::Seoul;
use court_api::CourtPrecedent;
use rss::{Rss, RssChannelConfig};
use worker::{
    console_log, event, Context, Env, Headers, Request, Response, Result as WorkerResult,
    RouteContext, Router,
};

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> WorkerResult<Response> {
    let router = Router::new();

    router
        .get_async("/scourt.xml", scourt_rss)
        .run(_req, _env)
        .await
}

async fn scourt_rss(_req: Request, _ctx: RouteContext<()>) -> WorkerResult<Response> {
    let items = court_api::scourt::get_scourt_precedent_list(49)
        .await
        .unwrap();

    let config = RssChannelConfig {
        title: "대법원 판례공보",
        link: "https://library.scourt.go.kr/search/judg/press/case",
        description: "대법원 판례공보",
        language: Some("ko-kr"),
        generator: Some("case-rss <https://github.com/vallisneria/case-rss>"),
    };

    let rss = rss::generate_rss(&config, &items);

    Ok(Response::ok(rss).unwrap())
}

impl Rss for CourtPrecedent {
    fn get_guid(&self) -> String {
        format!("{}", self.id)
    }

    fn get_link(&self) -> String {
        let kind_code = match self.court_type {
            CourtType::SupremeCourt => 2,
            _ => 1,
        };

        format!(
            "https://library.scourt.go.kr/search/judg/judgDetail?seqNo={}&kindCode={kind_code}",
            self.id
        )
    }

    fn get_title(&self) -> String {
        format!("{} ({})", self.title, self.full_case_id())
    }

    fn get_author(&self) -> String {
        let judge_name: Vec<String> = self
            .judges
            .iter()
            .map(|judge| judge.name.to_string())
            .collect();

        format!("{} ({})", self.court_type, judge_name.join(", "))
    }

    fn get_pubdate(&self) -> String {
        let date = NaiveDate::parse_from_str(self.pub_date.as_str(), "%Y.%m.%d.")
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();

        Seoul.from_local_datetime(&date).unwrap().to_rfc2822()
    }

    fn get_category(&self) -> String {
        format!("{}", self.case_type)
    }

    fn get_description(&self) -> String {
        format!(
            "<h2>판시사항</h2><p>{}</p><h2>판결요지</h2><p>{}</p>",
            self.judge_abstract, self.judge_note
        )
    }
}
