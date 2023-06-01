use core::panic;
use std::{ fs::{self, File}, io::{stdin, Write}, env::{self} };
use colored::{Colorize, ColoredString};
use rand::Rng;
use regex::{ Regex };

const FILE_NOT_FOUND_ERROR: &str = "File not found!";
const OUT_OF_BOUNDS: [u8; 1] = [32];

static mut STRING_PARSE: bool = false;
static mut REGEX: Option<Regex> = None;
static mut FILEPATH: String = String::new();

#[derive(Debug)]
struct Stack {
  stack: Vec<i32>,
  return_stack: Vec<usize>
}

#[allow(unused)]
impl Stack {
  fn new() -> Self { Stack { stack: Vec::new(), return_stack: Vec::new() } }
  fn pop(&mut self) -> i32 { self.stack.pop().unwrap_or(0) }
  fn push(&mut self, item: i32) { self.stack.push(item) }
  fn is_empty(&self) -> bool { self.stack.is_empty() }
  fn length(&self) -> usize { self.stack.len() }
  fn peek(&self) -> Option<&i32> { self.stack.last() }

  fn pop_r(&mut self) -> usize { self.return_stack.pop().unwrap_or(0) }
  fn push_r(&mut self, item: usize) { self.return_stack.push(item) }
}

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() <= 1 { panic!("No File Argument Supplied!") }
  let grid: Vec<Vec<u8>>;

  unsafe {
    FILEPATH = args[1].clone();
    REGEX.replace(Regex::new(r"\b[0-9]+?\b").unwrap());

    let source = fs::read_to_string(&FILEPATH).expect(FILE_NOT_FOUND_ERROR);
    grid = parse_grid(source);
  }
  execute_grid(grid);
}

fn parse_grid(source: String) -> Vec<Vec<u8>> {
  let mut grid: Vec<Vec<u8>> = vec![vec![]];
  let mut row = 0;
  for value in source.chars() {
    match value {
      '\n' => { 
        grid.push(vec![]);
        row += 1; 
      },
      other => {
        grid.get_mut(row).unwrap().push(other as u8);
      }
    }
  }
  grid
}

fn execute_grid(mut grid: Vec<Vec<u8>>) {
  let mut x: usize = 0;
  let mut y: usize = 0;
  let mut stack = Stack::new();
  let mut next_move = Move::Right;

  while next_move != Move::End {
    let code: u8 = grid.get(y).unwrap_or(&OUT_OF_BOUNDS.to_vec()).get(x).unwrap_or(&32).clone();
    next_move = execute(&code, &mut stack, &mut grid, &x, &y, next_move);

    match next_move {
      Move::Up => { if y == 0 { panic!("Tried to Move out of Grid at ({}, {})!", x, y) } y -= 1 },
      Move::Down => y += 1,
      Move::Left => { if x == 0 { panic!("Tried to Move out of Grid at ({}, {})!", x, y) } x -= 1 },
      Move::Right => x += 1,
      Move::Jump { cords, move_after } => {
        x = cords[0];
        y = cords[1];
        next_move = *move_after;
      },
      _ => ()
    }
  }
}

#[allow(unused_assignments)]
fn execute(code: &u8, stack: &mut Stack, grid: &mut Vec<Vec<u8>>, x: &usize, y: &usize,
   old_move: Move) -> Move {

  unsafe {
    if STRING_PARSE && *code != 34 { stack.push(*code as i32); old_move }
    else { match_code(code, stack, grid, x, y, old_move) }
  }
}

