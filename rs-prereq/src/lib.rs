#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::{self, signer::Signer};
    use solana_sdk::{
        hash::hash,
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, read_keypair_file},
        transaction::Transaction,
    };
    use solana_system_interface::instruction::transfer;
    use solana_system_interface::program as system_program;
    use std::str::FromStr;

    const RPC_URL: &str =
        "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}\n",
            kp.try_pubkey().unwrap().to_string()
        );
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn claim_airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        println!(
            "You've generated a new Solana wallet: {}\n",
            keypair.try_pubkey().unwrap().to_string()
        );
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);

        let sig_hash = hash(sig.as_ref());

        match sig.verify(&pubkey.to_bytes(), &sig_hash.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("DuNKWq3a93BysBvKS2xR9hcJCVfmT494F2RiFM7jpt8b").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        let balance = rpc_client
            .get_balance(&pubkey)
            .expect("Failed to get balance");

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&pubkey, &to_pubkey, balance)],
            Some(&pubkey),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&pubkey, &to_pubkey, balance - fee)],
            Some(&pubkey),
            &[&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        let signer = read_keypair_file("main-wallet.json").expect("Couldn't find wallet file");
        let rpc_client = RpcClient::new(RPC_URL);

        let mint = Keypair::new();
        let turbin3_prereq_program =
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program =
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();

        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

        let authority_seeds = &[b"collection", collection.as_ref()];
        let (authority, _) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new(prereq_pda, false),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
