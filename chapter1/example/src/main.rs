#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;
use std::cmp::max;


fn main() {
    let plays = plays();
    let invoice = invoice();
    let result = statement(&invoice, &plays);
    println!("{}", result);
}

#[derive(Serialize, Deserialize)]
struct Play {
    name: String,
    play_type: String,
}

#[derive(Serialize, Deserialize)]
struct Performance {
    playID: String,
    audience: u8
}

#[derive(Serialize, Deserialize)]
struct Invoice {
    customer: String,
    performances: Vec<Performance>,
}

fn statement(invoice: &Invoice, plays: &HashMap<String, Play>) -> String {
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let mut result = format!("Statement for {}\n", invoice.customer);

    for perf in &invoice.performances {
        let play = plays.get(&perf.playID).unwrap();
        let mut this_amount = 0;
        match play.play_type.as_ref() {
            "tragedy" => {
                this_amount = 40000;
                if perf.audience > 30 {
                    let temp = perf.audience as u32;
                    this_amount += 1000 * (temp - 30);
                }
            },
            "comedy" => {
                this_amount = 30000;
                let temp = perf.audience as u32;
                if perf.audience > 20 {
                    this_amount += 10000 + 500 * (temp - 20);
                }
                this_amount += 300 * temp;
            },
            _ => panic!("unknown play_type: {}", play.play_type)
        };
        // add volume credits
        volume_credits += max(perf.audience - 30, 0);
        // add extra credit for every ten comedy attendees
        match play.play_type.as_ref() {
            "comedy" => {
                let temp = perf.audience as f64;
                volume_credits += (temp / 5.0).floor() as u8;
            },
            _ => (),
        }

        result.push_str(format!("{}: {} ({} seats)\n", play.name, this_amount / 100, perf.audience).as_ref());
        // print line for this order

    }

    result.push_str(format!("Amount owed is {}\n", total_amount / 100).as_ref());
    result.push_str(format!("You earned {} credits\n", volume_credits).as_ref());

    result
}

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

