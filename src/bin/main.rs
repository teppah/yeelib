use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use yeelib_rs::{YeeClient, Light, YeeError};
use yeelib_rs::fields::{Transition, PowerStatus, Rgb};


fn main() -> Result<(), YeeError> {
    let client = YeeClient::new()?;
    let mut res: Vec<Light> = loop {
        let lights = client.find_lights(Duration::from_secs(1));
        // sometimes, it doesn't find anything, so rerun
        if lights.len() == 0 {
            println!("zero");
        } else {
            break lights;
        }
    };
    let light = res.get_mut(0).unwrap();
    println!("{:?}", light);


    loop {
        println!("dark");
        let result = light.set_bright(1, Transition::sudden());
        println!("{:?}", result);
        sleep(Duration::from_secs(4));

        println!("bright");
        let result = light.set_bright(100, Transition::smooth(Duration::from_millis(1000)).unwrap());
        println!("{:?}", result);
        sleep(Duration::from_secs(4));

        println!("off");
        let result = light.set_power(PowerStatus::Off, Transition::sudden());
        println!("{:?}", result);
        sleep(Duration::from_secs(4));

        println!("on");
        let result = light.set_power(PowerStatus::On, Transition::sudden());
        println!("{:?}", result);
        sleep(Duration::from_secs(4));

        println!("2700");
        light.set_ct_abx(2700, Transition::sudden());
        sleep(Duration::from_secs(4));

        println!("6500");
        light.set_ct_abx(6600, Transition::sudden());
        sleep(Duration::from_secs(4));

        light.set_rgb(Rgb::new(30, 40, 50), Transition::Sudden);
        sleep(Duration::from_secs(3));
        println!("rgb");
        let result = light.set_rgb(Rgb::new(240, 40, 180), Transition::Sudden);
        println!("{:?}", result);
        sleep(Duration::from_secs(4));
        light.toggle()?;
        println!("{}", light.power());
        sleep(Duration::from_secs(4));
    }

    // let bright = "{\"id\":23,\"method\":\"set_ct_abx\",\"params\":[2700,\"smooth\",400]}\r\n";
    // let dark = "{\"id\":23,\"method\":\"set_ct_abx\",\"params\":[6500,\"smooth\",400]}\r\n";
    // let mut ceiling_light = TcpStream::connect("192.168.2.24:55443")?;
    // loop {
    //     let mut buf = [0u8; 128];
    //     ceiling_light.write(bright.as_bytes())?;
    //     ceiling_light.read(&mut buf)?;
    //     println!("{}", String::from_utf8(buf.to_vec()).unwrap());
    //     sleep(Duration::from_secs(5));
    //
    //     let mut buf = [0u8; 128];
    //     ceiling_light.write(dark.as_bytes())?;
    //     ceiling_light.read(&mut buf)?;
    //     println!("{}", String::from_utf8(buf.to_vec()).unwrap());
    //     sleep(Duration::from_secs(5));
    // }

    Ok(())
}