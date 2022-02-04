//! Inside Module Docs
use std::sync::mpsc;
use std::thread;
use std::time;
use std::env;
use std::io;
use std::io::Read;
use figlet_rs::FIGfont;

mod helper;

const DEFAULT: &str = "5:00";

///
/// Function Docs
///
fn main() {
    let (mut hours, mut mins, mut secs) = helper::convert_seconds(env::args()
        .skip(1)
        .next()
        .unwrap_or(DEFAULT.to_string())
        .split(':')
        .map(|x| x.parse::<u32>())
        .rev()
        .enumerate()
        .fold(0u32, |a, (i, v)| -> u32 {
            match v {
                Ok(v) => a + 60u32.pow(i as u32) * v,
                Err(_) => 0
            }
        }));

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || draw(rx));

    if let Err(e) = tx.send((hours, mins, secs)) {
        panic!("{e}");
    }
    while hours > 0 || mins > 0 || secs > 0 {
        thread::sleep(time::Duration::from_secs(1));
        match (&mut hours, &mut mins, &mut secs) {
            (h,m,s) if *h>0 && *m==0 && *s==0 => {
                *h -= 1;
                *m = 59;
                *s = 59;
            },
            (_,m,s) if *m>0 && *s==0 => {
                *m -= 1;
                *s = 59;
            },
            (_,_,s) => {
                *s -= 1;
            }
        }
        if let Err(e) = tx.send((hours, mins, secs)) {
            panic!("{e}");
        }
    }

    // Wait for user input before closing app.
    io::stdin().read(&mut [0u8]).unwrap();
}

/// Responsible for determining when to render the next frame.
fn draw(rx: mpsc::Receiver<(u32, u32, u32)>) {
    let (mut swd, mut sht) = (0, 0);
    let (mut hours, mut mins, mut secs) = (0, 0, 0);

    loop {
        let mut should_render = false;

        if let Ok((new_hours, new_mins, new_secs)) = rx.try_recv() {
            should_render = true;
            hours = new_hours;
            mins = new_mins;
            secs = new_secs;
        }

        let (wd, ht) = term_size::dimensions().unwrap_or((50,28));
        if swd != wd || sht != ht {
            should_render = true;
            swd = wd;
            sht = ht;
        }

        if should_render {
            render(wd, ht, hours, mins, secs);
        }
        thread::sleep(time::Duration::from_millis(250));
    }
}

/// Responsible for rendering time to terminal
fn render(wd: usize, ht: usize, hours: u32, mins: u32, secs: u32) {
    let ffont = FIGfont::standand().unwrap();

    if let Some(msg) = ffont.convert(format!("{:02}:{:02}:{:02}", hours, mins, secs).as_str()) {
        let mut m_w = msg.to_string().lines().map(|s| s.len()).max().unwrap_or(1);
        let mut m_h = msg.height as usize;
        if m_w > wd || m_h > ht {
            m_w = 0;
            m_h = 0;
        }
        let midpoint = ((wd - m_w)/2, (ht - m_h)/2);

        print!("{esc}[2J", esc="\x1b");

        for (i, l) in msg.to_string().lines().enumerate() {
            print!("{esc}[{ht};{w}H", esc="\x1b", w=midpoint.0, ht=midpoint.1+i);
            println!("{}", l);
        }
    }
}
