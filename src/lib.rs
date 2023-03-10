use javarandom::JavaRandom;

const PIECE_COUNT: usize = 11;

pub struct Randomizer {
    // randomizer
    r: JavaRandom,
    pieces: Vec<usize>,
}

impl Randomizer {
    pub fn new(piece_enable: [bool; PIECE_COUNT], seed: i64) -> Randomizer {
        let mut randomizer = Randomizer {
            r: JavaRandom::with_seed(seed),
            pieces: vec![],
        };
        randomizer.set_state(piece_enable, seed);
        return randomizer;
    }


    pub fn set_state(&mut self, piece_enable: [bool; 11], seed: i64) -> &mut Randomizer {
        self.set_piece_enable(piece_enable);
        self.reseed(seed);
        return self;
    }

    pub fn set_piece_enable(&mut self, piece_enable: [bool; 11]) -> &mut Randomizer {
        let mut piece = 0;
        for i in 0..11 {
            if piece_enable[i] {
                piece += 1;
            }
        }
        self.pieces.clear();
        self.pieces.resize(piece, 0);
        piece = 0;
        for i in 0..PIECE_COUNT {
            if piece_enable[i] {
                self.pieces[piece] = i;
                piece += 1;
            }
        }
        return self;
    }

    pub fn reseed(&mut self, seed: i64) -> &mut Randomizer {
        self.r = JavaRandom::with_seed(seed);
        return self;
    }

    pub fn is_piece_szo_only(&mut self) -> bool {
        for i in 0..11 {
            if self.pieces[i] != 4 || self.pieces[i] != 6 || self.pieces[i] != 3 {
                return false;
            }
        }
        return true;
    }
}


pub struct BagRandomizer {
    pub randomizer: Randomizer,
    bag: Vec<usize>,
    pt: usize,
}

impl BagRandomizer {
    pub fn new(piece_enable: [bool; 11], seed: i64) -> BagRandomizer {
        let randomizer = Randomizer::new(piece_enable, seed);
        BagRandomizer {
            randomizer,
            bag: vec![],
            pt: 0,
        }
    }

    pub fn init(&mut self) -> &mut BagRandomizer {
        self.bag = vec![];
        self.bag.resize(self.randomizer.pieces.len(), 0);
        self.pt = 0;
        for i in 0..self.randomizer.pieces.len() {
            self.bag[i] = self.randomizer.pieces[i];
        }
        self.shuffle();
        return self;
    }

    pub fn shuffle(&mut self) -> &mut BagRandomizer {
        for i in (2..=7).rev() {
            let ind: usize = i;
            let j: usize = self.randomizer.r.next_int(Some(ind.try_into().unwrap())).try_into().unwrap();
            let temp: usize = self.bag[ind - 1];
            self.bag[ind - 1] = self.bag[j].into();
            self.bag[j] = temp;
        }
        return self;
    }

    pub fn next(&mut self) -> usize {
        let id = self.bag[self.pt];
        self.pt += 1;
        if self.pt == self.randomizer.pieces.len() {
            self.pt = 0;
            self.shuffle();
        }
        return id;
    }
}


pub struct BagNoSZORandomizer {
    pub bag_randomizer: BagRandomizer,
    first_bag: bool,
}

impl BagNoSZORandomizer {
    pub fn new(piece_enable: [bool; 11], seed: i64) -> BagNoSZORandomizer {
        let bag_randomizer = BagRandomizer::new(piece_enable, seed);
        BagNoSZORandomizer {
            bag_randomizer,
            first_bag: true,
        }
    }
    
    pub fn init(&mut self) -> &mut BagNoSZORandomizer {
        self.first_bag = true;
        self.bag_randomizer.init();
        return self;
    }

    pub fn shuffle(&mut self) -> &mut BagNoSZORandomizer {
        if self.first_bag && !self.bag_randomizer.randomizer.is_piece_szo_only() {
            while self.bag_randomizer.bag[0] == 4 || self.bag_randomizer.bag[0] == 6 || self.bag_randomizer.bag[0] == 3 {
                self.bag_randomizer.shuffle();
            }
            self.first_bag = false;
        }
        return self;
    }
}