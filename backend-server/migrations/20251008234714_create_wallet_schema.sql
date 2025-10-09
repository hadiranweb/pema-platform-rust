
CREATE TYPE WALLET_STATUS AS ENUM (
    'active',
    'inactive',
    'suspended'
);
CREATE TYPE TRANSACTION_TYPE AS ENUM (
    'deposit',
    'withdrawal',
    'purchase',
    'refund'
);
CREATE TYPE TRANSACTION_STATUS AS ENUM (
    'pending',
    'completed',
    'failed',
    'reversed'
);
CREATE TYPE PURCHASE_FLOW_STATUS AS ENUM (
    'initiated',
    'pending_approval',
    'approved',
    'rejected',
    'completed',
    'failed'
);
CREATE TYPE REFUND_STATUS AS ENUM (
    'pending',
    'approved',
    'rejected',
    'completed'
);
CREATE TYPE ADMIN_ACTION_TYPE AS ENUM (
    'approve_charge',
    'reject_charge',
    'reverse_transaction',
    'suspend_wallet',
    'activate_wallet'
);

CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    balance BIGINT NOT NULL DEFAULT 0,
    currency VARCHAR(10) NOT NULL DEFAULT 'PMC',
    status WALLET_STATUS NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE purchase_flows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    wallet_id UUID NOT NULL REFERENCES wallets(id),
    amount BIGINT NOT NULL,
    status PURCHASE_FLOW_STATUS NOT NULL DEFAULT 'initiated',
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    wallet_id UUID NOT NULL REFERENCES wallets(id),
    type TRANSACTION_TYPE NOT NULL,
    amount BIGINT NOT NULL,
    status TRANSACTION_STATUS NOT NULL DEFAULT 'pending',
    description TEXT,
    reference_id UUID, -- Could be purchase_flow_id or refund_request_id
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE refund_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    user_id UUID NOT NULL,
    amount BIGINT NOT NULL,
    reason TEXT,
    status REFUND_STATUS NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE admin_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    admin_id UUID NOT NULL,
    action_type ADMIN_ACTION_TYPE NOT NULL,
    target_id UUID NOT NULL, -- Wallet ID, Transaction ID, or Refund Request ID
    details JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance optimization
CREATE INDEX idx_wallets_user_id ON wallets(user_id);
CREATE INDEX idx_purchase_flows_user_id ON purchase_flows(user_id);
CREATE INDEX idx_purchase_flows_wallet_id ON purchase_flows(wallet_id);
CREATE INDEX idx_transactions_wallet_id ON transactions(wallet_id);
CREATE INDEX idx_transactions_reference_id ON transactions(reference_id);
CREATE INDEX idx_refund_requests_transaction_id ON refund_requests(transaction_id);
CREATE INDEX idx_admin_actions_admin_id ON admin_actions(admin_id);
CREATE INDEX idx_admin_actions_target_id ON admin_actions(target_id);

