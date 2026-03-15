//local mods
mod shortest_path;
mod cli_fruitsal;
mod booyer_moore;
fn main() {
    shortest_path::shortest_path();
    cli_fruitsal::cli_fruitsal();
    booyer_moore::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
}