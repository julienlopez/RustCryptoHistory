use unqlite::{UnQLite, Cursor};

use crate::error::Error;
use crate::transaction::Transaction;
use crate::transaction::Type;

pub struct Database {
    handle : unqlite::UnQLite
}

fn parse_type(value : &str) -> Result<Type, Error> {
    match value.chars().nth(0) {
        Some('B') => Ok(Type::Bought),
        Some('S') => Ok(Type::Sold),
        Some(_) => Err(Error::Message("cannot parse type : ".to_string() + &value.to_string())),
        None => Err(Error::Message("cannot parse type : ".to_string() + &value.to_string())),
    }
}

fn parse_transaction(currency: &str, line: &str) -> Result<Transaction, Error> {
    let parts : Vec<&str> = line.split(',').map(|str| str.trim()).collect();
    if parts.len() != 5 {
        return Err(Error::Message("Invalid transaction line : ".to_string() + &line.to_string()));
    }
    Ok(Transaction{currency : currency.to_string(),
        r#type: parse_type(parts[0])?,
        amount: parts[1].parse::<f64>()?,
        total_value: parts[2].parse::<f64>()?,
        price: parts[3].parse::<f64>()?,
        timestamp : parts[4].to_string()
    })
}

fn parse_transactions(currency: &str, line: &str) -> Result<Vec<Transaction>, Error> {
    line.lines().map(|str| str.trim()).map(|l| parse_transaction(currency, l)).collect()
}

impl Database {
    pub fn open(file_path: &str) -> Database {
        Database{ handle: UnQLite::create(file_path)}
    }

    pub fn open_readonly(file_path: &str) -> Database {
        Database{ handle: UnQLite::open_readonly(file_path)}
    }

    pub fn currencies(&self) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut entry = self.handle.first();
        // Iterate records
        loop {
            if entry.is_none() { break; }
            let record = entry.unwrap();
            let key = String::from_utf8(record.key()).unwrap();
            if key != "currencies.index" {
                res.push(key);
            }
            entry = record.next();
        }
        res
    }
    
    pub fn contains(&self, currency: &String) -> bool {
        self.currencies().contains(currency)
    }

    // Result<void> addTransaction(const Transaction& transaction);

    pub fn transactions(&self, currency: &str) -> Result<Vec<Transaction>, Error>{
        let mut entry = self.handle.first();
        // Iterate records
        loop {
            if entry.is_none() { break; }
            let record = entry.unwrap();
            let key = String::from_utf8(record.key())?;
            if key == currency {
                let value = String::from_utf8(record.value())?;
                return parse_transactions(currency, &value);
            }
            entry = record.next();
        }
        Err(Error::Message("Invalid currency".to_string()))
    }

    // pub fn all_transactions(&self) -> Result<Vec<Transaction>, Error> {
    //     // self.currencies().iter().map(|s| self.transactions(s)).into_iter().collect()
    //     Err(Error::Unknown)
    // }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Type;
    use crate::transaction::Transaction;

    #[test]
    fn test_parse_transaction() {
        let currency = "BTC";
        let transaction = "B,0.5,15.000000,30.0,2023-03-27T08:00:00";
        let res = parse_transaction(currency, transaction);
        assert!(res.as_ref().is_ok());
        assert_eq!(res.unwrap(), Transaction{currency : currency.to_string(), r#type : Type::Bought, amount : 0.5, total_value :  15., price : 30., timestamp : "2023-03-27T08:00:00".to_string()});
    }

    #[test]
    fn test_parse_transactions() {
        let currency = "BTC";
        let transactions = "B,0.5,15.000000,30.0,2023-01-01T08:00:00\nS,0.5,20.000000,40.0,2023-01-01T09:00:00";
        let res = parse_transactions(currency, transactions);
        assert!(res.as_ref().is_ok());
        let transactions = res.unwrap();
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0], Transaction{currency : currency.to_string(), r#type : Type::Bought, amount : 0.5, total_value :  15., price : 30., timestamp : "2023-01-01T08:00:00".to_string()});
        assert_eq!(transactions[1], Transaction{currency : currency.to_string(), r#type : Type::Sold, amount : 0.5, total_value :  20., price : 40., timestamp : "2023-01-01T09:00:00".to_string()});
    }
}