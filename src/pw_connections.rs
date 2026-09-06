use std::process::Command;

pub type Channel = String;

pub fn connect_to_channels() {
    let pw_link_output = pw_link_output();
    let (c_channels_l, c_channels_r) = get_connected_channels(&pw_link_output);
    let (s_channels_l, s_channels_r) = get_sink_channels(&pw_link_output);

    for channel in s_channels_l {
        if !c_channels_l.contains(&channel) {
            connect_channel_l(&channel);
        }
    }

    for channel in s_channels_r {
        if !c_channels_r.contains(&channel) {
            connect_channel_r(&channel);
        }
    }
}

fn pw_link_output() -> String {
    let pw_links = Command::new("pw-link")
        .arg("-l")
        .output()
        .expect("failed to pw-link -l");

    let link_string = String::from_utf8(pw_links.stdout).expect("corrupt string");

    return link_string;
}

fn get_connected_channels(pw_link_output: &String) -> (Vec<Channel>, Vec<Channel>) {
    let mut connected_links = pw_link_output
        .lines()
        .skip_while(|l| !(l.contains("suono") & !l.starts_with(" ")))
        .take_while(|l| l.starts_with("  ") | l.contains("suono"))
        .filter(|l| l.starts_with("  |<- "))
        .map(|l| l.strip_prefix("  |<- ").unwrap().to_string())
        .collect::<Vec<Channel>>();

    let half_len = connected_links.len() / 2;
    let right_connected_links = connected_links.drain(half_len..).collect::<Vec<Channel>>();
    let left_connected_links = connected_links;
    return (left_connected_links, right_connected_links);
}

fn get_sink_channels(pw_link_output: &String) -> (Vec<Channel>, Vec<Channel>) {
    let mut connected_links = pw_link_output
        .lines()
        .skip_while(|l| !(l.contains("sink:playback") & !l.starts_with(" ")))
        .take_while(|l| l.starts_with("  ") | l.contains("sink:playback"))
        .filter(|l| l.starts_with("  |<- "))
        .map(|l| l.strip_prefix("  |<- ").unwrap().to_string())
        .collect::<Vec<Channel>>();

    let half_len = connected_links.len() / 2;
    let right_connected_links = connected_links.drain(half_len..).collect::<Vec<Channel>>();
    let left_connected_links = connected_links;
    return (left_connected_links, right_connected_links);
}

fn connect_channel_l(channel: &Channel) {
    let _output = Command::new("pw-link")
        .arg(channel)
        .arg("alsa_capture.suono:input_FL")
        .output();
}

fn connect_channel_r(channel: &Channel) {
    let _output = Command::new("pw-link")
        .arg(channel)
        .arg("alsa_capture.suono:input_FR")
        .output();
}
