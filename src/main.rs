use alsa::rawmidi::Rawmidi;
use alsa::Direction;

use std::ffi::CString;
use std::ffi::CStr;

use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> 
{

// create CString
    let cstring = CString::new("hw:0,0,0")
        .expect("CString::new failed");

// Borrow as &CStr for function call
    let device_cstr: &CStr = cstring.as_c_str();
    println!("Reading Midi input");
    

// open midi byte stream by refering to the cstring as the device, invoking alsa::direction to decided whether input or output, and bool for blocking
        let mut midi_in = Rawmidi::open(device_cstr,Direction::Capture, false)
            .expect("Could not read raw midi byte stream from device");    
    
        let mut io = midi_in.io();

//Create an array usng unsigned 8 bit intergers with a fixed length of 1024
        let mut buffer = [0u8; 4];


//loop through reading the midi byte stream
       
    loop
    {  
        let bytes_read = io.read(&mut buffer)?;

        println!("Midi Packet");           
        println!("Read {} bytes from midi device: {:?}", bytes_read, &buffer[..bytes_read]);
    }    
    return Ok(())
}
