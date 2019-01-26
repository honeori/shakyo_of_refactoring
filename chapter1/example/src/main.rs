#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;


fn main() {
    let plays = plays();
    let invoice = invoice();
    println!("hamlet names is {}", plays["hamlet"].name);
    println!("the first invoice is {}", invoice.customer);

//    let invoices = invoice();
//    let invoice = &invoices[0];
    //statement(invoice, &plays);
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

fn statement(invoice: &Value, plays: &Value) {
    println!("hamlet names is {}", plays["hamlet"]["name"].as_str().unwrap());
    println!("the first invoice is {}", invoice["customer"].as_str().unwrap());
    let mut total_amount = 0;
    let mut volume_credits = 0;
    let result = format!("Statement for {}\n", invoice["customer"].as_str().unwrap());

    for perf in invoice["performances"].as_array().unwrap() {
        let playID = perf["playID"].as_str().unwrap();
        let play = &plays[playID];
        let thisAmount = 0;
    }
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

