use super::*;
use chain::*;
    
    //Testing hasher
    #[test]
    fn test_hasher(){
        let b = Block::new("helloworld");
        let c = Block::new("helloworld");
        assert_eq!(b.hash_block(),c.hash_block());
    }

    //Testing basic chain emulation
    #[test]
    fn test_blockchain(){
        let tr1 = Transaction::new("11232884929", "12312654366", 0.25);
        let tr2 = Transaction::new("11232234929", "12312134366", 0.5);
        let tr3 = Transaction::new("11223534929", "12311474366", 0.75);
        let mut p = Mempool::new();
        let ls: Vec<Block> = Vec::new(); 
        let mut bc = Blockchain::new(ls);
        Blockchain::on_transaction(tr1,&mut p);
        Blockchain::on_transaction(tr2,&mut p);
        Blockchain::on_transaction(tr3,&mut p);
        match bc.mint(&mut p){
            Ok(()) => {},
            Err(e) => {print!("{:?}",e)}
        };
        match bc.mint(&mut p){
            Ok(()) => {},
            Err(e) => {print!("{:?}",e)}
        };
        match bc.mint(&mut p){
            Ok(()) => {},
            Err(e) => {print!("{:?}",e)}
        };
        assert_ne!(bc.list[0].get_prev(),bc.list[1].get_prev());
        assert_ne!(bc.list[1].get_prev(),bc.list[2].get_prev());
        assert_eq!(bc.list[0].hash_block().to_string(),bc.list[1].get_prev());
        assert_eq!(bc.list[1].hash_block().to_string(),bc.list[2].get_prev());
    }
    
    //Testing the mint with PoW
    //Commented because finding hash with 8 ones is very time-consuming
    /*#[test]
    fn test_pow(){
        let tr1 = Transaction::new("11232884929", "12312654366", 0.25);
        let tr2 = Transaction::new("11232234929", "12312134366", 0.5);
        let tr3 = Transaction::new("11223534929", "12311474366", 0.75);
        let mut p = Mempool::new();
        let ls: Vec<Block> = Vec::new(); 
        let mut bcc = Blockchain::new(ls);
        Blockchain::on_transaction(tr1,&mut p);
        Blockchain::on_transaction(tr2,&mut p);
        Blockchain::on_transaction(tr3,&mut p);
        let hd: Header = bcc.mint_pow(&mut p).expect("Error! Mempool had not enough transactions to mint!");
        let mut s = DefaultHasher::new();
        bcc.list[1].trans.clone().hash(&mut s);
        hd.nonce.hash(&mut s);
        let h = s.finish();
        assert_eq!(hd.block_hash,h.to_string());
        assert_eq!(h.count_ones(),8);
        assert_eq!(bcc.list[1].trans.len(),3);
    }*/
    //Testing the config loading function
    #[test]
    fn test_config(){
        let cfg:Config = load_config();
        assert_eq!(&cfg.get_savepath(),"src/chain_state/save.json");
    }

    //Testing block mining
    //Commented because finding hash with 8 ones is very time-consuming
    /*#[test]
    fn test_mining(){
        let tr1 = Transaction::new("11232884929", "12312654366", 0.25);
        let tr2 = Transaction::new("11232234929", "12312134366", 0.5);
        let tr3 = Transaction::new("11223534929", "12311474366", 0.75);
        let mut p = Mempool::new();
        let ls: Vec<Block> = Vec::new(); 
        let mut bcc = Blockchain::new(ls);
        Blockchain::on_transaction(tr1,&mut p);
        Blockchain::on_transaction(tr2,&mut p);
        Blockchain::on_transaction(tr3,&mut p);
        let hd: Header = bcc.mint_pow(&mut p).expect("Error! Mempool had not enough transactions to mint!");
        let config:Config = load_config();
        let t = mine_block(&bcc.list[1], &hd, &config);
        assert!(&t.1.to_string().starts_with(&config.difficulty));
    }*/

    //Testing save/load of blockhain state
    #[test]
    fn test_saveload(){
        let tr1 = Transaction::new("11232884929", "12312654366", 0.25);
        let tr2 = Transaction::new("11232234929", "12312134366", 0.5);
        let tr3 = Transaction::new("11223534929", "12311474366", 0.75);
        let mut p = Mempool::new();
        let ls: Vec<Block> = Vec::new(); 
        let mut bc = Blockchain::new(ls);
        Blockchain::on_transaction(tr1,&mut p);
        Blockchain::on_transaction(tr2,&mut p);
        Blockchain::on_transaction(tr3,&mut p);
        match bc.mint(&mut p){
            Ok(()) => {},
            Err(e) => {print!("{:?}",e)}
        };
        match bc.mint(&mut p){
            Ok(()) => {},
            Err(e) => {print!("{:?}",e)}
        };
        match bc.mint(&mut p){
            Ok(()) => {},
            Err(e) => {print!("{:?}",e)}
        };
        let config:Config = load_config();
        let r = bc.save(&config);
        assert_eq!(r.unwrap(),());
        let gs: Vec<Block> = Vec::new(); 
        let mut bcc = Blockchain::new(gs);
        bcc=bcc.load("src/chain_state/save.json");
        assert_eq!(bcc,bc);
        assert_eq!(bcc.list.len(),bc.list.len());
        assert_eq!(bcc.list[0].trans.len(),bc.list[0].trans.len());
    }