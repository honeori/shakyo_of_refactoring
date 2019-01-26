
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use std::collections::HashMap;
use std::cmp::max;


#[derive(Serialize, Deserialize)]
pub struct Play {
    pub name: String,
    pub play_type: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Performance {
    pub playID: String,
    pub audience: u8
}

#[derive(Serialize, Deserialize)]
pub struct Invoice {
    pub customer: String,
    pub performances: Vec<Performance>,
}

//Performanceとメンバの共通部分があるがよい具合に共通化仕方がうかばない
#[allow(non_snake_case)]
struct StatementPerformance<'a> {
    playID: &'a String,
    audience: u8,
    play: &'a Play,
    amount: u32,
    volume_credits: u8,
}

struct StatementData<'a> {
    customer: &'a String,
    performances: &'a Vec<StatementPerformance<'a>>,
    total_volume_credits: u8,
}

pub fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    let play_for = |a_performance: &Performance| {
        plays.get(&a_performance.playID).unwrap()
    };

    let amount_for = |a_performance: &Performance| {
        let mut result;
        match play_for(a_performance).play_type.as_ref() {
            "tragedy" => {
                result = 40000;
                if a_performance.audience > 30 {
                    result += 1000 * (a_performance.audience as u32 - 30);
                }
            },
            "comedy" => {
                result = 30000;
                if a_performance.audience > 20 {
                    result += 10000 + 500 * (a_performance.audience as u32 - 20);
                }
                result += 300 * a_performance.audience as u32;
            },
            _ => panic!("unknown play_type: {}", play_for(a_performance).play_type)
        };
        result
    };

    let volume_credits_for = |a_performance: &Performance| {
        let mut result = 0;
        result += max(a_performance.audience - 30, 0);
        // add extra credit for every ten comedy attendees
        match play_for(a_performance).play_type.as_ref() {
            "comedy" => {
                result += (a_performance.audience as f64 / 5.0).floor() as u8;
            },
            _ => (),
        }
        result
    };

    fn total_volume_credits(performances: &Vec<StatementPerformance>) -> u8 {
        let mut result = 0;
        for perf in performances {
            result += perf.volume_credits;
        }
        result
    }

    let statement_performances = invoice.performances.iter().map(|performance| {
        StatementPerformance {
            playID: &performance.playID,
            audience: performance.audience,
            play: play_for(performance),
            amount: amount_for(performance),
            volume_credits: volume_credits_for(performance),
        }
    }).collect();

    let statement_data = StatementData{
        customer: &invoice.customer,
        performances: &statement_performances,
        total_volume_credits: total_volume_credits(&statement_performances),
    };
    render_plain_text(&statement_data)
}

fn render_plain_text(data: &StatementData) -> String {
    let total_amount = || {
        let mut result = 0;
        for perf in data.performances {
            result += perf.amount;
        }
        result
    };

    let mut result = format!("Statement for {}\n", data.customer);
    for perf in data.performances {
        result.push_str(format!("{}: {} ({} seats)\n", perf.play.name, perf.amount / 100, perf.audience).as_ref());
    }
    result.push_str(format!("Amount owed is {}\n", total_amount() / 100).as_ref());
    result.push_str(format!("You earned {} credits\n", data.total_volume_credits).as_ref());

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
