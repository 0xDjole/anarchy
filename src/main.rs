mod blockchain;

use blockchain::Blockchain;

fn main() {
     let mut blockchain = Blockchain::new();

     blockchain.create_block();

     println!("{:? }", blockchain)
}
