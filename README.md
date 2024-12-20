An asynchronous Rust binary listening to the Solana RPC node for freshly minted tokens added to Raydium's liquidity pools. Concurrently checks these tokens against Dexscreener and Rugcheck's apis for viable tokenomics. If found a purchase is made by the user's Solana private key wallet

Run instructions:

``PRIV_KEY``: User's Solana wallet private key
``PUB_KEY``: User's Solana wallet public key e.g. ``qwCrxRSVHcos8aiQ4BjULJDyh2KqSdudUde4tdFQupv``

`` 
cargo run -- --priv-key [PRIV_KEY] --pub-key [PUB_KEY]

``
