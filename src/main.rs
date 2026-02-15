// Importe la fonction `app` depuis la bibliothèque de notre crate.
use bevy_game_of_life::app;

fn main() {
    // Appelle la fonction de configuration partagée et lance l'application.
    // Cela fonctionne pour le bureau. La version web est lancée via `run()` dans lib.rs.
    app().run();
}
