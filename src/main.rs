use rand::Rng;
use std::io::{self, Read, Write};

fn main() {
    print!("\x1b[H\x1b[J"); // soft-clear the screen
    let mut field = MineField::new(10, 5, 5).unwrap();
    field.print().unwrap();
    for _ in 1..=10 {
        let event = read_mouse().unwrap();
        println!("event: {:?}", event);
        println!("clicked tile content: {:?}", field.clear_tile(event.x as usize, event.y as usize).unwrap());
    }
}

#[derive(Clone, Debug)]
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
                match self.tiles[adj_index].content { // consider making this its own function
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
                    // TODO: color mode
                    // TODO: hide un-probed tiles
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

    fn clear_tile(&mut self, target_x: usize, target_y: usize) -> Result<TileContent, &'static str> {
        // TODO: something is bugged in the way tiles are looked up
        if (target_x >= self.ncol) || (target_y >= self.nrow) { // checks for < 0 are implicit in the types
            return Err("out of bounds");
        }
        let target_tile: &mut Tile =  &mut self.tiles[target_x+target_y*(self.ncol as usize)];
        target_tile.probed = true;
        //todo!("clear contiguous tiles");
        return Ok(target_tile.content.clone());
    }

    fn flag_tile() {
        todo!()
    }
}

#[derive(Debug)]
struct MouseEvent {
    x: u8, // position from the left of the terminal, 1 based
    y: u8, // position from the top of the terminal, 1 based
    button: MouseButton, // which button was pressed/released
    shift: bool, // whether shift was held
    meta: bool, // whether meta/alt was held
    ctrl: bool, // whether control was held
}

#[derive(Debug)]
enum MouseButton { MB1, MB2, MB3, Release }

fn read_mouse() -> Result<MouseEvent, &'static str> {
    use termios::*;
    let FD_STDIN  = 0;
    let FD_STDOUT = 1;
    let tio_in_old = Termios::from_fd(FD_STDIN).expect("failed to get stdin"); // copy old attributes
    let mut tio_in = tio_in_old.clone();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = [0;1];

    //println!("termios_stdin: {:?}", tio_in);

    // set terminal attributes
    tio_in.c_lflag &= !ICANON; // no line-at-a-time, no line editing
    tio_in.c_lflag &= !ECHO; // don't print chars as they're typed
    tio_in.c_cc[VMIN] = 1; // byte-at-a-time
    tio_in.c_cc[VTIME] = 0; // hang until you get a char
    tcsetattr(FD_STDIN, TCSANOW, &tio_in).unwrap();

    // turn on mouse reporting
    print!("\x1b[?1000h");

    // flush streams
    //tcflush(FD_STDOUT, FD_STDOUT).expect("failed to flush output");
    stdout.flush().unwrap();
    termios::tcflush(FD_STDIN, FD_STDIN).unwrap();

    // read the input (consider doing this char-at-a-time, to recover from key-presses)
        // decode the report
            // see ~/src/c/cdraw/cdraw.c and ~/src/bash/bdraw/bdraw.sh
    stdin.read_exact(&mut buffer).unwrap();
    if buffer[0] != 27 {return Err("invalid response")} // escape
    stdin.read_exact(&mut buffer).unwrap();
    if buffer[0] != 91 {return Err("invalid response")} // '['
    stdin.read_exact(&mut buffer).unwrap();
    if buffer[0] != 77 {return Err("invalid response")} // 'M'
    stdin.read_exact(&mut buffer).unwrap();
    let b: u8 = buffer[0] -32;
    stdin.read_exact(&mut buffer).unwrap();
    let x: u8 = buffer[0] -32 -1; // subtract offset and convert to 0-based index
    stdin.read_exact(&mut buffer).unwrap();
    let y: u8 = buffer[0] -32 -1;

    let button: MouseButton;
    let b_discriminant: u8 = b & 0b11; // get the least two bits
    if          b_discriminant == 0 { button = MouseButton::MB1; } 
        else if b_discriminant == 1 { button = MouseButton::MB2; } 
        else if b_discriminant == 2 { button = MouseButton::MB3; } 
        else if b_discriminant == 3 { button = MouseButton::Release; } 
        else { panic!(); }
    let shift = (b &   0b100) != 0; // check if certain bits are set
    let meta  = (b &  0b1000) != 0;
    let ctrl  = (b & 0b10000) != 0;

    // turn off reporting
    print!("\x1b[?1000l");
    stdout.flush().unwrap();
    //tcflush(FD_STDOUT, FD_STDOUT).expect("failed to flush output");

    // restore the old attributes
    tcsetattr(0, TCSANOW, &tio_in_old).expect("failed to restore attributes for stdin");

    Ok(MouseEvent{x, y, button, shift, meta, ctrl})
 }
