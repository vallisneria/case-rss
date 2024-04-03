pub mod request;

use crate::rss::Rss;
use chrono::{NaiveDate, TimeZone};
use chrono_tz::Asia::Seoul;
use std::{fmt::Display, str::FromStr};

pub enum CourtType {
    /// 대법원
    SupremeCourt,

    /// 각급법원 (법원명)
    Court(String),
}

impl CourtType {
    pub fn new(court_type: String) -> CourtType {
        match court_type.as_str() {
            "대법원" => CourtType::SupremeCourt,
            _ => CourtType::Court(court_type),
        }
    }
}

impl Display for CourtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt: &str = match self {
            CourtType::SupremeCourt => "대법원",
            CourtType::Court(s) => s,
        };

        write!(f, "{}", txt)
    }
}

pub enum DecisionType {
    /// 전원합의체 판결
    EnBank,

    /// 판결
    Judgement,

    /// 결정
    Decision,

    /// 명령
    Order,
}

impl DecisionType {
    pub fn new(decision_type: String) -> DecisionType {
        match decision_type.as_str() {
            "결정" => DecisionType::Decision,
            "명령" => DecisionType::Order,
            "전원합의체 판결" => DecisionType::EnBank,
            _ => DecisionType::Judgement,
        }
    }
}

impl Display for DecisionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let txt = match self {
            DecisionType::EnBank => "전원합의체 판결",
            DecisionType::Judgement => "판결",
            DecisionType::Decision => "결정",
            DecisionType::Order => "명령",
        };

        write!(f, "{}", txt)
    }
}

pub struct CourtPrecedent {
    /// 판례일련번호
    pub id: u32,

    /// 사건명
    pub case_name: Option<String>,

    /// 사건번호
    pub case_code: Option<String>,

    /// 선고일자
    pub decision_date: Option<String>,

    /// 법원
    pub court: Option<CourtType>,

    /// 판결유형
    pub decision_type: Option<DecisionType>,

    /// 사건유형
    pub case_type: Option<String>,

    /// 판결요지
    pub description: Option<String>,
}

impl CourtPrecedent {
    fn new() -> CourtPrecedent {
        CourtPrecedent {
            id: 0,
            case_name: None,
            case_code: None,
            decision_date: None,
            court: None,
            decision_type: None,
            case_type: None,
            description: None,
        }
    }

    pub fn full_code(&self) -> String {
        let sungo: &str = match self.decision_type {
            Some(DecisionType::Judgement) => " 선고",
            Some(DecisionType::EnBank) => " 선고",
            _ => "자",
        };

        format!(
            "{} {}.{} {} {}",
            self.court.as_ref().unwrap(),
            self.decision_date.as_ref().unwrap(),
            sungo,
            self.case_code.as_ref().unwrap(),
            self.decision_type.as_ref().unwrap()
        )
    }
}

impl Rss for CourtPrecedent {
    fn get_title(&self) -> String {
        format!(
            "{} ({})",
            self.case_name.as_ref().unwrap(),
            self.full_code(),
        )
    }

    fn get_author(&self) -> String {
        self.court.as_ref().unwrap().to_string()
    }

    fn get_link(&self) -> String {
        format!(
            "https://casenote.kr/{court}/{code}",
            court = self.court.as_ref().unwrap(),
            code = self.case_code.as_ref().unwrap()
        )
    }

    fn get_guid(&self) -> String {
        self.id.to_string()
    }

    fn get_description(&self) -> String {
        self.description
            .as_ref()
            .unwrap_or(&String::new())
            .to_string()
    }

    fn get_category(&self) -> String {
        self.case_type.as_ref().unwrap().to_string()
    }

    fn get_pubdate(&self) -> String {
        let local_dt = NaiveDate::parse_from_str(self.decision_date.as_ref().unwrap(), "%Y.%m.%d")
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();

        Seoul.from_local_datetime(&local_dt).unwrap().to_rfc2822()
    }
}
