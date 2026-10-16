use std::io;

use cucopa::cuco;

pub fn main() -> io::Result<()> {
    cuco("cu", std::env::args().skip(1).collect())
}
