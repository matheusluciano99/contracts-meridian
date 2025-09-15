# Soroban Insurance Contracts

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── insurance
│       ├── src
│       │   ├── lib.rs
│       │   ├── policy_registry.rs
│       │   └── risk_pool.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Deploy dos Contratos

### 🚀 Método Recomendado: Soroban CLI

1. **Verificar instalação do Soroban CLI**:
```bash
soroban --version
```

2. **Executar deploy**:
```bash
./deploy-soroban.sh
```

O script irá:
- ✅ Fazer build dos contratos
- ✅ Configurar rede testnet
- ✅ Solicitar credenciais da conta
- ✅ Deploy PolicyRegistry
- ✅ Deploy RiskPool
- 📋 **Mostrar Contract IDs**

### 🔧 Método Manual (Soroban CLI)

Se preferir fazer manualmente:

```bash
# Build
soroban contract build

# Adicionar rede
soroban config network add testnet \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase "Test SDF Network ; September 2015"

# Adicionar conta
soroban keys add testnet --secret-key

# Deploy contratos
POLICY_ID=$(soroban contract deploy --wasm target/wasm32v1-none/release/insurance.wasm --source testnet --network testnet)
RISK_POOL_ID=$(soroban contract deploy --wasm target/wasm32v1-none/release/insurance.wasm --source testnet --network testnet)
```

## Contratos Disponíveis

### PolicyRegistry
- **Funções**: `activate_policy`, `pause_policy`, `get_policy`
- **Propósito**: Gerenciar apólices de seguro

### RiskPool
- **Funções**: `collect_premium`, `payout`, `get_balance`
- **Propósito**: Gerenciar pool de risco e pagamentos

## Testes

```bash
# Executar todos os testes
cargo test

# Testes específicos
cd contracts/insurance
cargo test
```

## Desenvolvimento

```bash
# Formatar código
cargo fmt --all

# Limpar builds
cargo clean
```