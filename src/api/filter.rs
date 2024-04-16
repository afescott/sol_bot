use std::collections::HashMap;

use chrono::{DateTime, Duration, TimeDelta, Utc};
use reqwest::{header, Body, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty, Value};
use serenity::all::Message;
use url::Url;

use crate::{
    api::helius,
    format_addresses,
    repositories::models::{EnhancedTransaction, Pairs, TokenType},
};

pub async fn filter_msg(msg: Message, client: &Client) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let value = serde_json::to_value(msg).unwrap();

    let arr = value["embeds"][0].get("fields").unwrap();

    if let Value::Array(j) = arr {
        let mut found = false;
        for ele in j {
            if found {
                get_transaction_info(ele.clone(), client).await;
                found = false;
            }
            if ele["value"].to_string().contains("minted") {
                let re = regex::Regex::new(r"minted").unwrap();
                for part in re.split(&ele["value"].to_string()) {
                    if !part.contains("tokens") && part.contains('.') {
                        let part = part.replace('.', "");
                        if part.contains("token") {
                            /*                             println!("{:?}", part); */
                            let words = &part
                                .split(' ')
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>();
                            tokens
                                .push(TokenType::Id(words.get(4).unwrap().replace(['.', '"'], "")));
                        } else {
                            println!("{:?}", ele);
                            let mut count = 0;
                            let mut string = "".to_string();
                            for ele in part.split(' ') {
                                let part = ele.replace(['.', '"'], "");
                                count += 1;

                                /*                                 println!("{:?}", part); */
                                if count > 2 {
                                    string += &part;
                                    string += " ";
                                    //if counter greater than word length
                                    /* if count != part.len() - 1 {
                                        string += " ";
                                    } */
                                }
                            }
                            string.remove(string.len() - 1).to_string();
                            println!("{:?}", string);
                            tokens.push(TokenType::Name(string));
                        }
                    }
                }
                if tokens.is_empty() {
                    found = true
                }
            }
        }
    }
    println!("{:?}", tokens.len());
    tokens
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
struct Req<T> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: T,
}

impl<T> Req<T> {
    pub fn new(method: &str, params: T) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: "1".to_string(),
            method: String::from(method),
            params,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseTransactionsRequest {
    pub transactions: Vec<String>,
}

async fn get_transaction_info(value: Value, client: &Client) {
    let r = value["value"]
        .to_string()
        .split("/tx/")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut asfa = r.get(1).unwrap().to_string();

    asfa.remove(asfa.len() - 1);

    let afsafg = vec![asfa];
    let transaction = ParseTransactionsRequest {
        transactions: afsafg.clone(),
    };

    println!("{:?}", transaction);

    /*     let req = Req::new("transactions", transactio); */
    /* helius::Helius::new(
        "c23e1823-5a4d-4739-9e8d-4b4936a3a3d5".to_string(),
        client.clone(),
    ); */

    /*
        working postman "code equiv"
        * curl --location 'https://api.helius.xyz/v0/transactions?api-key=c23e1823-5a4d-4739-9e8d-4b4936a3a3d5' \
    --header 'Content-Type: application/json' \
    --data '{ "transactions": ["4HyRPYm6wkjEjhoYKv9VyVzuZ7bPSyKkkqFjyJzQAM91UVjT8kFaQ2EF4KTgjD3U8kRvPaDeRHNhBW5s9qzVgrfD"] }'

        */

    let asfaf = Url::parse(
        "https://api.helius.xyz/v0/transactions?api-key=c23e1823-5a4d-4739-9e8d-4b4936a3a3d5",
    );

    println!("{:?}", asfaf);

    let transactions = vec![
        "4HyRPYm6wkjEjhoYKv9VyVzuZ7bPSyKkkqFjyJzQAM91UVjT8kFaQ2EF4KTgjD3U8kRvPaDeRHNhBW5s9qzVgrfD",
    ];
    let body = json!({ "transactions": transactions });

    let client = reqwest::Client::new();
    let response = client
            .post(asfaf.unwrap())
/*  .header("accept", "application/json") */
            .header("Content-Type", "application/json")
        .json(&body)
/*             .body("iUue99z8rTsdcKBCHzqzFc3q95LcaV3LkHKz2MaSrBd4d8cqLCxi3nTqTtdksqZNgRKLSZz9kaZooznp13Yxp5x") */
            .send()
            .await.unwrap();

    // Handle response
    let status = response.status();
    if status.is_success() {
        println!("Success! Response: {:?}", response.text().await);
    } else {
        println!("Error: {}", status);
        let error_text = response.text().await.unwrap();
        println!("Error details: {}", error_text);
    }
    /* let asfa = asfa.json::<EnhancedTransaction>().await.unwrap();

    /*     let asfa = serde_json::from_str::<EnhancedTransaction>(&asfa.text().await.unwrap()); */

    println!("{:?}", asfa); */
}

pub async fn pairs_filter(pairs: Pairs) {
    for ele in pairs.pairs {
        if let Some(time_created_pair) = ele.pair_created_at {
            /*             let time = chrono::DateTime::from_timestamp_millis(1619248666000); */

            /*             let time = chrono::DateTime::from_timestamp_millis(time.try_into().unwrap()); */
            println!("Created date: {:?}", time_created_pair);
            let time = TimeDelta::new(300, 0);
            let five_mins = Duration::num_milliseconds(&time.unwrap());

            /*             println!("{:?}", five_mins); */
            let r = five_mins + Utc::now().timestamp_millis();
            // pair created : 500,    now: 500 + 5.   pair created - now + 5
            println!("Now: {:?}", Utc::now().timestamp_millis());
            /*             println!("now + 5 mins: {:?}", r); */

            let time_deducted = r - i64::try_from(time_created_pair).unwrap();
            /*             println!("Now minus created date: {:?}", time_deducted); */

            if time_deducted < five_mins {
                println!("OK");
            } else {
                /*                 println!("Not ok"); */
            }
        }
    }
}
