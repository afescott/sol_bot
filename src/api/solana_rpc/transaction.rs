use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, UiTransactionStatusMeta,
};

pub fn find_mint_token(transaction: Option<UiTransactionStatusMeta>) {
    let mut transaction_conditions_met = true;
    if let Some(transaction) = transaction {
        if let OptionSerializer::Some(s) = transaction.inner_instructions {
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
                                println!("{:?}", instruction);
                                println!("found correct accounts length");
                            }
                        }
                        solana_transaction_status::UiInstruction::Parsed(_) => todo!(),
                    }
                }
            }
        };

        /* if let OptionSerializer::Some(s) = transaction.return_data {}

        if let OptionSerializer::Some(s) = transaction.inner_instructions {} */
    }
}
