/// 実際には、"quit"コマンドを使っても接続が解除される訳ではない
/// これはcontext.quit()が単に一旦Shardを停止させるというだけで、WebSocketを切断するという訳ではないから
/// TODO: Discordから切断(WebSocket)し、本体プログラムを終了するように変更
command!(quit(context, message) {
	println!("{}: >>quit ({})", message.timestamp, message.author);
	println!("quit charrenge", );
	if let Err(why) = message.reply("quit…") {
		println!(">>quit msg error: {}", why);
	}
	context.quit();
});
