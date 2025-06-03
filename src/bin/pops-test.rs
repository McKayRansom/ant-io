use ant_io::game::Game;

pub fn main() {
    let mut game: Game = Game::new();

    for i in 0..2897 {
        game.update_sim();
        assert!(!game.pillbugs.is_empty(), "Pillbugs died out at gen {}", i);
        assert!(game.pillbugs.len() < 500, "Pillbugs overpoped at gen {}", i);

        assert!(!game.spiders.is_empty(), "Spiders died out at gen {}", i);
        assert!(game.spiders.len() < 500, "Spiders overpoped at gen {}", i);

        assert!(
            !game.ant_colonies[0].workers.is_empty(),
            "Ants[0] died out at gen {}",
            i
        );
        assert!(
            game.ant_colonies[0].workers.len() < 500,
            "Ants[0] overpoped at gen {}",
            i
        );

        assert!(
            !game.ant_colonies[1].workers.is_empty(),
            "Ants[1] died out at gen {}",
            i
        );
        assert!(
            game.ant_colonies[1].workers.len() < 500,
            "Ants[1] overpoped at gen {}",
            i
        );
    }

    println!("Final pillbugs: {}", game.pillbugs.len());
    println!("Final spiders: {}", game.spiders.len());
    println!("Final ant[0]: {}", game.ant_colonies[0].workers.len());
    println!("Final ant[1]: {}", game.ant_colonies[1].workers.len());

}
