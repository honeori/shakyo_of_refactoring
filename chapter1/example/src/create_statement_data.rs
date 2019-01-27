
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
pub struct StatementPerformance<'a> {
    pub audience: u8,
    pub play: &'a Play,
    pub amount: u32,
    pub volume_credits: u8,
}

pub struct StatementData<'a> {
    pub customer: &'a String,
    pub performances: Vec<StatementPerformance<'a>>,
    pub total_volume_credits: u8,
    pub total_amount: u32,
}

pub fn create_statement_date<'a>(invoice: &'a Invoice, plays: &'a HashMap<String, Play>) -> StatementData<'a> {
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

    fn total_amount(performances: &Vec<StatementPerformance>) -> u32 {
        let mut result = 0;
        for perf in performances {
            result += perf.amount;
        }
        result
    };

    let statement_performances = invoice.performances.iter().map(|performance| {
        StatementPerformance {
            audience: performance.audience,
            play: play_for(performance),
            amount: amount_for(performance),
            volume_credits: volume_credits_for(performance),
        }
    }).collect();

    let statement_data = StatementData{
        customer: &invoice.customer,
        total_volume_credits: total_volume_credits(&statement_performances),
        total_amount: total_amount(&statement_performances),
        performances: statement_performances,
    };
    statement_data
}
