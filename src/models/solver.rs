use super::{lbool::LBool, lit::Lit};

pub struct Solver {
    model: Vec<LBool>,
    conflict: Vec<Lit>,
    verbosity: i32,

    #[cfg(any(feature = "bh_chb", feature = "bh_lrb"))]
    step_size: f64,
    #[cfg(any(feature = "bh_chb", feature = "bh_lrb"))]
    step_size_dec: f64,
    #[cfg(any(feature = "bh_chb", feature = "bh_lrb"))]
    min_step_size: f64,

    #[cfg(feature = "bh_vsids")]
    var_decay: f64,

    #[cfg(not(feature = "lbd_based_clause_deletion"))]
    clause_decay: f64,

    random_var_freq: f64,
    random_seed: f64,
    luby_restart: bool,
    ccmin_mode: i32, // Controls conflict clause minimization (0=none, 1=basic, 2=deep).
    phase_saving: i32, // Controls the level of phase saving (0=none, 1=limited, 2=full).
    rnd_pol: bool,   // Use random polarities for branching heuristics.
    rnd_init_act: bool, // Initialize variable activities with a small random value.
    garbage_frac: f64, // The fraction of wasted memory allowed before a garbage collection is triggered.

    restart_first: i32, // The initial restart limit.                                                                (default 100)
    restart_inc: f64, // The factor with which the restart limit is multiplied in each restart.                    (default 1.5)
    learntsize_factor: f64, // The initial limit for learnt clauses is a factor of the original clauses.                (default 1 / 3)
    learntsize_inc: f64, // The limit for learnt clauses is multiplied with this factor each restart.                 (default 1.1)

    learntsize_adjust_start_confl: i32,
    learntsize_adjust_inc: f64,

    // Statistics: (read-only member variable)
    solves: u64,
    starts: u64,
    decisions: u64,
    rnd_decisions: u64,
    propagations: u64,
    conflicts: u64,
    dec_vars: u64,
    clauses_literals: u64,
    learnts_literals: u64,
    max_literals: u64,
    tot_literals: u64,

    lbd_calls: u64,
    lbd_seen: Vec<u64>,
    picked: Vec<u64>,
    conflicted: Vec<u64>,

    #[cfg(feature = "almost_conflict")]
    almost_conflicted: Vec<u64>,
    #[cfg(feature = "anti_exploration")]
    canceled: Vec<u64>,

    #[cfg(feature = "bh_chb")]
    last_conflict: Vec<u64>,
    #[cfg(feature = "bh_chb")]
    action: i32,
    #[cfg(feature = "bh_chb")]
    reward_multiplier: f64,

    total_actual_rewards: Vec<f64>,
    total_actual_count: Vec<i32>,

    // Solver state:
    //
    ok: bool, // If FALSE, the constraints are already unsatisfiable. No part of the solver state may be used!
    // clauses: Vec<CRef>, // List of problem clauses.
    // learnts: Vec<CRef>, // List of learnt clauses.
    // #[cfg(not(feature = "lbd_based_clause_deletion"))]
    // cla_inc: f64, // Amount to bump next clause with.
    // activity: Vec<f64>, // A heuristic measurement of the activity of a variable.
    // var_inc: f64, // Amount to bump next variable with.
    // watches: OccLists<Lit, Vec<Watcher>, WatcherDeleted>, // 'watches[lit]' is a list of constraints watching 'lit' (will go there if literal becomes true).
    // assigns: Vec<LBool>,                                  // The current assignments.
    // polarity: Vec<bool>,          // The preferred polarity of each variable.
    // decision: Vec<bool>, // Declares if a variable is eligible for selection in the decision heuristic.
    // trail: Vec<Lit>, // Assignment stack; stores all assignments made in the order they were made.
    // trail_lim: Vec<i32>, // Separator indices for different decision levels in 'trail'.
    // vardata: Vec<VarData>, // Stores reason and level for each variable.
    // qhead: i32, // Head of queue (as index into the trail -- no more explicit propagation queue in MiniSat).
    // simpDB_assigns: i32, // Number of top-level assignments since last execution of 'simplify()'.
    // simpDB_props: i64, // Remaining number of propagations that must be made before next execution of 'simplify()'.
    // assumptions: Vec<Lit>, // Current set of assumptions provided to solve by the user.
    // order_heap: Heap<VarOrderLt>, // A priority queue of variables ordered with respect to the variable activity.
    progress_estimate: f64, // Set by 'search()'.
    remove_satisfied: bool, // Indicates whether possibly inefficient linear scan for satisfied clauses should be performed in 'simplify'.
}
