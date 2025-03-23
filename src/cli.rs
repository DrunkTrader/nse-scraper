// src/cli.rs
use crate::{NseScraper, Result};
use chrono::{Local, NaiveDate}; // Removed Datelike
use csv::Writer;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use crate::models::TimeFrame;

pub struct NseCli {
    scraper: NseScraper,
}

impl NseCli {
    pub fn new() -> Result<Self> {
        let scraper = NseScraper::new()?;
        Ok(Self { scraper })
    }

    pub async fn run(&self) -> Result<()> {
        println!("NSE Data Scraper CLI");
        println!("====================");

        // Get stock symbol
        let symbol = Self::prompt_input("Enter stock symbol (e.g., RELIANCE): ")?;
        
        // Get time frame
        println!("\nSelect time frame:");
        println!("1. Daily");
        println!("2. Weekly");
        println!("3. Monthly");
        let time_frame = Self::prompt_input("Enter your choice (1-3): ")?;
        let time_frame_enum = TimeFrame::from_str(&time_frame)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid time frame choice"))?;
        
        // Get duration
        println!("\nSelect duration:");
        println!("1. Last week");
        println!("2. Last month");
        println!("3. Last 3 months");
        println!("4. Last 6 months");
        println!("5. Last year");
        println!("6. Custom date range");
        let duration_choice = Self::prompt_input("Enter your choice (1-6): ")?;
        
        // Calculate date range
        let (from_date, to_date) = Self::calculate_date_range(&duration_choice)?;
        
        println!("\nFetching data for {} from {} to {}...", symbol, from_date, to_date);
        
        // Get historical data
        let historical = self.scraper.get_historical_data(&symbol, "EQ", &from_date, &to_date).await?;
        
        // Convert to selected time frame
        let consolidated = historical.to_time_frame(time_frame_enum);
        
        // Generate filename with time frame
        let filename = format!(
            "{}_{}_{}_{}.csv",
            symbol,
            consolidated.time_frame,
            consolidated.from_date,
            consolidated.to_date
        );
        
        // Save to CSV
        self.save_to_csv(&consolidated.data, &filename)?;
        
        println!("Data saved to {}", filename);
        
        Ok(())
    }
    
    fn prompt_input(prompt: &str) -> std::io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }
    
    fn calculate_date_range(choice: &str) -> std::io::Result<(String, String)> {
        let today = Local::now().naive_local().date();
        let to_date = today.format("%d-%m-%Y").to_string();
        
        let from_date = match choice.trim() {
            "1" => today.checked_sub_signed(chrono::Duration::days(7)),
            "2" => today.checked_sub_signed(chrono::Duration::days(30)),
            "3" => today.checked_sub_signed(chrono::Duration::days(90)),
            "4" => today.checked_sub_signed(chrono::Duration::days(180)),
            "5" => today.checked_sub_signed(chrono::Duration::days(365)),
            "6" => {
                print!("Enter start date (DD-MM-YYYY): ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                let date_parts: Vec<&str> = input.trim().split('-').collect();
                if date_parts.len() != 3 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid date format"));
                }
                
                let day = date_parts[0].parse::<u32>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Invalid day")
                })?;
                
                let month = date_parts[1].parse::<u32>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Invalid month")
                })?;
                
                let year = date_parts[2].parse::<i32>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Invalid year")
                })?;
                
                NaiveDate::from_ymd_opt(year, month, day)
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid choice")),
        };
        
        match from_date {
            Some(date) => Ok((date.format("%d-%m-%Y").to_string(), to_date)),
            None => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid date calculation")),
        }
    }
    
    fn save_to_csv<P: AsRef<Path>>(&self, data: &[crate::models::TimeFrameData], path: P) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = Writer::from_writer(file);
        
        // Write header
        writer.write_record(&["Date", "Open", "High", "Low", "Close", "Volume", "Value"])?;
        
        // Write data
        for day in data {
            writer.write_record(&[
                &day.date,
                &day.open.to_string(),
                &day.high.to_string(),
                &day.low.to_string(),
                &day.close.to_string(),
                &day.volume.to_string(),
                &day.value.to_string(),
            ])?;
        }
        
        writer.flush()?;
        Ok(())
    }
}