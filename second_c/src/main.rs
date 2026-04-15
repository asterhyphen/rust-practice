//local mods
mod booyer_moore;
mod cli_fruitsal;
mod shortest_path;
fn main() {
    shortest_path::shortest_path();
    cli_fruitsal::cli_fruitsal();
    booyer_moore::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
}
