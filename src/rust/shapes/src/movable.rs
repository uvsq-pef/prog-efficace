pub trait Movable {
    /// Déplace l'élément d'un décalage donné
    fn move_by(&mut self, dx: f64, dy: f64);

    /// Retourne true si l'élément se trouve à la position en paramètre.
    fn is_at(&self, x: f64, y: f64) -> bool;
}
