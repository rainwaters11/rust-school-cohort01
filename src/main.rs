fn main() {
    let tx1 = Transaction::new("Alice", "Bob", 50.0);
    let tx2 = Transaction::new("Bob", "Carol", 25.0);
    let tx3 = Transaction::new("Carol", "Alice", 10.0);

    let txs = vec![tx1, tx2, tx3];

    let block = Block::new(1, txs);

    block.print_summary();
}

#[derive(Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: f64,
}

impl Transaction {
    fn new(from: &str, to: &str, amount: f64) -> Self {
        Transaction {
            from: from.to_string(),
            to: to.to_string(),
            amount,
        }
    }

    fn describe(&self) -> String {
        format!("{} -> {} : {:.2}", self.from, self.to, self.amount)
    }
}

#[derive(Debug)]
struct Block {
    index: u32,
    txs: Vec<Transaction>,
}

impl Block {
    fn new(index: u32, txs: Vec<Transaction>) -> Self {
        Block { index, txs }
    }

    fn total_value(&self) -> f64 {
        self.txs.iter().map(|tx| tx.amount).sum()
    }

    fn print_summary(&self) {
        println!("=== Block #{} ===", self.index);
        println!("Transactions: {}", self.txs.len());
        for tx in &self.txs {
            println!("  {}", tx.describe());
        }
        println!("Total Value : {:.2}", self.total_value());
    }
}