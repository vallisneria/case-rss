use crate::law_api::{CourtPrecedent, CourtType, DecisionType};
use quick_xml::events::Event;
use quick_xml::Reader;

pub fn parse_court_precedent_list(xml_txt: &String) -> Vec<CourtPrecedent> {
    let mut result: Vec<CourtPrecedent> = Vec::new();
    let mut reader = Reader::from_str(xml_txt);
    reader.trim_text(true);
    let mut buf: Vec<u8> = Vec::new();
    let mut txt: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                panic!("Error at position {}: {:?}", reader.buffer_position(), e);
            }

            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"prec" => result.push(CourtPrecedent::new()),
                _ => (),
            },

            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),

            Ok(Event::CData(e)) => txt.push(e.escape().unwrap().unescape().unwrap().into_owned()),

            Ok(Event::End(e)) => match std::str::from_utf8(e.name().as_ref()).unwrap() {
                "판례일련번호" => {
                    let id = txt.last().unwrap().parse().unwrap();
                    result.last_mut().unwrap().id = id;
                }

                "사건명" => {
                    let case_name = txt.last().unwrap().to_string();
                    result.last_mut().unwrap().case_name = Some(case_name);
                }

                "사건번호" => {
                    let case_code = txt.last().unwrap().to_string();
                    result.last_mut().unwrap().case_code = Some(case_code);
                }

                "선고일자" => {
                    let decision_date = txt.last().unwrap().to_string();
                    result.last_mut().unwrap().decision_date = Some(decision_date);
                }

                "법원명" => {
                    let court: CourtType = CourtType::new(txt.last().unwrap().to_string());
                    result.last_mut().unwrap().court = Some(court)
                }

                "사건종류명" => {
                    let case_type = txt.last().unwrap().to_string();
                    result.last_mut().unwrap().case_type = Some(case_type);
                }

                "판결유형" => {
                    let decision = DecisionType::new(txt.last().unwrap().to_string());
                    result.last_mut().unwrap().decision_type = Some(decision);
                }

                "prec" => txt.clear(),

                _ => (),
            },

            _ => (),
        }

        buf.clear();
    }

    result
}

pub fn parse_court_description(xml_txt: &String) -> String {
    let mut reader = Reader::from_str(xml_txt);
    reader.trim_text(true);
    let mut buf: Vec<u8> = Vec::new();
    let mut txt: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),

            Ok(Event::CData(e)) => {
                let data = e.escape().unwrap().unescape().unwrap().into_owned();
                txt.push(data);
            }

            Ok(Event::End(e)) => match std::str::from_utf8(e.name().as_ref()).unwrap() {
                "판시사항" => break txt.last().unwrap().to_string(),

                _ => (),
            },
            _ => (),
        }
    }
}
