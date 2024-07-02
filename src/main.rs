use rand::Rng;

fn main() {
    println!("Hello, world!");
    let field = MineField::new(10, 5, 5).unwrap();
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
    fn new(nrow: usize, ncol: usize, nmines: usize) -> Result<MineField, &'static str> {
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

    fn populate_mines(&mut self, nmines: usize) -> Result<(), &'static str> {
        let ntiles: usize = (self.nrow*self.ncol).into();
        let mut remaining = nmines;

        while remaining > 0 {
            // This is "slow" for fields that are large and dense.
            // It might make more sense to remove mines from a fully-populated field,
            // when > half of the tiles are mines.
            let target_ind: usize = rand::thread_rng().gen_range(0..ntiles);
            let target_x: usize = target_ind % self.ncol;
            let target_y: usize = target_ind / self.ncol;
            match self.place_mine(target_x, target_y) {
                Ok(()) => remaining -= 1,
                Err("occupied") => continue,
                Err(other) => return Err(other),
            }
        }
        Ok(())
    }

    fn place_mine(&mut self, target_x: usize, target_y: usize) -> Result<(), &'static str> {
        let target_tile = &mut self.tiles[target_x+target_y*(self.ncol as usize)];

        if matches!(target_tile.content, TileContent::Mine) { // there's already a mine in the target tile
            return Err("occupied")
        }
        target_tile.content = TileContent::Mine;

        // update the adjacent tiles
        'xloop: for i in -1..=1 {
            let adj_x = target_x as isize +i;
            if (adj_x < 0) || (adj_x >= self.ncol as isize) {continue 'xloop}
            'yloop: for j in -1..=1 {
                let adj_y = target_y as isize +j;
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
                    TileContent::Eight => panic!("tried to increment a tile that was already Eight"),
                    TileContent::Mine => (),
                }
            }
        }
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

/*
 * see ~/src/c/cdraw/cdraw.c and ~/src/bash/bdraw/bdraw.sh for info on how to handle inputs
 * "\033[?1000h" turn on mouse reporting (see "Mouse tracking" and "Mouse Reporting" in `man console_codes`.)
 * "\033[?1000l" turn off mouse reporting
 * There are fancier escape codes (1005, 1006, 1015), but these don't work on my setup.
 * I'm not sure if that's an issue with Kitty, tmux, or my configs, but I don't care too much at the moment.
 * The main downside to the 1000 protocol is that x and y positions are limited to values from 1 to 223.
 * (So, for locations past 223, it just reports 223.)
 * That's plenty good for minesweeper though!
 */
struct MouseEvent { // can safely use smaller integer types; check console_codes
    x: usize, // position from the left of the terminal, 1 based
    y: usize, // position from the top of the terminal, 1 based
    button: usize, // which button was pressed/released
    shift: bool, // whether shift was held
    meta: bool, // whether meta/alt was held
    ctrl: bool, // whether control was held
}

fn read_mouse() -> Result<MouseEvent, &'static str> {
    todo!();
    Err("did nothing")
 }
