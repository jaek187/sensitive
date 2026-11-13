mod conversion;

fn main() {
    let dpi = 1600u32;
    let cm = 55f32;
    let game = conversion::Game::Valorant;
    let game_name = game.name();
    let sens = game.cm_to_sens(cm, dpi);

    println!("{cm}cm @ {dpi}dpi -> {sens} in {game_name}");
}
