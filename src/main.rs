use rand::Rng;
use std::io::{self, Read, Write};

// TODO: 
// check win condition (progressive counter would be cool, instead of re-checking the whole field each move)
// proper end-game behavior (show mines, show clicked mine)
// ability to flag mines
// match the color scheme of traditional (Windows) minesweeper
// scoring system
// timer?
// create a status line(s)
// take command line args
//      density, width, height, help, score leaderboard
// score history file (csv of score, userid, and date)
//      take path for score history from environment variable

fn main() {
    let mut field = MineField::new(20, 10, 10).unwrap();
    println!("Hello, world!");
    soft_clear();
    loop {
        print!("\x1b[H");
        field.print().unwrap();
        let event = read_mouse().unwrap();
        match event.button {
            MouseButton::Release => continue,
            _ => (),
        }
        println!("event: {:?}", event);
        //field.probe_tile(event.x as usize, event.y as usize).unwrap();
        println!("clicked tile content: {:?}", field.probe_chain(event.x as usize, event.y as usize).unwrap());
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TileContent { Mine, Zero, One, Two, Three, Four, Five, Six, Seven, Eight, }

struct Tile {
    probed: bool,
    content: TileContent,
}

impl Tile {
    fn print(&self) -> (){
        let mut bg: &str = "\x1b[100m";
        let mut rst: &str = "\x1b[0m";
        if !self.probed {
            print!("{bg}?{rst}");
            return ()
        }

        bg = "\x1b[40";
        rst = "\x1b[0m";
        match self.content {
            // TODO: color mode
            // TODO: hide un-probed tiles
            TileContent::Mine  => print!("{bg}\x1b[41;30mM\x1b[0m{rst}"), // red bg, black fg
            TileContent::Zero  => print!("{bg}\x1b[39m {rst}"),
            TileContent::One   => print!("{bg}\x1b[39m1{rst}"), // default
            TileContent::Two   => print!("{bg}\x1b[95m2{rst}"), // light magenta
            TileContent::Three => print!("{bg}\x1b[35m3{rst}"), // magenta
            TileContent::Four  => print!("{bg}\x1b[34m4{rst}"), // blue
            TileContent::Five  => print!("{bg}\x1b[36m5{rst}"), // cyan
            TileContent::Six   => print!("{bg}\x1b[32m6{rst}"), // green
            TileContent::Seven => print!("{bg}\x1b[33m7{rst}"), // yellow
            TileContent::Eight => print!("{bg}\x1b[31m8{rst}"), // red
        }
    }
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
        for y in 0..(self.nrow) {
            for x in 0..(self.ncol) {
                self.tiles[(x+y*self.ncol) as usize].print();
            }
            print!("\n")
        }
        Ok(())
    }

    fn probe_tile(&mut self, target_x: usize, target_y: usize) -> Result<TileContent, &'static str> {
        // TODO: this should just be part of probe_chain()
        let tile: &mut Tile = &mut self.tiles[target_x +target_y*self.ncol];
        tile.probed = true;
        match tile.content { // consider making this its own function
            TileContent::Mine => { return Err("GAME OVER!"); },
            _ => (),
        }
        return Ok(tile.content.clone())
    }

    fn probe_chain(&mut self, target_x: usize, target_y: usize) -> Result<TileContent, &'static str> {
        if (target_x >= self.ncol) || (target_y >= self.nrow) { // checks for < 0 are implicit in the types
            return Err("out of bounds");
        }
        self.probe_tile(target_x, target_y).unwrap();
        let target_tile: &mut Tile = &mut self.tiles[target_x +target_y*self.ncol];
        let target_content = target_tile.content.clone();

        // chain contiguous zeroes; probe adjacent tiles
        // TODO: try to make this cleaner
        let mut adj_x: isize;
        let mut adj_y: isize;
        for sign in [1,-1] {
            for axis in [0, 1] {
                adj_x = target_x as isize +sign*(1-axis);
                adj_y = target_y as isize +sign*axis;
                if     (adj_x < 0) || (adj_x >= self.ncol as isize) // bounds check
                    || (adj_y < 0) || (adj_y >= self.nrow as isize) { continue }
                let adj_x = adj_x as usize;
                let adj_y = adj_y as usize;
                let adj_content = self.tiles[adj_x +adj_y*self.ncol].content.clone();
                let adj_probed = self.tiles[adj_x +adj_y*self.ncol].probed;
                match adj_content { // consider making this its own function
                    TileContent::Zero  => { if !adj_probed { self.probe_chain(adj_x, adj_y).unwrap(); } },
                    TileContent::Mine => (),
                    _ => {
                        match target_content {
                            TileContent::Zero => { self.probe_tile(adj_x, adj_y).unwrap(); },
                            _ => (),
                        }
                    },
                }
            }

        }
        return Ok(target_content);
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

fn soft_clear() -> () {
    print!("\x1b[H\x1b[J"); // soft-clear the screen
}
