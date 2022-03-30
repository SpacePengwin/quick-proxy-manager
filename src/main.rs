use std::env;
use std::process::Command;
use clap::Arg;
use clap::Command as CMDArg;

fn main() {
    let matches = CMDArg::new("Proxy Manager")
    .version("0.1.0")
    .arg(Arg::new("mode")
    .short('m')
    .long("mode")
    .takes_value(true)
    .possible_values(&["enable", "disable"]))
    .arg(Arg::new("adapter")
    .short('a')
    .long("adapter")
    .takes_value(true)
    .required(false)
    .default_value("None"))
    .arg(Arg::new("domain")
    .short('d')
    .long("domain")
    .takes_value(true)
    .required(false)
    .default_value("None"))
    .arg(Arg::new("port")
    .short('p')
    .long("port")
    .takes_value(true)
    .required(false)
    .default_value("None"))
    .get_matches();
    let mode_value = matches.value_of("mode").unwrap();
    let adapter_value = matches.value_of("adapter").unwrap();
    if env::consts::OS == "windows" {
        match mode_value {
            "disable" => disable_proxy_windows(),
            "enable" => enable_proxy_windows("localhost", "40400"),
            _ => println!("Nothing special")

        }


    } else if env::consts::OS == "macos" {
        match mode_value {
            "disable" => disable_proxy_macos(&adapter_value),
            "enable" => enable_proxy_macos(&adapter_value, "localhost", "40400"),
            _ => println!("Nothing special")

        }

    } else {
        println!("{}", env::consts::OS);
    }
}
fn enable_proxy_macos(adapter: &str, domain: &str, port: &str) {
    // Set proxy to specific domain:port
    let output = Command::new("networksetup")
    .arg("-setsocksfirewallproxy")
    .arg(&adapter)
    .arg(&domain)
    .arg(&port).output()
    .expect("Failed to enable proxy on macos.");

    let status = output.status;
    if status.success() {
        println!("Proxy Enabled!")
    } else {
        println!("Failed to enable!")
    }
}
fn disable_proxy_macos(adapter: &str) {
        let output = Command::new("networksetup")
        .arg("-setsocksfirewallproxystate")
        .arg(&adapter)
        .arg("off").output()
        .expect("Failed to disable proxy");

        if output.status.success() {
            println!("Proxy Disabled!");
        } else {
            println!("Failed to disable!");
        }
}

fn enable_proxy_windows(domain: &str, port: &str) {
    let args_cmd = format!("socks={}:{}", &domain, &port);
    let command = format!("Set-ItemProperty -Path 'HKCU:/Software/Microsoft/Windows/CurrentVersion/Internet Settings' -name ProxyServer -Value \"{}\"", &args_cmd);
    let output = Command::new("powershell.exe")
    .arg(&command).output().expect("Failed to enable proxy");
    
    if output.status.success() {
        println!("Proxy Enabled!")
    } else {
        println!("Failed to enable!")
    }

    let command_two = format!("Set-ItemProperty -Path 'HKCU:/Software/Microsoft/Windows/CurrentVersion/Internet Settings' -name ProxyEnable -Value 1");
    let output_two = Command::new("powershell.exe")
    .arg(&command_two).output().expect("Failed to enable proxy 2");

    if output_two.status.success() {
        println!("Proxy Enabled!")
    } else {
        println!("Failed to enable!")
    }

}

fn disable_proxy_windows() {
    let command = String::from("Set-ItemProperty -Path 'HKCU:/Software/Microsoft/Windows/CurrentVersion/Internet Settings' -name ProxyServer -Value \"\"");
    let command_two = String::from("Set-ItemProperty -Path 'HKCU:/Software/Microsoft/Windows/CurrentVersion/Internet Settings' -name ProxyEnable -Value 0");

    let output = Command::new("powershell.exe")
    .arg(&command).output().expect("Failed to disable proxy");
    let output_two = Command::new("powershell.exe")
    .arg(&command_two).output().expect("Failed to disable proxy 2");

    if output.status.success() {
        println!("Proxy Enabled!")
    } else {
        println!("Failed to enable!")
    }

    if output_two.status.success() {
        println!("Proxy Enabled!")
    } else {
        println!("Failed to enable!")
    }
}