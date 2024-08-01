use std::{collections::HashMap, thread};

use chrono::{DateTime, Duration, TimeDelta, Utc};
use reqwest::{header, Body, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty, Value};
use serenity::all::Message;
use tracing::Level;
use url::Url;

use crate::{
    api::helius,
    format_addresses,
    repositories::{
        mem::StorageRepository,
        models::{EnhancedTransaction, Pairs, TokenType},
    },
};

pub struct DiscordParser {}

impl DiscordParser {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn filter_msg(
        &self,
        msg: Message,
        client: &Client,
        state: &Vec<EnhancedTransaction>,
    ) -> crate::error::Result<Vec<TokenType>> {
        let mut tokens: Vec<TokenType> = Vec::new();
        let mut values: Vec<Value> = Vec::new();

        let value = serde_json::to_value(msg).unwrap();

        let arr = value["embeds"][0].get("fields").unwrap();

        if let Value::Array(j) = arr {
            let mut found = false;
            for ele in j {
                if found {
                    values.push(ele.clone());

                    let enhanced_transactions = self.get_transaction_info(client, &values).await?;
                    tokens.push(TokenType::Pairs(enhanced_transactions));
                    found = false;
                }
                if ele["value"].to_string().contains("minted") {
                    let re = regex::Regex::new(r"minted").unwrap();
                    for part in re.split(&ele["value"].to_string()) {
                        if !part.contains("tokens") && part.contains('.') {
                            let part = part.replace('.', "");
                            if part.contains("token") {
                                let words = &part
                                    .split(' ')
                                    .map(|s| s.to_string())
                                    .collect::<Vec<String>>();
                                tokens.push(TokenType::Id(
                                    words.get(4).unwrap().replace(['.', '"'], ""),
                                ));
                            } else {
                                let mut count = 0;
                                let mut string = "".to_string();
                                for ele in part.split(' ') {
                                    let part = ele.replace(['.', '"'], "");
                                    count += 1;
                                    if count > 2 {
                                        string += &part;
                                        string += " ";
                                    }
                                }
                                string.remove(string.len() - 1).to_string();
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
        Ok(tokens)
    }

    pub async fn get_transaction_info(
        &self,
        client: &Client,
        values: &Vec<Value>,
    ) -> crate::error::Result<Vec<EnhancedTransaction>> {
        let span = tracing::span!(Level::INFO, "main");
        let guard = span.enter();

        let mut vec: Vec<String> = Vec::new();
        let mut stack: Vec<EnhancedTransaction> = Vec::new();

        for ele in values {
            let mut r = ele["value"].to_string();
            r.remove(r.len() - 1);

            // println!("{:?}", r);
            let jim = r
                .split("/tx/")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            let token_addr = jim.get(1).unwrap().to_string();

            tracing::info!(parsed = ?token_addr, "token_addr");

            vec.push(token_addr.to_string());

            let asfaf = Url::parse(
            "https://api.helius.xyz/v0/transactions?api-key=c23e1823-5a4d-4739-9e8d-4b4936a3a3d5",
        );

            let json_body = serde_json::to_value(&vec).unwrap();

            let body = json!({ "transactions": json_body });

            let client = reqwest::Client::new();
            let response = client
                .post(asfaf.unwrap())
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await?;

            // Handle response
            let status = response.status();
            if status.is_success() {
                println!("Success! Response: {:?}", status);

                stack = response.json::<Vec<EnhancedTransaction>>().await.unwrap();
                println!("{:?}", stack.len());
            } else {
                println!("Error: {}", status);
                let error_text = response.text().await?;
                println!("Error details: {}", error_text);
            }
        }
        println!("Enhanced transaction done");
        Ok(stack)
    }
}