fn match_code(code: &u8, stack: &mut Stack, grid: &mut Vec<Vec<u8>>, x: &usize, y: &usize,
   old_move: Move) -> Move {

  match code {
    // Movement
    32 => old_move, // SPACE
    94 => Move::Up,    // ^ 
    118 => Move::Down, // v 
    60 => Move::Left,  // < 
    62 => Move::Right, // > 
    95 => if stack.peek().is_some() && stack.pop() != 0 { Move::Left } else { Move::Right } // _ 
    124 => if stack.peek().is_some() && stack.pop() != 0 { Move::Up } else { Move::Down }   // |
    63 => match rand::thread_rng().gen_range(0..5) { // ?
      1 => Move::Up,
      2 => Move::Down,
      3 => Move::Left,
      4 => Move::Right,
      _ => old_move
    }
    35 => jump(x, y, old_move), // #

    // Arithemetic
    43 => { // +
      let a = stack.pop();
      let b = stack.pop();
      stack.push(a + b);
      old_move
    },
    45 => { // -
      let a = stack.pop();
      let b = stack.pop();
      stack.push(b - a);
      old_move
    },
    42 => { // *
      let a = stack.pop();
      let b = stack.pop();
      stack.push(a * b);
      old_move
    },
    47 => { // /
      let a = stack.pop();
      let b = stack.pop();
      stack.push(b / a);
      old_move
    },
    37 => { // %
      let a = stack.pop();
      let b = stack.pop();
      stack.push(b % a);
      old_move
    },
    33 => { // !
      let a = stack.pop();
      if a == 0 { stack.push(1) } else { stack.push(0) }
      old_move
    },
    96 => { // `
      let a = stack.pop();
      let b = stack.pop();
      if b > a { stack.push(1) } else { stack.push(0) }
      old_move
    }

    // Stack Manipulation
    48..=57 => { // 0 - 9
      stack.push(*code as i32 - 48);
      old_move
    },
    34 => { // "
      unsafe { STRING_PARSE = !STRING_PARSE; }
      old_move
    },
    58 => { // :
      let a = stack.peek().unwrap_or(&0);
      stack.push(*a);
      old_move
    },
    92 => { // \
      let a = stack.pop();
      let b = stack.pop();
      stack.push(a);
      stack.push(b);
      old_move
    },
    36 => { stack.pop(); old_move }, // $

    // I/O
    46 => { // .
      let flag = stack.pop() as u8;
      let b_b = stack.pop() as u8;
      let g_b = stack.pop() as u8;
      let r_b = stack.pop() as u8;
      let b = stack.pop() as u8;
      let g = stack.pop() as u8;
      let r = stack.pop() as u8;

      let mut s = stack.pop().to_string().normal();
      s = format_string(s, flag);

      print!("{}", s.truecolor(r, g, b).on_truecolor(r_b, g_b, b_b));
      old_move
    },
    44 => unsafe { // ,
      let flag = stack.pop() as u8;
      let b_b = stack.pop() as u8;
      let g_b = stack.pop() as u8;
      let r_b = stack.pop() as u8;
      let b = stack.pop() as u8;
      let g = stack.pop() as u8;
      let r = stack.pop() as u8;

      let mut s = (char::from_u32_unchecked(stack.pop() as u32)).to_string().normal();
      s = format_string(s, flag);

      print!("{}", s.truecolor(r, g, b).on_truecolor(r_b, g_b, b_b));
      old_move
    },
    38 => unsafe { // &
      let mut input_text = String::new();
      stdin().read_line(&mut input_text).expect("Failed to read from stdin");
      let trimmed = input_text.trim();
      let str_match = REGEX.as_ref().unwrap().find(trimmed);
      if str_match.is_some() { println!("{}", str_match.unwrap().as_str()) }
      old_move
    },
    126 => { // ~
      let mut input_text = String::new();
      stdin().read_line(&mut input_text).expect("Failed to read from stdin");
      stack.push(*input_text.as_bytes().get(0).expect("Invalid Input Detected") as i32);
      old_move
    }

    // Misc
    103 => { // g
      let y = stack.pop();
      let x = stack.pop();
      let v = *grid.get(y as usize).unwrap_or(&OUT_OF_BOUNDS.to_vec()).get(x as usize).unwrap_or(&32) as i32;
      match v {
          32 => stack.push(0),
          other => stack.push(other)
      }
      old_move
    },
    112 => unsafe { // s
      let y = stack.pop();
      let x = stack.pop();
      let v = stack.pop();
      if x < 0 || y < 0 { panic!("Tried to write outside boundry!") }

      // Update grid object
      if y as usize > grid.len() - 1 { grid.append(&mut vec![vec![]; y as usize - grid.len() + 1]) }
      let row = grid.get_mut(y as usize).unwrap();
      if x as usize >= row.len() { row.append(&mut vec![32; x as usize - row.len() + 1]) }
      row[x as usize] = v as u8;

      // Update file
      let mut s = String::new();
      for row in grid {
        for ch in row {
          s.push_str(&(*ch as char).to_string());
        }
        s.push('\n');
      }
      s.remove(s.len() - 1);
      let mut output = File::create(&FILEPATH).expect(FILE_NOT_FOUND_ERROR);
      output.write_all(s.as_bytes()).expect("Unknown Error Occured While Writing to File!");
      
      old_move
    }
    64 => Move::End, // @

    _ => panic!("Unrecognized Command Character found!") 
  }
}

fn jump(x: &usize, y: &usize, old_move: Move) -> Move {
  match old_move {
      Move::Up => Move::Jump { cords: [*x, y - 2], move_after: Box::new(Move::Up) },
      Move::Down => Move::Jump { cords: [*x, y + 2], move_after: Box::new(Move::Down) },
      Move::Left => Move::Jump { cords: [x - 2, *y], move_after: Box::new(Move::Left) },
      Move::Right => Move::Jump { cords: [x + 2, *y], move_after: Box::new(Move::Right) },
      Move::Jump { cords: _, move_after: next_move } => jump(x, y, *next_move),
      _ => unreachable!()
  }
}

fn format_string(s: ColoredString, flag: u8) -> ColoredString {
  match flag {
    1 => s.bold(),
    2 => s.italic(),
    3 => s.underline(),
    4 => s.strikethrough(),
    5 => s.bold().italic(),
    6 => s.bold().underline(),
    7 => s.bold().strikethrough(),
    8 => s.strikethrough().italic(),
    9 => s.strikethrough().underline(),
    10 => s.italic().underline(),
    11 => s.bold().italic().underline(),
    12 => s.bold().italic().strikethrough(),
    13 => s.bold().underline().strikethrough(),
    14 => s.italic().underline().strikethrough(),
    15 => s.bold().italic().underline().strikethrough(),
    _ => s
  }
}

#[derive(PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Jump{ cords: [usize; 2], move_after: Box<Move> },
    End
}