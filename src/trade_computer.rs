use crate::trade::Details;
use crate::trade::Trade;
use crate::transaction::Transaction;
use crate::transaction::Type;

fn find_opening_of_a_trade(transactions: &Vec<Transaction>) -> Option<usize> {
    transactions.iter().position(|t| t.r#type == Type::Bought)
}

fn approx(d1: f64, d2: f64) -> bool {
    (d2 - d1).abs() < d2 * 0.001
}

fn find_corresponding_closing_transaction(
    transactions: &Vec<Transaction>,
    opening_transaction: usize,
) -> Option<usize> {
    let opening = &transactions[opening_transaction];
    transactions.iter().position(|t| {
        t.r#type == Type::Sold && t.currency == opening.currency && approx(t.amount, opening.amount)
    })
}

fn details_from_transaction(transaction: &Transaction) -> Details {
    Details {
        amount: transaction.amount,
        total_value: transaction.total_value,
        price: transaction.price,
        timestamp: transaction.timestamp.clone(),
    }
}

pub fn analyze(mut transactions: Vec<Transaction>) -> Vec<Trade> {
    if transactions.is_empty() {
        return vec![];
    }
    transactions.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
    transactions.sort_by(|a, b| a.currency.cmp(&b.currency));

    let mut trades = vec![];
    while let Some(opening) = find_opening_of_a_trade(&transactions) {
        match find_corresponding_closing_transaction(&transactions, opening) {
            Some(close) => {
                trades.push(Trade {
                    currency: transactions[opening].currency.clone(),
                    opening_details: details_from_transaction(&transactions[opening]),
                    closing_details: Some(details_from_transaction(&transactions[close])),
                });
                transactions.remove(close);
            }
            None => {
                trades.push(Trade {
                    currency: transactions[opening].currency.clone(),
                    opening_details: details_from_transaction(&transactions[opening]),
                    closing_details: None,
                });
            }
        }
        transactions.remove(opening);
    }
    trades
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare(transaction: &Transaction, trade_details: &Details) -> bool {
        trade_details.amount == transaction.amount
            && trade_details.total_value == transaction.total_value
            && trade_details.price == transaction.price
            && trade_details.timestamp == transaction.timestamp
    }

    #[test]
    fn test_empty_input() {
        assert!(analyze(vec![]).is_empty());
    }

    #[test]
    fn test_with_open_trades_only() {
        let transactions = vec![
            Transaction {
                currency: "BTC".to_string(),
                r#type: Type::Bought,
                amount: 1.0,
                total_value: 10.0,
                price: 10.0,
                timestamp: "timestamp1".to_string(),
            },
            Transaction {
                currency: "ETH".to_string(),
                r#type: Type::Bought,
                amount: 2.0,
                total_value: 40.0,
                price: 20.0,
                timestamp: "timestamp2".to_string(),
            },
        ];
        let trades = analyze(transactions.clone());
        assert_eq!(trades.len(), 2);

        assert_eq!(&trades[0].currency, &transactions[0].currency);
        assert!(compare(&transactions[0], &trades[0].opening_details));
        assert!(&trades[0].closing_details.is_none());

        assert_eq!(&trades[1].currency, &transactions[1].currency);
        assert!(compare(&transactions[1], &trades[1].opening_details));
        assert!(&trades[1].closing_details.is_none());
    }

    #[test]
    fn test_with_closed_trades_only() {
        let transactions = vec![
            Transaction {
                currency: "BTC".to_string(),
                r#type: Type::Bought,
                amount: 1.0,
                total_value: 10.0,
                price: 10.0,
                timestamp: "timestamp1".to_string(),
            },
            Transaction {
                currency: "ETH".to_string(),
                r#type: Type::Bought,
                amount: 2.0,
                total_value: 40.0,
                price: 20.0,
                timestamp: "timestamp3".to_string(),
            },
            Transaction {
                currency: "BTC".to_string(),
                r#type: Type::Sold,
                amount: 1.0,
                total_value: 15.0,
                price: 15.0,
                timestamp: "timestamp2".to_string(),
            },
            Transaction {
                currency: "ETH".to_string(),
                r#type: Type::Sold,
                amount: 2.0,
                total_value: 30.0,
                price: 15.0,
                timestamp: "timestamp4".to_string(),
            },
        ];
        let trades = analyze(transactions.clone());
        assert_eq!(trades.len(), 2);

        assert_eq!(&trades[0].currency, &transactions[0].currency);
        assert!(compare(&transactions[0], &trades[0].opening_details));
        assert!(&trades[0].is_closed());
        assert!(compare(
            &transactions[2],
            &trades[0].closing_details.as_ref().unwrap()
        ));

        assert_eq!(&trades[1].currency, &transactions[1].currency);
        assert!(compare(&transactions[1], &trades[1].opening_details));
        assert!(&trades[1].is_closed());
        assert!(compare(
            &transactions[3],
            &trades[1].closing_details.as_ref().unwrap()
        ));
    }
}
