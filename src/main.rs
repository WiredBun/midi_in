use alsa::rawmidi::Rawmidi;
use alsa::Direction;

use std::ffi::CString;
use std::ffi::CStr;

use std::io::Read;
use std::io::Write;
use std::fs::File;


const CLOCK: u8 = 248;

//struct key_map 
//{

//}


fn main() -> Result<(), Box<dyn std::error::Error>> 
{

// create CString
    let cstring = CString::new("hw:1,0,0")
        .expect("CString::new failed");

// Borrow as &CStr for function call
    let device_cstr: &CStr = cstring.as_c_str();
    println!("Reading Midi input");
    

// open midi byte stream by refering to the cstring as the device, invoking alsa::direction to decided whether input or output, and bool for blocking
    let midi_in = Rawmidi::open(device_cstr,Direction::Capture, false)
            .expect("Could not read raw midi byte stream from device");    
    
    let mut io = midi_in.io();

//Create an array usng unsigned 8 bit intergers with a fixed length of 1024
    let mut buffer: Vec<u8>;


    let mut keyin = File::create("keyin")
        .expect("unable to create file 'keyin'");
    
//loop through reading the midi byte stream
       
    loop
    {  
        buffer = vec![0];

        let bytes_read = io.read(&mut buffer)?;           
        println!("Read {} bytes from midi device: {:?}", bytes_read, &buffer[..bytes_read]);
     
        if buffer[0] == CLOCK
        { 
            println!("Clock Signal");
     
        } 
            else
            {
                
                println!("Key");
                keyin.write_all(&buffer)
                    .expect("Failed to write to file");
            }

    }    
    return Ok(())
}
