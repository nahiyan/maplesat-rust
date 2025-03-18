mod models;

fn main() {
    // Ensure that only one branching heuristic is enabled
    fn ensure_one_bh_enabled() {
        let features = [
            cfg!(feature = "bh_chb"),
            cfg!(feature = "bh_lrb"),
            cfg!(feature = "bh_vsids"),
        ];

        let enabled_features = features.iter().filter(|&&f| f).count();

        if enabled_features > 1 {
            panic!("You can only enable one branching heuristic at once.");
        }
    }

    ensure_one_bh_enabled();
}
