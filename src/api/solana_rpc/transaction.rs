use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, UiTransactionStatusMeta,
};

const Base_Mint_Index: i32 = 7;
const Quote_Mint_Index: u8 = 8;

pub fn find_mint_token(transaction: Option<UiTransactionStatusMeta>) {
    let mut transaction_conditions_met = true;
    if let Some(transaction) = transaction {
        if let OptionSerializer::Some(s) = transaction.clone().inner_instructions {
            for ele in s {
                //not sure if this is quite right but test anyway
                if ele.instructions.len() < 37 {
                    transaction_conditions_met = false;
                } else {
                    println!("over 37 transactions: success : {:?} ", ele);
                }
                for ele in ele.instructions {
                    match ele {
                        solana_transaction_status::UiInstruction::Compiled(instruction) => {
                            if instruction.accounts.len() < 10 {
                                /*                                 println!("false"); */
                                transaction_conditions_met = false;
                            } else {
                                println!("found correct accounts length");

                                println!("account: {:?}", instruction.accounts);

                                let result = safe_index(
                                    instruction.accounts[8],
                                    instruction.accounts.clone(),
                                );
                                if result {
                                    println!("{:?}", instruction.accounts[0]);
                                    println!("{:?}", instruction.accounts[2]);
                                    println!("{:?}", instruction.accounts[3]);
                                    println!("{:?}", instruction.accounts[4]);
                                    println!("{:?}", instruction.accounts[5]);
                                    println!("{:?}", instruction.accounts[6]);
                                    println!("{:?}", instruction.accounts[7]);
                                    println!("{:?}", instruction.accounts[8]);
                                    println!();
                                    println!("{:?}", transaction);
                                }
                            }
                            //START HERE

                            /*                             println!("account: {:?}", instruction.accounts); */
                            /*                             if (instruction.accounts[Base_Mint_Index] = MINT_INDEX) {} */
                        }
                        solana_transaction_status::UiInstruction::Parsed(instruction) => {
                            println!("{:?}", instruction)
                        }
                    }
                }
                if transaction_conditions_met {}
            }
        };
    }
}

fn safe_index(idx: u8, tx: Vec<u8>) -> bool {
    /* let serum_openbook =
        solana_sdk::bs58::decode("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX".as_bytes())
            .into_vec()
            .unwrap();

    let srm_pub_key = Pubkey::new(&serum_openbook); */
    println!(
        "instruction number 8 {:?}, account instructions length: {:?}",
        idx as usize,
        tx.len()
    );
    if idx as usize >= tx.len() {
        /*                 le srm_pub_key.to_bytes(); */
        false
    } else {
        let asda = tx[idx as usize];
        true
        /*         return tx.Message.AccountKeys[idx]; */
    }
}
