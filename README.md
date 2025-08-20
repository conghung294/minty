# Bonding Curve Token Launch Program

Một Solana program được xây dựng bằng Pinocchio framework để tạo và quản lý token với bonding curve pricing mechanism.

## Tính năng

- **Tạo Token với Bonding Curve**: Khởi tạo token mới với pricing curve tùy chỉnh
- **Mua Token**: Mua token từ bonding curve với giá được tính toán động
- **Bán Token**: Bán token về bonding curve và nhận SOL
- **Nhiều loại Curve**: Hỗ trợ Linear, Exponential, và Logarithmic pricing curves
- **Fee System**: Hệ thống phí linh hoạt với fee recipient
- **Slippage Protection**: Bảo vệ người dùng khỏi slippage quá lớn

## Cấu trúc Program

### Instructions

1. **Initialize** (Discriminator: 0)
   - Khởi tạo bonding curve mới với token mint
   - Thiết lập pricing parameters và fee structure

2. **BuyTokens** (Discriminator: 1)
   - Mua token từ bonding curve
   - Tự động tính giá dựa trên curve và supply hiện tại

3. **SellTokens** (Discriminator: 2)
   - Bán token về bonding curve
   - Burn token và trả SOL cho seller

### State

- **BondingCurve**: Account chứa thông tin curve state
  - Authority, token mint, reserves
  - Curve parameters (type, base price, slope)
  - Fee configuration

## Pricing Curves

### Linear Curve (Type 0)
```
price = base_price + (slope * supply / 1e9)
```

### Exponential Curve (Type 1)
```
price = base_price * (1 + slope/1e9)^supply
```

### Logarithmic Curve (Type 2)
```
price = base_price * log(1 + slope * supply / 1000)
```

## Cách sử dụng

### 1. Build Program

```bash
cargo build-sbf
```

### 2. Deploy Program

```bash
solana program deploy target/deploy/x_token.so
```

### 3. Initialize Bonding Curve

```typescript
const initializeIx = {
  programId: PROGRAM_ID,
  keys: [
    { pubkey: authority, isSigner: true, isWritable: false },
    { pubkey: bondingCurve, isSigner: false, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: true },
    { pubkey: payer, isSigner: true, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
  ],
  data: Buffer.concat([
    Buffer.from([0]), // Initialize discriminator
    // InitializeInstructionData struct
  ])
};
```

### 4. Buy Tokens

```typescript
const buyTokensIx = {
  programId: PROGRAM_ID,
  keys: [
    { pubkey: buyer, isSigner: true, isWritable: true },
    { pubkey: bondingCurve, isSigner: false, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: buyerTokenAccount, isSigner: false, isWritable: true },
    { pubkey: feeRecipient, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
  ],
  data: Buffer.concat([
    Buffer.from([1]), // BuyTokens discriminator
    // BuyTokensInstructionData struct
  ])
};
```

### 5. Sell Tokens

```typescript
const sellTokensIx = {
  programId: PROGRAM_ID,
  keys: [
    { pubkey: seller, isSigner: true, isWritable: true },
    { pubkey: bondingCurve, isSigner: false, isWritable: true },
    { pubkey: mint, isSigner: false, isWritable: false },
    { pubkey: sellerTokenAccount, isSigner: false, isWritable: true },
    { pubkey: feeRecipient, isSigner: false, isWritable: true },
    { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
  ],
  data: Buffer.concat([
    Buffer.from([2]), // SellTokens discriminator
    // SellTokensInstructionData struct
  ])
};
```

## Testing

### Rust Tests
Chạy unit tests:

```bash
cargo test
# hoặc
make test
```

### Client Tests
Chạy TypeScript client tests:

```bash
cd client
npm install
npm run test
# hoặc từ root directory
make client-test
```

### All Tests
Chạy tất cả tests:

```bash
make test-all
```

## Deployment

### Quick Deploy to Devnet

```bash
make deploy-devnet
```

### Manual Deployment

```bash
# Build program
cargo build-sbf

# Deploy to devnet
solana config set --url devnet
solana program deploy target/deploy/x_token.so

# Update program ID in client
# Edit client/bonding_curve_client.ts with the new program ID
```

### Using the Client

```bash
# Install client dependencies
cd client
npm install

# Run client tests (will create and fund test account)
npm run test

# Use in your own project
import { BondingCurveClient } from './bonding_curve_client';
```

## Development Workflow

```bash
# Setup development environment
make setup

# Development cycle
make dev          # Format, build, test
make quick-test   # Build and test only

# Full build and test
make all

# Deploy and test
make deploy-devnet
make client-test
```

## Security Considerations

- Slippage protection thông qua max/min amount parameters
- Authority validation cho các operations quan trọng
- Arithmetic overflow protection
- PDA validation để đảm bảo account security

## Dependencies

- `pinocchio`: Core framework
- `pinocchio-token`: Token program interactions
- `pinocchio-system`: System program interactions
- `bytemuck`: Zero-copy serialization

## License

MIT License
