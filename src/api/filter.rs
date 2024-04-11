use serde_json::Value;
use serenity::all::Message;

use crate::repositories::models::Token;

pub async fn filter_msg(msg: Message) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let value = serde_json::to_value(msg).unwrap();

    let arr = value["embeds"][0].get("fields").unwrap();

    if let Value::Array(j) = arr {
        for ele in j {
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
                            tokens.push(Token::Id(words.get(4).unwrap().replace(['.', '"'], "")));
                        } else {
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

                            tokens.push(Token::Name(string));
                        }
                    }
                }
            }
        }
    }
    tokens
}
