extern crate howl;

use howl::Discord;

fn main() {
	let discord = Discord::new("", "").expect("login failed");
	
	let test_zone = howl::ChannelId("".into());
	//println!("{:#?}", discord.send_message(&test_zone, "Hello from Rust", &[], "", false));
	discord.broadcast_typing(&test_zone).expect("broadcast typing failed");
	
	let mut connection = discord.connect().expect("connect failed");
	let closed;
	loop {
		match connection.recv_event() {
			Ok(howl::Event::Closed(n)) => { closed = n; break },
			Ok(howl::Event::Ready { .. }) => { println!("Ready."); continue },
			Ok(howl::Event::MessageCreate(message)) => {
				let (server, channel) = match connection.state.find_public_channel(&message.channel_id) {
					Some(info) => info,
					None => { println!("PRIVMSG {:?}", message); continue },
				};
				println!("[{} #{}] {}: {}", server.name, channel.name, message.author.name, message.content);
				if message.content == "/test" || message.content.starts_with("/test ") {
					warn(discord.send_message(&message.channel_id, "This is a reply to the test.", &[], "", false));
				}
			}
			Ok(other) => println!("--- {:?}", other),
			Err(err) => println!("Recv error: {:?}", err),
		}
	}
	println!("Closed upon with status '{}'", closed);
	
	//discord.logout().expect("logout failed");
}

fn warn<T, E: ::std::fmt::Debug>(result: Result<T, E>) {
	match result {
		Ok(_) => {},
		Err(err) => println!("[warn] {:?}", err)
	}
}
