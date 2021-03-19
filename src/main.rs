use blockchain_rust::blockchain::Blockchain;

fn main() {
    let mut bc = Blockchain::new();
    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");
    bc.traverse();
}
