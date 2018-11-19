use chrono::prelude::*;
use cute_dnd_dice::Roll;
use regex::Regex;

command!(page(_ctx, msg, arg) {
	println!("{}: >>page ({})", msg.timestamp, msg.author);

	// 引数をSCP-JPのURLと合わせてreply
	let scp_jp = "http://ja.scp-wiki.net/";
	let dir = arg.single::<String>().unwrap_or_else(|_| "".to_string());
	let url = format!("{}{}", &scp_jp, &dir);
	if let Err(why) = msg.reply(&url) {
		println!(">>page msg err: {}", why);
	};
});

command!(sandbox(_ctx, msg, arg) {
	println!("{}: >>sandbox ({})", msg.timestamp, msg.author);

	// 引数をSCP-JPサンドボックスのURLと合わせてreply
	let sb = "http://scp-jp-sandbox2.wikidot.com/";
	let dir = arg.single::<String>().unwrap_or_else(|_| "start".to_string());
	// TODO: サンドボックスのURLとして非正規なユーザー名が送られてきた場合の対応
	let url = format!("{}{}", &sb, &dir);
	if let Err(why) = msg.reply(&url) {
		println!(">>sandbox msg err: {}", why);
	};
});

command!(draft(_ctx, msg, arg) {
	println!("{}: >>draft ({})", msg.timestamp, msg.author);

	// 引数を取り出す
	let date = match arg.single::<String>() {
		Ok(str) => str,
		// 引数が無かったら、当日のdraftReserveページを送信
		Err(_) 	=> Local::today().format("%Y-%m-%d").to_string()
	};

	// 引数がYYYY-MM-DDの形でなかったら、エラーメッセージ送信
	let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
	// 正規表現のmatchが遅いらしいので要改善
	if re.is_match(&date) {
		if let Err(why) = msg.reply("no match 'YYYY-MM-DD'") {
			println!(">>draft msg err: {}", why);
		};
	} else {	// 引数がYYYY-MM-DDの形だったら、該当日のdraftReserveページを送信
		let nijiru = "http://njr-sys.net/draftReserve/";
		let url = format!("{}{}", nijiru, date);
		if let Err(why) = msg.reply(&url) {
			println!(">>draft msg err: {}", why);
		};
	}
});

command!(dice(_ctx, msg, arg) {
	println!("{}: >>dice ({})", msg.timestamp, msg.author);

	// 引数を取り出す
	let d = arg.single::<String>().unwrap_or_else(|_| "invalid".to_string());

	match Roll::from_str(&d) {
		// 引数が'AdX'の形だったら'AdX'を送信
		Ok(roll) => {
			let reply = format!("{} = {}", &d, &roll.roll());
			if let Err(why) = msg.reply(&reply) {
				println!(">>dice err {}", why);
			};
		},
		// 引数がなかったらエラーメッセージを送信
		// 引数が'AdX'の形でなかったらエラーメッセージを送信
		Err(_) => {
			let reply = format!("invalid arg {}", &d);
			if let Err(why) = msg.reply(&reply) {
				println!(">>dice err {}", why);
			};
		}
	}
});

// TODO: draft-status
// TODO: tale (search)
