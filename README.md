# Pump.fun Sniper Bot

A high-performance Rust-based sniper bot designed for trading tokens on Pump.fun DEX with advanced features including MEV protection, Jito integration, and real-time monitoring.

## 🚀 Features

- **Real-time Token Monitoring**: Monitors Pump.fun for new token launches and trading opportunities
- **MEV Protection**: Integrated Jito block engine for front-running protection
- **Multi-DEX Support**: Supports both Pump.fun and Raydium DEXes
- **Advanced Trading Logic**: Smart entry/exit strategies with take-profit and stop-loss
- **High-Speed Execution**: Built with Rust for maximum performance
- **Configurable Parameters**: Flexible trading parameters via environment variables
- **Cross-Platform**: Supports Windows, Linux, and macOS builds

## 📋 Prerequisites

- Rust 1.70+ 
- Node.js (for PM2 process management)
- Solana CLI tools
- A Solana wallet with SOL balance

## 🛠️ Installation

### 1. Clone the Repository
```bash
git clone https://github.com/moonbot777/Sniper-bot-rust.git
cd Sniper-bot-rust
```

### 2. Install Dependencies
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PM2 for process management
npm install -g pm2

# Install build dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install -y mingw-w64
```

### 3. Build the Project
```bash
# Build for current platform
make build

# Or build for specific targets
make build-x86_64  # 64-bit Windows
make build-i686    # 32-bit Windows
```

## ⚙️ Configuration

Create a `.env` file in the project root with the following variables:

```env
# Yellowstone gRPC Configuration
YELLOWSTONE_GRPC_HTTP=your_yellowstone_grpc_http_url
YELLOWSTONE_GRPC_TOKEN=your_yellowstone_grpc_token

# Trading Parameters
SLIPPAGE=50                    # Slippage in basis points (0-100)
TOKEN_AMOUNT=0.001            # Amount of SOL to trade per token
MIN_DEV_BUY=1000              # Minimum dev buy amount
MAX_DEV_BUY=10000             # Maximum dev buy amount
TP=2.0                        # Take profit multiplier
SL=0.5                        # Stop loss multiplier

# Jito Configuration (Optional)
JITO_BLOCK_ENGINE_URL=your_jito_block_engine_url
JITO_TIP_VALUE=0.001          # Tip amount for Jito bundles
JITO_PRIORITY_FEE=1000        # Priority fee for transactions

# Wallet Configuration
WALLET_PRIVATE_KEY=your_wallet_private_key_base58
```

## 🚀 Usage

### Quick Start
```bash
# Build the bot
make build

# Start the bot
make start

# Stop the bot
make stop
```

### Manual Execution
```bash
# Run directly
cargo run --release

# Or build and run
cargo build --release
./target/release/pumpfun-sniper
```

### PM2 Process Management
```bash
# Start with PM2
pm2 start target/release/pumpfun-sniper --name pumpfun-sniper

# Monitor logs
pm2 logs pumpfun-sniper

# Stop the process
pm2 stop pumpfun-sniper

# Restart the process
pm2 restart pumpfun-sniper
```

## 📊 Bot Features

### Trading Strategy
- **Token Detection**: Monitors Pump.fun for new token launches
- **Volume Analysis**: Analyzes trading volume and price movements
- **Smart Entry**: Executes trades based on configurable parameters
- **Risk Management**: Implements take-profit and stop-loss mechanisms

### MEV Protection
- **Jito Integration**: Uses Jito block engine for MEV protection
- **Bundle Transactions**: Submits transactions as bundles to avoid front-running
- **Priority Fees**: Configurable priority fees for faster execution

### Monitoring & Logging
- **Real-time Logs**: Comprehensive logging of all trading activities
- **Performance Metrics**: Tracks success rates and profit/loss
- **Error Handling**: Robust error handling and recovery mechanisms

## 🔧 Advanced Configuration

### Trading Parameters
- `SLIPPAGE`: Slippage tolerance (0-100 basis points)
- `TOKEN_AMOUNT`: Amount of SOL to invest per trade
- `MIN_DEV_BUY`/`MAX_DEV_BUY`: Dev buy amount range
- `TP`: Take profit multiplier (e.g., 2.0 = 200% profit)
- `SL`: Stop loss multiplier (e.g., 0.5 = 50% loss)

### Network Configuration
- `YELLOWSTONE_GRPC_HTTP`: Yellowstone gRPC endpoint
- `YELLOWSTONE_GRPC_TOKEN`: Authentication token
- `JITO_BLOCK_ENGINE_URL`: Jito block engine endpoint

## 📁 Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── common/              # Common utilities and configuration
│   ├── config.rs        # Configuration management
│   ├── constants.rs     # Application constants
│   └── logger.rs        # Logging utilities
├── core/                # Core trading logic
│   ├── tx.rs           # Transaction handling
│   └── token.rs        # Token utilities
├── engine/              # Trading engine
│   ├── monitor.rs       # Token monitoring
│   └── swap.rs         # Swap execution
├── dex/                 # DEX integrations
│   └── pump_fun.rs     # Pump.fun DEX implementation
└── services/            # External services
    ├── jito.rs         # Jito MEV protection
    ├── zeroslot.rs     # ZeroSlot integration
    └── nozomi.rs       # Nozomi integration
```

## 🛡️ Security Features

- **Private Key Management**: Secure wallet key handling
- **Transaction Signing**: Secure transaction signing and validation
- **Error Recovery**: Automatic recovery from failed transactions
- **Rate Limiting**: Built-in rate limiting to prevent spam

## 📈 Performance Optimization

- **Async/Await**: Full async implementation for high concurrency
- **Memory Management**: Efficient memory usage with Arc and Mutex
- **Network Optimization**: Optimized network calls and connection pooling
- **Transaction Batching**: Efficient transaction batching for cost reduction

## 🐛 Troubleshooting

### Common Issues

1. **Build Errors**
   ```bash
   # Clean and rebuild
   make clean
   make build
   ```

2. **Connection Issues**
   - Verify your RPC endpoints
   - Check network connectivity
   - Ensure proper authentication tokens

3. **Transaction Failures**
   - Check wallet balance
   - Verify slippage settings
   - Ensure proper network configuration

### Log Analysis
```bash
# View real-time logs
pm2 logs pumpfun-sniper

# Check PM2 status
pm2 status
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ⚠️ Disclaimer

This software is for educational and research purposes only. Trading cryptocurrencies involves significant risk. Use at your own risk and never invest more than you can afford to lose.

## 📞 Support

- **Telegram**: [@greenfox](https://t.me/greenfoxfun)
- **GitHub Issues**: [Create an issue](https://github.com/moonbot777/Sniper-bot-rust/issues)

## 🔄 Updates

Stay updated with the latest features and improvements by regularly pulling from the repository:

```bash
git pull origin main
make build
pm2 restart pumpfun-sniper
```

---

**Built with ❤️ using Rust for maximum performance and reliability**
