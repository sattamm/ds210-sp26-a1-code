extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::Query;
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    use analytics_lib::dataset::Value;
    use analytics_lib::query::{Aggregation, Condition, Query};

    fn parse_value(s: &str) -> Value {
        let s = s.trim();
        if s.starts_with('"') && s.ends_with('"') {
            Value::String(s[1..s.len() - 1].to_string())
        } else {
            Value::Integer(s.parse().unwrap())
        }
    }

    fn parse_condition(s: &str) -> Condition {
        let s = s.trim();

        if s.starts_with("!(") && s.ends_with(")") {
            let inside = &s[2..s.len() - 1];
            return Condition::Not(Box::new(parse_condition(inside)));
        }

        if s.starts_with("(") && s.ends_with(")") {
            let inside = &s[1..s.len() - 1];
            if inside.contains(" OR ") {
                let parts: Vec<&str> = inside.splitn(2, " OR ").collect();
                return Condition::Or(
                    Box::new(parse_condition(parts[0])),
                    Box::new(parse_condition(parts[1])),
                );
            }
            if inside.contains(" AND ") {
                let parts: Vec<&str> = inside.splitn(2, " AND ").collect();
                return Condition::And(
                    Box::new(parse_condition(parts[0])),
                    Box::new(parse_condition(parts[1])),
                );
            }
        }

        let parts: Vec<&str> = s.split(" == ").collect();
        Condition::Equal(parts[0].trim().to_string(), parse_value(parts[1]))
    }

    let parts: Vec<&str> = input.split(" GROUP BY ").collect();
    let filter_str = parts[0].trim_start_matches("FILTER ").trim();
    let rest: Vec<&str> = parts[1].split_whitespace().collect();

    let group_by = rest[0].to_string();
    let aggregate = match rest[1] {
        "COUNT" => Aggregation::Count(rest[2].to_string()),
        "SUM" => Aggregation::Sum(rest[2].to_string()),
        "AVERAGE" => Aggregation::Average(rest[2].to_string()),
        _ => panic!("bad aggregation"),
    };

    Query::new(parse_condition(filter_str), group_by, aggregate)
}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}