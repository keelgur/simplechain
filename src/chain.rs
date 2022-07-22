use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use chrono::{Utc, DateTime};
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use serde::Deserialize;
use borsh::*;


#[derive(Debug)]
pub enum Errs{
    EmptyMempoolError,
    NotEnoughTransactionsError,
}

#[derive(Hash)]
 #[derive(Clone)]
 #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Transaction{
    from: String,
    to: String,
    amount: String,
}

impl Transaction{
    pub fn new(fr:&str,t:&str,am:f64) -> Self {
    Self { from: fr.to_owned(), to: t.to_owned(), amount: am.to_string() }
  }
}
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
 pub struct Block{
     prev: String,
     pub trans: Vec<Transaction>,
}

impl Hash for Block{
    fn hash<H: Hasher>(&self, mut state: &mut H) {
        self.prev.hash(&mut state);
        self.trans.hash(&mut state);
    }    
}

impl Block{
    pub fn new(pr:&str) -> Self {
        Self { prev: pr.to_owned(), trans: vec![] }
      }
    pub fn hash_block(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
    pub fn get_prev(&self)->String{
        self.prev.clone()
    }
    
}
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct Blockchain{
    pub list: Vec<Block>,
}

pub struct Mempool{
    pending_trans:Vec<Transaction>,
}

impl Mempool{
    pub fn new() -> Self {
        Self { pending_trans: vec![] }
    }
}

impl Blockchain{
    pub fn new(mut list: Vec<Block>) -> Self{
       let mut s = DefaultHasher::new();
       let tr: Vec<Transaction> = Vec::new();
       Utc::now().timestamp().hash(&mut s);
       list.push(Block{
           trans: tr,
           prev: s.finish().to_string(),
       });
       Blockchain{
           list,
       }
    }
    
     pub fn on_transaction(tx: Transaction, pool:&mut Mempool) {
         pool.pending_trans.push(tx);
    }
    pub fn mint(&mut self, pool: &mut Mempool) ->Result<(),Errs>{
        let mut sv = Vec::new();
        if !pool.pending_trans.first().is_none(){
        sv.push(pool.pending_trans.first().cloned().unwrap());
        pool.pending_trans.remove(0);
        }
        else{
        return Err(Errs::EmptyMempoolError)}
        self.list.push(Block{
            prev:self.list[self.list.len()-1].hash_block().to_string(),
            trans: sv,
        
     });
     Ok(())
   }
}


pub struct Header{
    timestamp: DateTime::<Utc>,
    block_hash: String,
    nonce: u64,
}


impl Blockchain{
    fn mint_pow(&mut self, pool: &mut Mempool)->Result<Header,Errs>{
        let now = Utc::now();
        let svl:Vec<Transaction>;
        if pool.pending_trans.len()>=3{
           svl=pool.pending_trans.drain(0..3).collect();
            }
            else{
            return Err(Errs::NotEnoughTransactionsError)}
            println!("{}",svl[0].from);
            println!("{}",svl[1].from);
            println!("{}",svl[2].from);
         let mut nonce = 0;
         loop{
            let mut s = DefaultHasher::new();
         svl.clone().hash(&mut s);
         nonce.hash(&mut s);
         let h = s.finish();
         if h.count_ones()==8{
            self.list.push(Block{
                prev:self.list[self.list.len()-1].hash_block().to_string(),
                trans:svl,
            });
            return Ok(Header{
              timestamp:now,
              block_hash: h.to_string(),
              nonce:nonce,
            });
            }
            nonce+=1;
         }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config{
    difficulty: String,
    save_path: String,
}

pub fn load_config()->Config{
  let json_path = Path::new("src/config.json");
  let file = File::open(json_path).expect("file not found");
  let config:Config = serde_json::from_reader(file).expect("error while reading");
 config
}

impl Config{
    pub fn get_savepath(&self)->String{
        self.save_path.clone()
    }
}


fn mine_block(b:&Block, head:&Header, diff:&Config) -> (u64, String) {
    let mut nonce = 0;

    loop {
        if nonce % 100000 == 0 {
            println!("nonce: {}", nonce);
        }
        let mut s = DefaultHasher::new();
        let difficulty_prefix:&str = &diff.difficulty[0..];
        head.timestamp.hash(&mut s);
        b.prev.hash(&mut s);
        b.trans.hash(&mut s);
        head.nonce.hash(&mut s);
        let hash = s.finish();
        let bhash:u8 = s.finish().try_into().unwrap();
        if bhash.to_string().starts_with(difficulty_prefix) {
            return (nonce, bhash.to_string())
        }
        nonce += 1;
    }
}

impl Blockchain{
    pub fn load<P: AsRef<Path>>(&self,path: P) -> Blockchain {
        if let Ok(mut file) = File::open(path) {
            let mut buf = vec![];
            if file.read_to_end(&mut buf).is_ok() {
                if let Ok(blockchain) = Blockchain::try_from_slice(&buf[..]) {
                    return blockchain;
                }
            }
           
        }
        println!("Failed to read from given path!!!");
            let mut l:Vec<Block>=Vec::new();
            return Blockchain::new(l)
    }

    pub fn save(&self,config: &Config) -> std::io::Result<()> {
        let mut f = File::create(&config.save_path)?;
        let buf = borsh::to_vec(&self)?;
        f.write_all(&buf[..])?;
        Ok(())
    }
}

