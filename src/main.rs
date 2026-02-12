use chainofresp::{Chain, CheckFileType, Context, DisplayImage, ReadImage};

fn main() {
    let mut ctx = Context {
        file: "input.jpg".to_string(),
        ..Default::default()
    };

    let chain = Chain::new()
        .add(CheckFileType)
        .add(ReadImage)
        .add(DisplayImage);

    if let Err(err) = chain.run(&mut ctx) {
        eprintln!("chain failed: {}", err);
    }
}
