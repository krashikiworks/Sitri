command!(ping(_context, message) {
	println!("{}: >>ping ({})", message.timestamp, message.author);
	let _ = message.reply("pong!");
});
