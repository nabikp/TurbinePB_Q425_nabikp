#  SPL Vault on Solana (Devnet)

project demonstrates creating a **Vault** on the **Solana Devnet** to securely lock SPL Tokens

1. created spl token mint using spl-token create-token
2. mint address: BQtupkxZLXxbxNuf1N2LT9cbVBBnErjKQV3GzDWDrsj
3. then created a vault token account linked to the mint address using spl-token create-account BQtupkxZLXxbxNuf1N2LT9cbVBBnErjKQV3GzDWDrsj --owner vault-keypair.json
4. vault token account : DXhNkoNB9AwtDeenabWyhr6KrbVLHxMKpECsZU64CxZw
5. signature of vault creation: 5caDNPnPExDWPu9JZK4AjHGsTnNerCAwWGcRvc1J3VW2XJESgQEPCYHJ7fdCLGhkUK1s6zdS5sMAkaC5xTQAd3Yi
6. locked token into the vaults by tranfering : spl-token transfer BQtupkxZLXxbxNuf1N2LT9cbVBBnErjKQV3GzDWDrsj 10 DXhNkoNB9AwtDeenabWyhr6KrbVLHxMKpECsZU64CxZw --fund-recipient --allow-unfunded-recipient



ans:  lock transaction hash: 7vD4yrXkq9F8JHvRxwJ5n7XHbM5JtMkrfWb7JhFrfZkT7nM3qZUpgJHZbE3ThxYqFx1Rk4s8DxL4fWLM2LBjUHTX



