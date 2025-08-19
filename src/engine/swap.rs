use clap::ValueEnum;
use serde::Deserialize;

/// Represents the direction of a swap operation
#[derive(ValueEnum, Debug, Clone, Deserialize, PartialEq)]
pub enum SwapDirection {
    /// Buy operation - purchasing tokens
    #[serde(rename = "buy")]
    Buy,
    /// Sell operation - selling tokens
    #[serde(rename = "sell")]
    Sell,
}

impl From<SwapDirection> for u8 {
    fn from(value: SwapDirection) -> Self {
        match value {
            SwapDirection::Buy => 0,
            SwapDirection::Sell => 1,
        }
    }
}

impl From<u8> for SwapDirection {
    fn from(value: u8) -> Self {
        match value {
            0 => SwapDirection::Buy,
            1 => SwapDirection::Sell,
            _ => panic!("Invalid swap direction: {}", value),
        }
    }
}

/// Represents the input type for swap operations
#[derive(ValueEnum, Debug, Clone, Deserialize, PartialEq)]
pub enum SwapInType {
    /// Quantity-based input
    #[serde(rename = "qty")]
    Qty,
    /// Percentage-based input
    #[serde(rename = "pct")]
    Pct,
}

impl From<SwapInType> for u8 {
    fn from(value: SwapInType) -> Self {
        match value {
            SwapInType::Qty => 0,
            SwapInType::Pct => 1,
        }
    }
}

impl From<u8> for SwapInType {
    fn from(value: u8) -> Self {
        match value {
            0 => SwapInType::Qty,
            1 => SwapInType::Pct,
            _ => panic!("Invalid swap input type: {}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_direction_conversion() {
        assert_eq!(u8::from(SwapDirection::Buy), 0);
        assert_eq!(u8::from(SwapDirection::Sell), 1);
        
        assert_eq!(SwapDirection::from(0), SwapDirection::Buy);
        assert_eq!(SwapDirection::from(1), SwapDirection::Sell);
    }

    #[test]
    fn test_swap_in_type_conversion() {
        assert_eq!(u8::from(SwapInType::Qty), 0);
        assert_eq!(u8::from(SwapInType::Pct), 1);
        
        assert_eq!(SwapInType::from(0), SwapInType::Qty);
        assert_eq!(SwapInType::from(1), SwapInType::Pct);
    }

    #[test]
    #[should_panic(expected = "Invalid swap direction: 2")]
    fn test_invalid_swap_direction() {
        let _ = SwapDirection::from(2);
    }

    #[test]
    #[should_panic(expected = "Invalid swap input type: 2")]
    fn test_invalid_swap_in_type() {
        let _ = SwapInType::from(2);
    }
}
