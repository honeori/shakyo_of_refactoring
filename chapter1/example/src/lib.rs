
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

pub fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result = format!("Statement for {}\n", invoice.customer);
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

    for perf in &invoice.performances {
        result.push_str(format!("{}: {} ({} seats)\n", play_for(perf).name, amount_for(perf) / 100, perf.audience).as_ref());
        // print line for this order
        total_amount += amount_for(perf);
    }

    for perf in &invoice.performances {
        volume_credits += volume_credits_for(perf);
    }


    result.push_str(format!("Amount owed is {}\n", total_amount / 100).as_ref());
    result.push_str(format!("You earned {} credits\n", volume_credits).as_ref());

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
