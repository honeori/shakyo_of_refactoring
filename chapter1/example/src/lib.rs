
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod create_statement_data;

use self::create_statement_data::*;
use std::collections::HashMap;

pub use create_statement_data::{Play, Invoice};

pub fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    render_plain_text(&create_statement_date(invoice, plays))
}

pub fn html_statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    render_html_text(&create_statement_date(invoice, plays))
}

fn render_plain_text(data: &StatementData) -> String {
    let mut result = format!("Statement for {}\n", data.customer);
    for perf in &data.performances {
        result.push_str(format!("{}: {} ({} seats)\n", perf.play.name, perf.amount / 100, perf.audience).as_ref());
    }
    result.push_str(format!("Amount owed is {}\n", data.total_amount / 100).as_ref());
    result.push_str(format!("You earned {} credits\n", data.total_volume_credits).as_ref());

    result
}

fn render_html_text(data: &StatementData) -> String {
    let mut result = format!("<h1>Statement for {}</h1>\n", data.customer);
    result.push_str(format!("<table>\n").as_ref());
    result.push_str(format!("<tr><th>play</th><th>seats</th><th>cost</th></tr>\n").as_ref());
    for perf in &data.performances {
        result.push_str(format!("<tr><td>{}</td><td>{}</td><td>{} seats</td></tr>\n", perf.play.name, perf.audience, perf.amount / 100).as_ref());
    }
    result.push_str(format!("</table>\n").as_ref());
    result.push_str(format!("<p>Amount owed is <em>{}</em></p>\n", data.total_amount / 100).as_ref());
    result.push_str(format!("<p>You earned <em>{}</em> credits</p>\n", data.total_volume_credits).as_ref());

    result
}


#[allow(dead_code)] // use for test just test only
fn plays() -> HashMap<String, Play> {
    let data = r#"
{
    "hamlet": {"name": "Hamlet", "play_type": "tragedy"},
    "as-like": {"name": "As You Like It", "play_type": "comedy"},
    "othello": {"name": "Othello", "play_type": "tragedy"}
}
    "#;
    let v: HashMap<String, Play> = serde_json::from_str(data).unwrap();

    v
}


#[allow(dead_code)] // use for test just test only
fn invoice() -> Invoice {
    let data = r#"
    {
        "customer": "BigCo",
        "performances": [
            {
                "playID": "hamlet",
                "audience": 55
            },
            {
                "playID": "as-like",
                "audience": 35
            },
            {
                "playID": "othello",
                "audience": 40
            }
        ]
    }
    "#;
    let v: Invoice = serde_json::from_str(data).unwrap();
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_statement() {
        let plays = plays();
        let invoice = invoice();
        let result = statement(&invoice, &plays);
        assert_eq!(result, r#"Statement for BigCo
Hamlet: 650 (55 seats)
As You Like It: 580 (35 seats)
Othello: 500 (40 seats)
Amount owed is 1730
You earned 47 credits
"#
        );
    }
}
