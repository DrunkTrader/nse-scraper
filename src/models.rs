// src/models.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteData {
    pub info: StockInfo,
    pub priceInfo: PriceInfo,
    pub securityInfo: SecurityInfo,
    pub metadata: Metadata,
    #[serde(rename = "marketDeptOrderBook")]
    pub market_depth: MarketDepth,
    pub tradingInfo: TradingInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockInfo {
    pub symbol: String,
    pub companyName: String,
    pub industry: Option<String>,
    pub activeSeries: Vec<String>,
    pub debtSeries: Vec<String>,
    pub isFNOSec: bool,
    pub isCASec: bool,
    pub isSLBSec: bool,
    pub isDebtSec: bool,
    pub isSuspended: bool,
    pub isETFSec: bool,
    pub isDelisted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceInfo {
    pub lastPrice: f64,
    pub change: f64,
    pub pChange: f64,
    pub previousClose: f64,
    pub open: f64,
    pub close: Option<f64>,
    pub vwap: f64,
    pub lowerCP: Option<String>,
    pub upperCP: Option<String>,
    pub pPriceBand: String,
    pub basePrice: f64,
    pub intraDayHighLow: IntraDayHighLow,
    pub weekHighLow: WeekHighLow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntraDayHighLow {
    pub min: f64,
    pub max: f64,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeekHighLow {
    pub min: f64,
    pub max: f64,
    pub minDate: String,
    pub maxDate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityInfo {
    pub boardStatus: String,
    pub tradingStatus: String,
    pub tradingSegment: String,
    pub sessionNo: String,
    pub slb: String,
    pub classOfShare: String,
    pub derivatives: String,
    pub surveillance: String,
    pub faceValue: f64,
    pub issuedSize: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub series: String,
    pub symbol: String,
    pub isin: String,
    pub status: String,
    pub listingDate: String,
    pub industry: String,
    pub lastUpdateTime: String,
    pub pdSectorInd: String,
    pub pdSectorPe: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketDepth {
    pub buy: Vec<DepthOrder>,
    pub sell: Vec<DepthOrder>,
    pub tradeInfo: TradeInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthOrder {
    pub price: f64,
    pub quantity: u32,
    pub orders: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeInfo {
    pub totalBuyQuantity: u64,
    pub totalSellQuantity: u64,
    pub totalTradedValue: f64,
    pub totalTradedVolume: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingInfo {
    pub totalTradedVolume: u64,
    pub totalTradedValue: f64,
    pub totalMarketCap: f64,
    pub ffmc: f64,
    pub impact: f64,
    pub deliveryQuantity: Option<u64>,
    pub deliveryPercentage: Option<f64>,
    pub deliveryToTradedQuantity: Option<f64>,
    pub varMargin: Option<f64>,
    pub marketLot: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketStatus {
    pub marketState: Vec<MarketSegmentState>,
    pub lastUpdateTime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketSegmentState {
    pub market: String,
    pub marketStatus: String,
    pub tradeDate: String,
    pub index: Option<String>,
    pub last: Option<f64>,
    pub variation: Option<f64>,
    pub percentChange: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndicesData {
    pub data: Vec<IndexData>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexData {
    pub key: String,
    pub index: String,
    pub indexSymbol: String,
    pub last: f64,
    pub variation: f64,
    pub percentChange: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub previousClose: f64,
    pub yearHigh: f64,
    pub yearLow: f64,
    pub pe: Option<f64>,
    pub pb: Option<f64>,
    pub dy: Option<f64>,
    pub declines: Option<u32>,
    pub advances: Option<u32>,
    pub unchanged: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalData {
    pub symbol: String,
    pub data: Vec<DailyData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyData {
    #[serde(rename = "CH_TIMESTAMP")]
    pub timestamp: String,
    #[serde(rename = "CH_OPENING_PRICE")]
    pub open: f64,
    #[serde(rename = "CH_TRADE_HIGH_PRICE")]
    pub high: f64,
    #[serde(rename = "CH_TRADE_LOW_PRICE")]
    pub low: f64,
    #[serde(rename = "CH_CLOSING_PRICE")]
    pub close: f64,
    #[serde(rename = "CH_LAST_TRADED_PRICE")]
    pub last: f64,
    #[serde(rename = "CH_PREVIOUS_CLS_PRICE")]
    pub prev_close: f64,
    #[serde(rename = "CH_TOT_TRADED_QTY")]
    pub volume: u64,
    #[serde(rename = "CH_TOT_TRADED_VAL")]
    pub value: f64,
    #[serde(rename = "CH_52WEEK_HIGH_PRICE")]
    pub year_high: f64,
    #[serde(rename = "CH_52WEEK_LOW_PRICE")]
    pub year_low: f64,
    
    // Additional fields for technical analysis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsi: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sma_20: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sma_50: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sma_200: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ema_12: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ema_26: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macd: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macd_signal: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macd_histogram: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bollinger_upper: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bollinger_middle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bollinger_lower: Option<f64>,
}

// New struct for time frame selection
#[derive(Debug, Clone, Copy)]
pub enum TimeFrame {
    Daily,
    Weekly,
    Monthly,
}

impl TimeFrame {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "1" => Some(TimeFrame::Daily),
            "2" => Some(TimeFrame::Weekly),
            "3" => Some(TimeFrame::Monthly),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeFrame::Daily => "daily",
            TimeFrame::Weekly => "weekly",
            TimeFrame::Monthly => "monthly",
        }
    }
}

// New struct for consolidated data after time frame conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeFrameData {
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
    pub value: f64,
}

// New struct for technical indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalIndicators {
    pub date: String,
    pub rsi_14: Option<f64>,
    pub sma_20: Option<f64>,
    pub sma_50: Option<f64>,
    pub sma_200: Option<f64>,
    pub ema_12: Option<f64>,
    pub ema_26: Option<f64>,
    pub macd: Option<f64>,
    pub macd_signal: Option<f64>,
    pub macd_histogram: Option<f64>,
    pub bollinger_upper: Option<f64>,
    pub bollinger_middle: Option<f64>,
    pub bollinger_lower: Option<f64>,
}

// New struct for consolidated data with technical indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedData {
    pub symbol: String,
    pub time_frame: String,
    pub from_date: String,
    pub to_date: String,
    pub data: Vec<TimeFrameData>,
    pub indicators: Option<Vec<TechnicalIndicators>>,
}

// Helper functions for time frame conversion
impl HistoricalData {
    pub fn to_time_frame(&self, time_frame: TimeFrame) -> ConsolidatedData {
        match time_frame {
            TimeFrame::Daily => self.to_daily(),
            TimeFrame::Weekly => self.to_weekly(),
            TimeFrame::Monthly => self.to_monthly(),
        }
    }
    
    fn to_daily(&self) -> ConsolidatedData {
        let mut data = Vec::new();
        
        for day in &self.data {
            data.push(TimeFrameData {
                date: day.timestamp.clone(),
                open: day.open,
                high: day.high,
                low: day.low,
                close: day.close,
                volume: day.volume,
                value: day.value,
            });
        }
        
        // Get date range
        let from_date = self.data.first().map(|d| d.timestamp.clone()).unwrap_or_default();
        let to_date = self.data.last().map(|d| d.timestamp.clone()).unwrap_or_default();
        
        ConsolidatedData {
            symbol: self.symbol.clone(),
            time_frame: "daily".to_string(),
            from_date,
            to_date,
            data,
            indicators: None,
        }
    }
    
    fn to_weekly(&self) -> ConsolidatedData {
        let mut weekly_data = Vec::new();
        if self.data.is_empty() {
            return ConsolidatedData {
                symbol: self.symbol.clone(),
                time_frame: "weekly".to_string(),
                from_date: String::new(),
                to_date: String::new(),
                data: weekly_data,
                indicators: None,
            };
        }
        
        // Sort data by date
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| {
            // Parse dates like "01-01-2024"
            let parse_date = |date_str: &str| -> Option<NaiveDate> {
                let parts: Vec<&str> = date_str.split('-').collect();
                if parts.len() != 3 {
                    return None;
                }
                
                let day = parts[0].parse::<u32>().ok()?;
                let month = parts[1].parse::<u32>().ok()?;
                let year = parts[2].parse::<i32>().ok()?;
                
                NaiveDate::from_ymd_opt(year, month, day)
            };
            
            let date_a = parse_date(&a.timestamp);
            let date_b = parse_date(&b.timestamp);
            
            date_a.cmp(&date_b)
        });
        
        // Group by week
        let mut current_week: Option<(TimeFrameData, NaiveDate)> = None;
        
        for day in sorted_data {
            let date_str = &day.timestamp;
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() != 3 {
                continue;
            }
            
            let day_val = parts[0].parse::<u32>().unwrap_or(1);
            let month = parts[1].parse::<u32>().unwrap_or(1);
            let year = parts[2].parse::<i32>().unwrap_or(2023);
            
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day_val) {
                match &mut current_week {
                    None => {
                        // Start a new week
                        current_week = Some((
                            TimeFrameData {
                                date: day.timestamp.clone(),
                                open: day.open,
                                high: day.high,
                                low: day.low,
                                close: day.close,
                                volume: day.volume,
                                value: day.value,
                            },
                            date
                        ));
                    }
                    Some((week_data, week_start)) => {
                        // Check if this is a new week
                        let days_diff = date.signed_duration_since(*week_start).num_days();
                        if days_diff >= 7 {
                            // Push current week and start a new one
                            weekly_data.push(week_data.clone());
                            current_week = Some((
                                TimeFrameData {
                                    date: day.timestamp.clone(),
                                    open: day.open,
                                    high: day.high,
                                    low: day.low,
                                    close: day.close,
                                    volume: day.volume,
                                    value: day.value,
                                },
                                date
                            ));
                        } else {
                            // Update current week
                            week_data.high = week_data.high.max(day.high);
                            week_data.low = week_data.low.min(day.low);
                            week_data.close = day.close; // Last day's close
                            week_data.volume += day.volume;
                            week_data.value += day.value;
                        }
                    }
                }
            }
        }
        
        // Add the last week if any
        if let Some((week_data, _)) = current_week {
            weekly_data.push(week_data);
        }
        
        // Get date range
        let from_date = sorted_data.first().map(|d| d.timestamp.clone()).unwrap_or_default();
        let to_date = sorted_data.last().map(|d| d.timestamp.clone()).unwrap_or_default();
        
        ConsolidatedData {
            symbol: self.symbol.clone(),
            time_frame: "weekly".to_string(),
            from_date,
            to_date,
            data: weekly_data,
            indicators: None,
        }
    }
    
    fn to_monthly(&self) -> ConsolidatedData {
        let mut monthly_data = Vec::new();
        if self.data.is_empty() {
            return ConsolidatedData {
                symbol: self.symbol.clone(),
                time_frame: "monthly".to_string(),
                from_date: String::new(),
                to_date: String::new(),
                data: monthly_data,
                indicators: None,
            };
        }
        
        // Sort data by date
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| {
            // Parse dates like "01-01-2024"
            let parse_date = |date_str: &str| -> Option<NaiveDate> {
                let parts: Vec<&str> = date_str.split('-').collect();
                if parts.len() != 3 {
                    return None;
                }
                
                let day = parts[0].parse::<u32>().ok()?;
                let month = parts[1].parse::<u32>().ok()?;
                let year = parts[2].parse::<i32>().ok()?;
                
                NaiveDate::from_ymd_opt(year, month, day)
            };
            
            let date_a = parse_date(&a.timestamp);
            let date_b = parse_date(&b.timestamp);
            
            date_a.cmp(&date_b)
        });
        
        // Group by month
        let mut current_month: Option<(TimeFrameData, (u32, i32))> = None;
        
        for day in sorted_data {
            let date_str = &day.timestamp;
            let parts: Vec<&str> = date_str.split('-').collect();
            if parts.len() != 3 {
                continue;
            }
            
            let day_val = parts[0].parse::<u32>().unwrap_or(1);
            let month = parts[1].parse::<u32>().unwrap_or(1);
            let year = parts[2].parse::<i32>().unwrap_or(2023);
            
            match &mut current_month {
                None => {
                    // Start a new month
                    current_month = Some((
                        TimeFrameData {
                            date: format!("{}-{}", month, year), // Format as MM-YYYY
                            open: day.open,
                            high: day.high,
                            low: day.low,
                            close: day.close,
                            volume: day.volume,
                            value: day.value,
                        },
                        (month, year)
                    ));
                }
                Some((month_data, (current_month_num, current_year))) => {
                    // Check if this is a new month
                    if month != *current_month_num || year != *current_year {
                        // Push current month and start a new one
                        monthly_data.push(month_data.clone());
                        current_month = Some((
                            TimeFrameData {
                                date: format!("{}-{}", month, year),
                                open: day.open,
                                high: day.high,
                                low: day.low,
                                close: day.close,
                                volume: day.volume,
                                value: day.value,
                            },
                            (month, year)
                        ));
                    } else {
                        // Update current month
                        month_data.high = month_data.high.max(day.high);
                        month_data.low = month_data.low.min(day.low);
                        month_data.close = day.close; // Last day's close
                        month_data.volume += day.volume;
                        month_data.value += day.value;
                    }
                }
            }
        }
        
        // Add the last month if any
        if let Some((month_data, _)) = current_month {
            monthly_data.push(month_data);
        }
        
        // Get date range
        let from_date = sorted_data.first().map(|d| d.timestamp.clone()).unwrap_or_default();
        let to_date = sorted_data.last().map(|d| d.timestamp.clone()).unwrap_or_default();
        
        ConsolidatedData {
            symbol: self.symbol.clone(),
            time_frame: "monthly".to_string(),
            from_date,
            to_date,
            data: monthly_data,
            indicators: None,
        }
    }
    
    // Calculate technical indicators
    pub fn calculate_indicators(&mut self) {
        if self.data.is_empty() {
            return;
        }
        
        // Sort data by date
        self.data.sort_by(|a, b| {
            let parse_date = |date_str: &str| -> Option<NaiveDate> {
                let parts: Vec<&str> = date_str.split('-').collect();
                if parts.len() != 3 {
                    return None;
                }
                
                let day = parts[0].parse::<u32>().ok()?;
                let month = parts[1].parse::<u32>().ok()?;
                let year = parts[2].parse::<i32>().ok()?;
                
                NaiveDate::from_ymd_opt(year, month, day)
            };
            
            let date_a = parse_date(&a.timestamp);
            let date_b = parse_date(&b.timestamp);
            
            date_a.cmp(&date_b)
        });
        
        // Calculate SMA
        self.calculate_sma(20);
        self.calculate_sma(50);
        self.calculate_sma(200);
        
        // Calculate EMA
        self.calculate_ema(12);
        self.calculate_ema(26);
        
        // Calculate MACD
        self.calculate_macd();
        
        // Calculate RSI
        self.calculate_rsi(14);
        
        // Calculate Bollinger Bands
        self.calculate_bollinger_bands(20, 2.0);
    }
    
    fn calculate_sma(&mut self, period: usize) {
        if self.data.len() < period {
            return;
        }
        
        let mut sum_window = 0.0;
        
        // Fill initial window
        for i in 0..period {
            sum_window += self.data[i].close;
        }
        
        // Calculate initial SMA
        let sma = sum_window / period as f64;
        match period {
            20 => self.data[period - 1].sma_20 = Some(sma),
            50 => self.data[period - 1].sma_50 = Some(sma),
            200 => self.data[period - 1].sma_200 = Some(sma),
            _ => return,
        }
        
        // Calculate remaining SMAs using sliding window
        for i in period..self.data.len() {
            sum_window = sum_window - self.data[i - period].close + self.data[i].close;
            let sma = sum_window / period as f64;
            
            match period {
                20 => self.data[i].sma_20 = Some(sma),
                50 => self.data[i].sma_50 = Some(sma),
                200 => self.data[i].sma_200 = Some(sma),
                _ => return,
            }
        }
    }
    
    fn calculate_ema(&mut self, period: usize) {
        if self.data.len() < period {
            return;
        }
        
        // Calculate multiplier
        let multiplier = 2.0 / (period as f64 + 1.0);
        
        // Use SMA as the first EMA value
        let mut sum = 0.0;
        for i in 0..period {
            sum += self.data[i].close;
        }
        let mut ema = sum / period as f64;
        
        // Set initial EMA
        match period {
            12 => self.data[period - 1].ema_12 = Some(ema),
            26 => self.data[period - 1].ema_26 = Some(ema),
            _ => return,
        }
        
        // Calculate EMA for remaining points
        for i in period..self.data.len() {
            ema = (self.data[i].close - ema) * multiplier + ema;
            
            match period {
                12 => self.data[i].ema_12 = Some(ema),
                26 => self.data[i].ema_26 = Some(ema),
                _ => return,
            }
        }
    }
    
    fn calculate_macd(&mut self) {
        // Need both EMAs calculated first
        if self.data.len() < 26 {
            return;
        }
        
        // Calculate MACD line (EMA12 - EMA26)
        for i in 26..self.data.len() {
            if let (Some(ema12), Some(ema26)) = (self.data[i].ema_12, self.data[i].ema_26) {
                let macd = ema12 - ema26;
                self.data[i].macd = Some(macd);
            }
        }
        
        // Calculate MACD signal line (9-day EMA of MACD)
        let signal_period = 9;
        if self.data.len() < 26 + signal_period {
            return;
        }
        
        // Get initial sum for the signal
        let mut sum = 0.0;
        for i in 26..26 + signal_period {
            if let Some(macd) = self.data[i].macd {
                sum += macd;
            } else {
                return; // Missing MACD value
            }
        }
        
        let multiplier = 2.0 / (signal_period as f64 + 1.0);
        let mut signal = sum / signal_period as f64;
        
        // Set initial signal
        self.data[26 + signal_period - 1].macd_signal = Some(signal);
        
        // Calculate signal for remaining points
        for i in (26 + signal_period)..self.data.len() {
            if let Some(macd) = self.data[i].macd {
                signal = (macd - signal) * multiplier + signal;
                self.data[i].macd_signal = Some(signal);
                
                // Calculate histogram
                self.data[i].macd_histogram = Some(macd - signal);
            }
        }
    }
    
    fn calculate_rsi(&mut self, period: usize) {
        if self.data.len() <= period {
            return;
        }
        
        let mut gains = Vec::new();
        let mut losses = Vec::new();
        
        // Calculate price changes
        for i in 1..self.data.len() {
            let change = self.data[i].close - self.data[i-1].close;
            if change >= 0.0 {
                gains.push(change);
                losses.push(0.0);
            } else {
                gains.push(0.0);
                losses.push(-change);
            }
        }
        
        // Calculate initial averages
        let mut avg_gain = gains[0..period].iter().sum::<f64>() / period as f64;
        let mut avg_loss = losses[0..period].iter().sum::<f64>() / period as f64;
        
        // Calculate initial RSI
        let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        self.data[period].rsi = Some(rsi);
        
        // Calculate RSI for remaining points using smoothing
        for i in (period+1)..self.data.len() {
            // Smooth averages
            avg_gain = (avg_gain * (period as f64 - 1.0) + gains[i-1]) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) + losses[i-1]) / period as f64;
            
            // Calculate RS and RSI
            let rs = if avg_loss == 0.0 { 100.0 } else { avg_gain / avg_loss };
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            self.data[i].rsi = Some(rsi);
        }
    }
    
    fn calculate_bollinger_bands(&mut self, period: usize, num_std_dev: f64) {
        if self.data.len() < period {
            return;
        }
        
        for i in (period-1)..self.data.len() {
            // Calculate SMA (middle band)
            let mut sum = 0.0;
            for j in (i-(period-1))..=i {
                sum += self.data[j].close;
            }
            let sma = sum / period as f64;
            
            // Calculate standard deviation
            let mut variance_sum = 0.0;
            for j in (i-(period-1))..=i {
                variance_sum += (self.data[j].close - sma).powi(2);
            }
            let std_dev = (variance_sum / period as f64).sqrt();
            
            // Calculate bands
            self.data[i].bollinger_middle = Some(sma);
            self.data[i].bollinger_upper = Some(sma + num_std_dev * std_dev);
            self.data[i].bollinger_lower = Some(sma - num_std_dev * std_dev);
        }
    }
}