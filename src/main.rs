mod modules;
use modules::insider_tracker::InsiderTrade;

fn main() {
    match InsiderTrade::fetch_blk_insider_trades() {
        Ok(trades) => {
            println!("📈 Dernières transactions d'initiés sur $BLK :\n");

            for trade in trades {
                trade.println();
            }
        }
        Err(e) => eprintln!("Erreur de récupération : {}", e),
    }
}
