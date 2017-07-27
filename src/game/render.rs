use std::io::Cursor;

use byteorder::WriteBytesExt;

use ::error::Error;
use super::buffer;

pub fn weird_gradient (
    buffer: &mut buffer::Image,
    x_offset: u32,
    y_offset: u32
) -> Result<(), Error> {
    let mut cursor = Cursor::new(&mut *buffer.memory);
    for p_y in 0..buffer.height {
        for p_x in 0..buffer.width {
            let red   = 0x00;
            let blue  = ((p_x + x_offset) % 0xFF) as u8;
            let green = ((p_y + y_offset) % 0xFF) as u8;
            let alpha = 0xFF;
            cursor.write_u8(red)?;
            cursor.write_u8(green)?;
            cursor.write_u8(blue)?;
            cursor.write_u8(alpha)?;
        }
    }
    Ok(())
}

pub fn player (
    buffer: &mut buffer::Image,
    player_x: u32,
    player_y: u32
) -> Result<(), Error> {
    let top = player_y;
    let bottom = player_y + 10;
    // TODO(CJS): We could probably assert! and use get_unchecked eventaully...
    // let end = buffer.memory.len();
    for x in player_x..player_x+10 {
        let x_offset = x * buffer.bytes_per_pixel;
        let mut y_offset = top * buffer.pitch;
        let mut pixel;
        for _ in top..bottom {
            pixel = (x_offset + y_offset) as usize;
            // HACK(CJS): until I get byteorder to write here efficently i'll
            // just hack in the for bytes manually in LE order...
            // let color = 0xFFFFFFFF; // white
            if let Some(p) = buffer.memory.get_mut(pixel + 0) { *p = 0xFF; }
            if let Some(p) = buffer.memory.get_mut(pixel + 1) { *p = 0xFF; }
            if let Some(p) = buffer.memory.get_mut(pixel + 2) { *p = 0xFF; }
            if let Some(p) = buffer.memory.get_mut(pixel + 3) { *p = 0xFF; }
            y_offset += buffer.pitch;
        }
    }
    Ok(())
}
