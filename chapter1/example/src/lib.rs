
#[macro_use]
extern crate serde_derive;

extern crate serde;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::cmp::max;


#[derive(Serialize, Deserialize)]
pub struct Play {
    pub name: String,
    pub play_type: String,
}

#[derive(Serialize, Deserialize)]
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(1, 1)
    }
}
