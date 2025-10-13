use yew::prelude::*;
use gloo_console::log;
use web_sys::HtmlInputElement;

use crate::components::common::spinner::Spinner;
use crate::components::common::error::ErrorDisplay;
use crate::components::common::input::Input;
use crate::components::common::button::Button;
use crate::services::wallet_service::{WalletService, Wallet, Transaction, DepositRequest, WithdrawRequest, TransferRequest};

#[function_component(WalletPage)]
pub fn wallet_page() -> Html {
    let wallet_state = use_state(|| None::<Wallet>);
    let transactions_state = use_state(|| None::<Vec<Transaction>>);
    let error_state = use_state(|| None::<String>);
    let loading_state = use_state(|| true);

    let deposit_amount_state = use_state(|| String::new());
    let withdraw_amount_state = use_state(|| String::new());
    let transfer_amount_state = use_state(|| String::new());
    let transfer_recipient_id_state = use_state(|| String::new());

    let fetch_wallet_data = { 
        let wallet_state = wallet_state.clone();
        let transactions_state = transactions_state.clone();
        let error_state = error_state.clone();
        let loading_state = loading_state.clone();
        Callback::from(move |_| {
            let wallet_state = wallet_state.clone();
            let transactions_state = transactions_state.clone();
            let error_state = error_state.clone();
            let loading_state = loading_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                loading_state.set(true);
                match WalletService::get_my_wallet().await {
                    Ok(wallet) => {
                        wallet_state.set(Some(wallet.clone()));
                        match WalletService::get_wallet_transactions().await {
                            Ok(transactions) => {
                                transactions_state.set(Some(transactions));
                                error_state.set(None);
                            },
                            Err(e) => {
                                log!("Failed to fetch transactions:", e.to_string());
                                error_state.set(Some(e.to_string()));
                            }
                        }
                    },
                    Err(e) => {
                        log!("Failed to fetch wallet:", e.to_string());
                        error_state.set(Some(e.to_string()));
                    }
                }
                loading_state.set(false);
            });
        })
    };

    use_effect_with_deps(move |fetch_wallet_data| {
        fetch_wallet_data.emit(());
        || ()
    }, fetch_wallet_data.clone());

    let on_deposit_amount_change = Callback::from(move |value: String| {
        deposit_amount_state.set(value);
    });

    let on_withdraw_amount_change = Callback::from(move |value: String| {
        withdraw_amount_state.set(value);
    });

    let on_transfer_amount_change = Callback::from(move |value: String| {
        transfer_amount_state.set(value);
    });

    let on_transfer_recipient_id_change = Callback::from(move |value: String| {
        transfer_recipient_id_state.set(value);
    });

    let handle_deposit = { 
        let deposit_amount_state = deposit_amount_state.clone();
        let fetch_wallet_data = fetch_wallet_data.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let amount_str = (*deposit_amount_state).clone();
            let fetch_wallet_data = fetch_wallet_data.clone();
            let error_state = error_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match amount_str.parse::<f64>() {
                    Ok(amount) => {
                        let request = DepositRequest { amount };
                        match WalletService::deposit_funds(request).await {
                            Ok(_) => {
                                log!("Deposit successful");
                                error_state.set(None);
                                fetch_wallet_data.emit(());
                            },
                            Err(e) => {
                                log!("Deposit failed:", e.to_string());
                                error_state.set(Some(e.to_string()));
                            }
                        }
                    },
                    Err(_) => error_state.set(Some("Invalid deposit amount".to_string())),
                }
            });
        })
    };

    let handle_withdraw = { 
        let withdraw_amount_state = withdraw_amount_state.clone();
        let fetch_wallet_data = fetch_wallet_data.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let amount_str = (*withdraw_amount_state).clone();
            let fetch_wallet_data = fetch_wallet_data.clone();
            let error_state = error_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match amount_str.parse::<f64>() {
                    Ok(amount) => {
                        let request = WithdrawRequest { amount };
                        match WalletService::withdraw_funds(request).await {
                            Ok(_) => {
                                log!("Withdrawal successful");
                                error_state.set(None);
                                fetch_wallet_data.emit(());
                            },
                            Err(e) => {
                                log!("Withdrawal failed:", e.to_string());
                                error_state.set(Some(e.to_string()));
                            }
                        }
                    },
                    Err(_) => error_state.set(Some("Invalid withdrawal amount".to_string())),
                }
            });
        })
    };

    let handle_transfer = { 
        let transfer_amount_state = transfer_amount_state.clone();
        let transfer_recipient_id_state = transfer_recipient_id_state.clone();
        let fetch_wallet_data = fetch_wallet_data.clone();
        let error_state = error_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let amount_str = (*transfer_amount_state).clone();
            let recipient_id_str = (*transfer_recipient_id_state).clone();
            let fetch_wallet_data = fetch_wallet_data.clone();
            let error_state = error_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match amount_str.parse::<f64>() {
                    Ok(amount) => {
                        match recipient_id_str.parse::<::uuid::Uuid>() {
                            Ok(recipient_wallet_id) => {
                                let request = TransferRequest { recipient_wallet_id, amount };
                                match WalletService::transfer_funds(request).await {
                                    Ok(_) => {
                                        log!("Transfer successful");
                                        error_state.set(None);
                                        fetch_wallet_data.emit(());
                                    },
                                    Err(e) => {
                                        log!("Transfer failed:", e.to_string());
                                        error_state.set(Some(e.to_string()));
                                    }
                                }
                            },
                            Err(_) => error_state.set(Some("Invalid recipient ID".to_string())),
                        }
                    },
                    Err(_) => error_state.set(Some("Invalid transfer amount".to_string())),
                }
            });
        })
    };

    html! {
        <div class="wallet-page">
            <h1>{ "My Wallet" }</h1>
            { if *loading_state { html! { <Spinner /> } } else { html! {} } }
            { if let Some(err) = &*error_state { html! { <ErrorDisplay message={err.clone()} /> } } else { html! {} } }

            { if let Some(wallet) = &*wallet_state {
                html! {
                    <div class="wallet-summary">
                        <h2>{ format!("Balance: {:.2} {}", wallet.balance, wallet.currency) }</h2>
                        <p>{ format!("Wallet ID: {}", wallet.id) }</p>
                        <p>{ format!("Status: {}", wallet.status) }</p>
                    </div>
                }
            } else { html! {} } }

            <div class="wallet-actions">
                <h3>{ "Deposit Funds" }</h3>
                <form onsubmit={handle_deposit}>
                    <Input
                        id="deposit-amount"
                        input_type="number"
                        value={(*deposit_amount_state).clone()}
                        onchange={on_deposit_amount_change}
                        placeholder="Amount"
                        step="0.01"
                    />
                    <Button label="Deposit" button_input_type="submit" />
                </form>

                <h3>{ "Withdraw Funds" }</h3>
                <form onsubmit={handle_withdraw}>
                    <Input
                        id="withdraw-amount"
                        input_type="number"
                        value={(*withdraw_amount_state).clone()}
                        onchange={on_withdraw_amount_change}
                        placeholder="Amount"
                        step="0.01"
                    />
                    <Button label="Withdraw" button_input_type="submit" />
                </form>

                <h3>{ "Transfer Funds" }</h3>
                <form onsubmit={handle_transfer}>
                    <Input
                        id="transfer-recipient-id"
                        input_input_type="text"
                        value={(*transfer_recipient_id_state).clone()}
                        onchange={on_transfer_recipient_id_change}
                        placeholder="Recipient Wallet ID"
                    />
                    <Input
                        id="transfer-amount"
                        input_input_type="number"
                        value={(*transfer_amount_state).clone()}
                        onchange={on_transfer_amount_change}
                        placeholder="Amount"
                        step="0.01"
                    />
                    <Button label="Transfer" button_input_type="submit" />
                </form>
            </div>

            <div class="wallet-transactions">
                <h3>{ "Transaction History" }</h3>
                { if let Some(transactions) = &*transactions_state {
                    if transactions.is_empty() {
                        html! { <p>{ "No transactions yet." }</p> }
                    } else {
                        transactions.iter().map(|tx| {
                            html! {
                                <div key={tx.id.to_string()} class="transaction-item">
                                    <p>{ format!("Type: {:?}", tx.transaction_type) }</p>
                                    <p>{ format!("Amount: {:.2}", tx.amount) }</p>
                                    <p>{ format!("Status: {:?}", tx.status) }</p>
                                    <p>{ format!("Description: {}", tx.description.as_deref().unwrap_or("N/A")) }</p>
                                    <p>{ format!("Date: {}", tx.created_at.format("%Y-%m-%d %H:%M")) }</p>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                } else {
                    html! { <p>{ "Loading transactions..." }</p> }
                } }
            </div>
        </div>
    }
}

