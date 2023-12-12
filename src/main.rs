use std::{process::Command, thread, time::Duration};
use midir::{MidiOutput, MidiOutputConnection};
use regex::Regex;


fn press_recording(outport: &mut MidiOutputConnection) {
    outport.send(&[0xB0, 0x11, 0x01]).unwrap();
    thread::sleep(Duration::from_millis(100));
    outport.send(&[0xB0, 0x11, 0x00]).unwrap();
}

fn main() {
    let midi_out = MidiOutput::new("Zoom Recorder").unwrap();
    let ports = midi_out.ports();
    let outport = ports.iter().find(|p| midi_out.port_name(p).unwrap() == "RODECaster Duo").expect("MIDI port not found");

    let mut outport = midi_out.connect(outport, "zoom-recorder").unwrap();


    let mut avail = true;
    let re = Regex::new(r"(.*zoom)(.*key [0-9]+)").unwrap();

    loop {
        let output_ps = Command::new("ps")
            .arg("x")
            .output()
            .expect("Failed to execute ps");

        let output = String::from_utf8_lossy(&output_ps.stdout);

        if re.is_match(&output) && avail {
            let code = re.captures(&output).and_then(|cap| cap.get(2)).map_or("", |m| m.as_str());
            println!("Meeting ID: {}", code);
            println!("Meeting Status: ON");
            press_recording(&mut outport);
            println!("Recording Status: ON");
            avail = false;
        } else if !re.is_match(&output) && !avail {
            println!("Avail.");
            println!("Meeting Status: OFF");
            press_recording(&mut outport);
            println!("Recording Status: OFF");
            avail = true;
        } else {
            thread::sleep(Duration::from_millis(200));
        }
    }
}
