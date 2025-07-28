use std::error::Error;
use scraper::{Html, Selector};
use reqwest::blocking::get;

#[derive(Debug)]
pub struct InsiderTrade {
    pub name: String,
    pub title: String,
    pub trade_date: String,
    pub transaction_type: String,
    pub price: String,
    pub qty: String,
}

impl InsiderTrade{
    pub fn fetch_blk_insider_trades() -> Result<Vec<InsiderTrade>, Box<dyn Error>> {
        let url = "http://openinsider.com/screener?s=BLK&o=&pl=&ph=&ll=&lh=&fd=730&fdr=&td=0&tdr=&fdlyl=&fdlyh=&daysago=&xp=1&xs=1&vl=&vh=&ocl=&och=&sic1=-1&sicl=100&sich=9999&grp=0&nfl=&nfh=&nil=&nih=&nol=&noh=&v2l=&v2h=&oc2l=&oc2h=&sortcol=0&cnt=100&page=1";

        let body = get(url)?.text()?;
        let document = Html::parse_document(&body);

        let row_selector = Selector::parse("table.tinytable tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let mut trades = Vec::new();

        for (i, row) in document.select(&row_selector).enumerate() {
            if i == 0 { continue; } // skip header

            let tds: Vec<_> = row.select(&td_selector).collect();
            if tds.len() < 10 { continue; }

            trades.push(InsiderTrade {
                name: tds[1].text().collect::<String>().trim().to_string(),
                title: tds[2].text().collect::<String>().trim().to_string(),
                trade_date: tds[4].text().collect::<String>().trim().to_string(),
                transaction_type: tds[6].text().collect::<String>().trim().to_string(),
                price: tds[7].text().collect::<String>().trim().to_string(),
                qty: tds[8].text().collect::<String>().trim().to_string(),
            });
        }

        Ok(trades)
    }

}
