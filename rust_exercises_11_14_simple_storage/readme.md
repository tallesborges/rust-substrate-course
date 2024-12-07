// get
cargo contract call --contract 5EvuvCuT2RSk6gcNSMXCBowAQBUx6C6EUiMFQiQfcefM5KHv --message retrieve --suri //Alice --output-json

// set
cargo contract call --contract 5EvuvCuT2RSk6gcNSMXCBowAQBUx6C6EUiMFQiQfcefM5KHv --message store --args 2 --suri //Alice --skip-confirm --execute
