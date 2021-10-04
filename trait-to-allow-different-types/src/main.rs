use trait_to_allow_different_types::*;

fn main() {

	let screen = Screen {
		components: vec![
			Box::new(
				SelectBox { 
					width: 10,
					height: 5,
					options: vec![
						String::from("Option 1"),
						String::from("Option 2"),
						String::from("Option 3"),
					]
				}
			),
			Box::new(
				Button {
					width: 10,
					height: 5,
					label: String::from("Pretty btn")
				}
			),
			Box::new(String::from(" :0"))
		]
	};

	screen.run();

}

