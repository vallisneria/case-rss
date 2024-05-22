pub mod scourt;

use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug)]
pub struct CourtPrecedent {
    /// 일련번호
    pub id: u32,

    /// 사건명
    pub title: String,

    /// 사건번호
    /// 예: 2024다12345, 2018도12345
    pub case_id: String,

    /// 법원명
    pub court_type: CourtType,

    /// 사건종류
    pub case_type: CaseType,

    /// 선고일자
    pub decision_date: String,

    /// 판례공보 발행 일자
    pub pub_date: String,

    /// 판시사항
    pub judge_abstract: String,

    /// 판결요지
    pub judge_note: String,

    /// 판결종류
    pub decision_type: DecisionType,

    /// 판사
    pub judges: Vec<Judge>,
}

impl CourtPrecedent {
    pub fn new(overview: CourtPrecedentOverview, detail: CourtPrecedentDetail) -> Self {
        Self {
            id: overview.id,
            title: overview.title,
            case_id: overview.case_id,
            court_type: overview.court_type,
            case_type: overview.case_type,
            decision_date: overview.decision_date,
            pub_date: detail.pub_date,
            judge_abstract: detail.judge_abstract,
            judge_note: detail.judge_note,
            decision_type: detail.decision_type,
            judges: detail.judges,
        }
    }

    pub fn full_case_id(&self) -> String {
        let (ja, case_type) = match self.decision_type {
            DecisionType::Enbank => ("선고", "전원합의체 판결"),
            DecisionType::EnbankDecision => ("자", "전원합의체 결정"),
            DecisionType::Judgement => ("선고", "판결"),
            DecisionType::Decision => ("자", "결정"),
            _ => ("", ""),
        };

        format!(
            "{court_name} {date} {ja} {code} {case_type}",
            court_name = self.court_type,
            date = self.decision_date,
            code = self.case_id
        )
    }
}

#[derive(Debug, Deserialize)]
struct CourtPrecedentOverview {
    /// 일련번호
    #[serde(rename = "SEQ_NO")]
    id: u32,

    /// 제목
    #[serde(rename = "TITLE")]
    title: String,

    /// 사건번호
    #[serde(rename = "CASE_NUM")]
    case_id: String,

    /// 법원명
    #[serde(rename = "LAW_DESC")]
    court_type: CourtType,

    #[serde(rename = "TRANS_CASE_CLASS_DESC")]
    case_type: CaseType,

    #[serde(rename = "DECI_DATE")]
    decision_date: String,

    #[serde(rename = "TEXT_FILE_LIST")]
    detail_file: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CourtPrecedentDetail {
    /// 판결공보 수록일자
    pub_date: String,

    /// 판시사항
    judge_abstract: String,

    /// 판결요지
    judge_note: String,

    /// 판결종류
    decision_type: DecisionType,

    /// 판사
    judges: Vec<Judge>,
}

#[derive(Debug, Deserialize)]
pub enum CourtType {
    /// 대법원
    #[serde(rename = "대법원")]
    SupremeCourt,

    /// 각급법원
    Court(String),
}

impl Display for CourtType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupremeCourt => write!(f, "대법원"),
            Self::Court(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub enum CaseType {
    #[serde(rename = "민사")]
    Civil,

    #[serde(rename = "형사")]
    Criminal,

    #[serde(rename = "일반행정")]
    Administration,

    #[serde(rename = "조세")]
    Tax,

    #[serde(rename = "가사")]
    Family,

    #[serde(rename = "특허")]
    Patent,

    #[default]
    Unknown,
}

impl Display for CaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Civil => "민사",
            Self::Criminal => "형사",
            Self::Family => "가사",
            Self::Administration => "일반행정",
            Self::Tax => "조세",
            Self::Patent => "특허",
            Self::Unknown => "",
        };

        write!(f, "{}", name)
    }
}

#[derive(Debug, Deserialize, Default)]
pub enum DecisionType {
    /// 전원합의체 판결
    #[serde(rename = "전원합의체 판결")]
    Enbank,

    /// 전원합의체 결정
    #[serde(rename = "전원합의체 결정")]
    EnbankDecision,

    /// 판결
    #[serde(rename = "판결")]
    Judgement,

    /// 결정
    #[serde(rename = "결정")]
    Decision,

    #[default]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct Judge {
    pub role: String,
    pub name: String,
    pub position: String,
}
