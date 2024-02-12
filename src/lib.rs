mod law_api;
mod rss;

use law_api::request::get_court_precedent_list;
use law_api::{CourtPrecedent, CourtType};
use rss::RssChannelConfig;
use worker::{
    event, Context, Env, Headers, Request, Response, Result as WorkerResult, RouteContext, Router,
};

#[event(fetch)]
async fn main(_req: Request, _env: Env, _ctx: Context) -> WorkerResult<Response> {
    let router = Router::new();

    router
        .get_async("/scourt.xml", scourt_rss)
        .get_async("/court.xml", court_rss)
        .run(_req, _env)
        .await
}

async fn scourt_rss(_req: Request, _ctx: RouteContext<()>) -> WorkerResult<Response> {
    let auth: String = _ctx.secret("AUTH").unwrap().to_string();
    let items: Vec<CourtPrecedent> =
        get_court_precedent_list(&auth, CourtType::SupremeCourt, 49).await;

    let xml_header: Headers = {
        let mut h = Headers::new();
        h.append("Content-Type", "text/xml;charset=utf-8").unwrap();

        h
    };

    let rss_conf: RssChannelConfig = RssChannelConfig {
        title: "대법원 판례 목록",
        link: "https://law.yoyang.one/scourt.rss",
        description: "대법원 판례 목록",
        language: Some("ko-kr"),
    };

    let body = rss::generate_rss(&rss_conf, &items);

    Ok(Response::with_headers(
        Response::ok(&body).unwrap(),
        xml_header,
    ))
}

async fn court_rss(_req: Request, _ctx: RouteContext<()>) -> WorkerResult<Response> {
    let auth: String = _ctx.secret("AUTH").unwrap().to_string();
    let items: Vec<CourtPrecedent> =
        get_court_precedent_list(&auth, CourtType::Court("".to_string()), 49).await;

    let xml_header: Headers = {
        let mut h = Headers::new();
        h.append("Content-Type", "text/xml;charset=utf-8").unwrap();

        h
    };

    let rss_conf: RssChannelConfig = RssChannelConfig {
        title: "하급법원 판례 목록",
        link: "https://law.yoyang.one/court.rss",
        description: "하급법원 판례 목록",
        language: Some("ko-kr"),
    };

    let body = rss::generate_rss(&rss_conf, &items);

    Ok(Response::with_headers(
        Response::ok(&body).unwrap(),
        xml_header,
    ))
}
