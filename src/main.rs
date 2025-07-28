mod modules;
use modules::insider_tracker::InsiderTrade;

fn normalize_name(raw: &str) -> String {
    let parts: Vec<&str> = raw.trim().split_whitespace().collect();
    if parts.len() == 2 {
        format!("{} {}", parts[1], parts[0])
    } else {
        raw.to_string()
    }
}
fn clean_number(raw: &str) -> String {
    raw.replace(',', "").replace('$', "").trim().to_string()
}

fn main() {
    match InsiderTrade::fetch_blk_insider_trades() {
        Ok(trades) => {
            println!("📈 Dernières transactions d'initiés sur $BLK :\n");

            for trade in trades {
                let name = normalize_name(&trade.name);
                let qty = clean_number(&trade.qty);
                let price = clean_number(&trade.price);
                let action = match trade.transaction_type.to_lowercase().contains("sale") {
                    true => "vendu",
                    false => "acheté",
                };

                println!(
                    "Le {}, {} ({}) a {} {} actions à {} $ ({})",
                    trade.trade_date,
                    name,
                    &trade.transaction_type[..1], // S ou P
                    action,
                    qty,
                    price,
                    trade.transaction_type,
                );
            }
        }
        Err(e) => eprintln!("Erreur de récupération : {}", e),
    }
}
