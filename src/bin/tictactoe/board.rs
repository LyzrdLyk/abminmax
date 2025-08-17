use std::vec;

#[derive(Debug)]
pub struct Board {
    players: u8,
    length: u8,
    dimension: u8,
    state: Vec<u8>,
    current: u8,
}

impl Board {
    pub fn new(players: u8, length: u8, dimension: u8) -> Board {
        let card: usize = length.pow(dimension as u32) as usize;
        let mut ini_state: Vec<u8> = vec::Vec::with_capacity(card);
        ini_state.resize(card, 0);
        Board {
            players: players,
            length: length,
            dimension: dimension,
            current: 1,
            state: ini_state,
        }
    }

    pub fn apply_move(&self, vmove: usize) -> Option<Board> {
        let vslot = vmove - 1;
        let curstate = &self.state;
        if self.state[vslot] != 0 {
            return None;
        }
        let mut mvec: Vec<u8> = curstate.clone();
        let player = self.current;
        mvec[vslot] = player;

        Some(Board{players: self.players, length: self.length, dimension: self.dimension, current: (player %  self.players) + 1 , state: mvec })
    }

    // pub fn next(&mut self) -> Option<Self::Item> {
    //     self.count += 1;
    //     if self.count <= 5 {
    //         Some(self.count)
    //     } else {
    //         None
    //     }
    // }
}

pub struct BoardIterator {
    parent: Board,
    current: u8,
    vmove: Option<u8>,
}

impl BoardIterator {
    pub fn new(parent: Board, current: u8) -> Self {
        Self {
            parent,
            current,
            vmove: None,
        }
    }

    pub fn next(&mut self) -> Option<(usize, Board)> {
        match self.vmove {
            //start
            None => {
                let move_num: usize = 1;
                let child =  self.parent.apply_move(move_num);
                match child{
                    None => None,
                    Some(board) => Some((move_num,board))
                }
            }
            //continue
            Some(_vnum) => None,
        }
    }
}
