use rand::Rng;

fn main() {
    println!("Hello, world!");
    let field = MineField::new(10, 5, 10).unwrap();
    let _ = field.print();
}

enum TileContent { Mine, Zero, One, Two, Three, Four, Five, Six, Seven, Eight, }

struct Tile {
    probed: bool,
    content: TileContent,
}

struct MineField {
    ncol: usize,
    nrow: usize,
    tiles: Vec<Tile>,
}

impl MineField {
    fn new(ncol: usize, nrow: usize, nmines: usize) -> Result<MineField, &'static str> {
        let ntiles = ncol*nrow;
        if ntiles <= 0 { panic!("tried to create a MineField with no tiles") }
        let mut tiles: Vec<Tile> = Vec::new();
        for _ in 1..=ntiles {
            tiles.push(Tile{probed: false, content: TileContent::Zero});
        }
        let mut field = MineField{ncol, nrow, tiles};
        field.populate_mines(nmines as usize)?;
        Ok(field)
    }

    fn populate_mines(&mut self, remaining: usize) -> Result<(), &'static str> {
        let ntiles: usize = (self.nrow*self.ncol).into();
        let target_ind: usize = rand::thread_rng().gen_range(0..ntiles);
        let target_x: usize = target_ind % self.ncol;
        let target_y: usize = target_ind/self.ncol;
        let target_tile = &mut self.tiles[target_ind];
        //let mut adj_ind: isize = -1;
        // better to keep adj_x and adj_y separate (instead of combining into an index)
        // that way, it's easy to check if the x is running off the edge of the field
        let mut adj_x: isize = -1;
        let mut adj_y: isize = -1;

        if matches!(target_tile.content, TileContent::Mine) { // there's already a mine in the target tile
            // TODO: replace this recursive implementation with an itterative one
            // The recursive version easily causes stack overflows on dense mine fields!
            self.populate_mines(remaining)?; // try again
        } else {
            target_tile.content = TileContent::Mine;

            // update the adjacent tiles
            'xloop: for i in -1..=1 {
                adj_x = target_x as isize +i;
                if (adj_x < 0) || (adj_x >= self.ncol as isize) {continue 'xloop}
                'yloop: for j in -1..=1 {
                    adj_y = target_y as isize +j;
                    if (adj_y < 0) || (adj_y >= self.nrow as isize) {continue 'yloop}
                    let adj_index = (adj_x as usize) +(adj_y as usize)*self.ncol;
                    //print!("target_x: {target_x}, target_y: {target_y}, target_ind: {target_ind}, adj_x: {adj_x}, adj_y: {adj_y}, adj_index: {adj_index}, i: {i}, j: {j}\n");
                    match self.tiles[adj_index].content {
                        TileContent::Zero  => self.tiles[adj_index].content = TileContent::One,
                        TileContent::One   => self.tiles[adj_index].content = TileContent::Two,
                        TileContent::Two   => self.tiles[adj_index].content = TileContent::Three,
                        TileContent::Three => self.tiles[adj_index].content = TileContent::Four,
                        TileContent::Four  => self.tiles[adj_index].content = TileContent::Five,
                        TileContent::Five  => self.tiles[adj_index].content = TileContent::Six,
                        TileContent::Six   => self.tiles[adj_index].content = TileContent::Seven,
                        TileContent::Seven => self.tiles[adj_index].content = TileContent::Eight,
                        _ => (),
                    }
                }
            }
        }

        if remaining > 1 {
            self.populate_mines(remaining-1)?;
        }
        Ok(())
    }

    fn populate_adjacency(&mut self) -> Result<(), &'static str> {
        /*
        for x in 0..(self.nrow) {
            for y in 0..(self.ncol) {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if (i+self.nrow < 0) | 
                    }
                }
            }
        }
        */
        Ok(())
    }

    fn print(&self) -> Result<(), &'static str> {
        for x in 0..(self.ncol) {
            for y in 0..(self.nrow) {
                match self.tiles[(x+y*self.ncol) as usize].content {
                    TileContent::Mine  => print!("M"),
                    TileContent::Zero  => print!(" "),
                    TileContent::One   => print!("1"),
                    TileContent::Two   => print!("2"),
                    TileContent::Three => print!("3"),
                    TileContent::Four  => print!("4"),
                    TileContent::Five  => print!("5"),
                    TileContent::Six   => print!("6"),
                    TileContent::Seven => print!("7"),
                    TileContent::Eight => print!("8"),
                }
            }
            print!("\n")
        }
        Ok(())
    }
}
