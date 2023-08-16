#[derive(Debug, PartialEq)]
pub struct Details {
    pub amount: f64,
    pub total_value: f64,
    pub price: f64,
    pub timestamp: String,
}

#[derive(Debug, PartialEq)]
pub struct Trade {
    pub currency: String,
    pub opening_details: Details,
    pub closing_details: Option<Details>,
}

impl Trade {
    pub fn is_open(&self) -> bool {
        self.closing_details.is_none()
    }

    pub fn is_closed(&self) -> bool {
        self.closing_details.is_some()
    }

    pub fn result(&self) -> Option<f64> {
        self.closing_details.as_ref().map(|cd| {
            (cd.total_value - self.opening_details.total_value) / self.opening_details.total_value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_trade() {
        let trade = Trade {
            currency: "BTC".to_string(),
            opening_details: Details {
                amount: 0.5,
                total_value: 30.,
                price: 60.0,
                timestamp: "2023-01-01T08:00:00".to_string(),
            },
            closing_details: None,
        };
        assert!(trade.is_open());
        assert!(!trade.is_closed());
        assert!(trade.result().is_none());
    }

    #[test]
    fn test_closed_trade() {
        let trade = Trade {
            currency: "BTC".to_string(),
            opening_details: Details {
                amount: 0.5,
                total_value: 30.,
                price: 60.0,
                timestamp: "2023-01-01T08:00:00".to_string(),
            },
            closing_details: Some(Details {
                amount: 0.5,
                total_value: 40.,
                price: 80.0,
                timestamp: "2023-01-01T09:00:00".to_string(),
            }),
        };
        assert!(!trade.is_open());
        assert!(trade.is_closed());
        let result = trade.result();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 1. / 3.);
    }
}
