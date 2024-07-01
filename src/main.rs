use rand::Rng;

fn main() {
    println!("Hello, world!");
    let field = MineField::new(20, 20, 5).unwrap();
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
        let index = rand::thread_rng().gen_range(0..ntiles);
        let target_tile = &mut self.tiles[index];
        //let target_tile: Option<&mut Tile> = self.tiles.get();
        //let target_tile: &mut Tile = match target_tile {
        //    Some(t) => t,
        //    None => return Err("tried to place a mine where there was no tile"),
        //};

        if matches!(target_tile.content, TileContent::Mine) {
            self.populate_mines(remaining)?; // try again
        } else {
            target_tile.content = TileContent::Mine;
        }

        if remaining > 1 {
            self.populate_mines(remaining-1)?;
        }
        Ok(())
    }

    fn print(&self) -> Result<(), &'static str> {
        for x in 0..(self.nrow) {
            for y in 0..(self.ncol) {
                match self.tiles[(x+y*self.ncol) as usize].content {
                    TileContent::Mine  => print!("M"),
                    TileContent::Zero  => print!("0"),
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
