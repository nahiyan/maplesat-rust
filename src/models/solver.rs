use super::{lbool::LBool, lit::Lit, var_data::VarData};

pub struct Solver<'a> {
    pub model: Vec<LBool>,
    pub conflict: Vec<Lit>,
    pub verbosity: i32,

    #[cfg(any(feature = "bh_chb", feature = "bh_lrb"))]
    pub step_size: f64,
    #[cfg(any(feature = "bh_chb", feature = "bh_lrb"))]
    pub step_size_dec: f64,
    #[cfg(any(feature = "bh_chb", feature = "bh_lrb"))]
    pub min_step_size: f64,

    #[cfg(feature = "bh_vsids")]
    pub var_decay: f64,

    #[cfg(not(feature = "lbd_based_clause_deletion"))]
    pub clause_decay: f64,

    pub random_var_freq: f64,
    pub random_seed: f64,
    pub luby_restart: bool,
    pub ccmin_mode: i32, // Controls conflict clause minimization (0=none, 1=basic, 2=deep).
    pub phase_saving: i32, // Controls the level of phase saving (0=none, 1=limited, 2=full).
    pub rnd_pol: bool,   // Use random polarities for branching heuristics.
    pub rnd_init_act: bool, // Initialize variable activities with a small random value.
    pub garbage_frac: f64, // The fraction of wasted memory allowed before a garbage collection is triggered.

    pub restart_first: i32, // The initial restart limit.                                                                (default 100)
    pub restart_inc: f64, // The factor with which the restart limit is multiplied in each restart.                    (default 1.5)
    pub learntsize_factor: f64, // The initial limit for learnt clauses is a factor of the original clauses.                (default 1 / 3)
    pub learntsize_inc: f64, // The limit for learnt clauses is multiplied with this factor each restart.                 (default 1.1)

    pub learntsize_adjust_start_confl: i32,
    pub learntsize_adjust_inc: f64,

    // Statistics: (read-only member variable)
    pub solves: u64,
    pub starts: u64,
    pub decisions: u64,
    pub rnd_decisions: u64,
    pub propagations: u64,
    pub conflicts: u64,
    pub dec_vars: u64,
    pub clauses_literals: u64,
    pub learnts_literals: u64,
    pub max_literals: u64,
    pub tot_literals: u64,

    pub lbd_calls: u64,
    pub lbd_seen: Vec<u64>,
    pub picked: Vec<u64>,
    pub conflicted: Vec<u64>,

    #[cfg(feature = "almost_conflict")]
    pub almost_conflicted: Vec<u64>,
    #[cfg(feature = "anti_exploration")]
    pub canceled: Vec<u64>,

    #[cfg(feature = "bh_chb")]
    pub last_conflict: Vec<u64>,
    #[cfg(feature = "bh_chb")]
    pub action: i32,
    #[cfg(feature = "bh_chb")]
    pub reward_multiplier: f64,

    pub total_actual_rewards: Vec<f64>,
    pub total_actual_count: Vec<i32>,

    // TODO: Implement the stuff below
    // Solver state:
    pub ok: bool, // If FALSE, the constraints are already unsatisfiable. No part of the solver state may be used!
    // pub clauses: Vec<CRef>, // List of problem clauses.
    // pub learnts: Vec<CRef>, // List of learnt clauses.
    // #[cfg(not(feature = "lbd_based_clause_deletion"))]
    pub cla_inc: f64,       // Amount to bump next clause with.
    pub activity: Vec<f64>, // A heuristic measurement of the activity of a variable.
    pub var_inc: f64,       // Amount to bump next variable with.
    // pub watches: OccLists<Lit, Vec<Watcher>, WatcherDeleted>, // 'watches[lit]' is a list of constraints watching 'lit' (will go there if literal becomes true).
    pub assigns: Vec<LBool>,       // The current assignments.
    pub polarity: Vec<bool>,       // The preferred polarity of each variable.
    pub decision: Vec<bool>, // Declares if a variable is eligible for selection in the decision heuristic.
    pub trail: Vec<Lit>, // Assignment stack; stores all assignments made in the order they were made.
    pub trail_lim: Vec<i32>, // Separator indices for different decision levels in 'trail'.
    pub vardata: Vec<VarData<'a>>, // Stores reason and level for each variable.
    pub qhead: i32, // Head of queue (as index into the trail -- no more explicit propagation queue in MiniSat).
    pub simpDB_assigns: i32, // Number of top-level assignments since last execution of 'simplify()'.
    pub simpDB_props: i64, // Remaining number of propagations that must be made before next execution of 'simplify()'.
    pub assumptions: Vec<Lit>, // Current set of assumptions provided to solve by the user.
    // pub order_heap: Heap<VarOrderLt>, // A priority queue of variables ordered with respect to the variable activity.
    pub progress_estimate: f64, // Set by 'search()'.
    pub remove_satisfied: bool, // Indicates whether possibly inefficient linear scan for satisfied clauses should be performed in 'simplify'.

    // ClauseAllocator     ca;

    // Temporaries (to reduce allocation overhead). Each variable is prefixed by the method in which it is
    // used, exept 'seen' wich is used in several places.
    pub seen: Vec<char>,
    pub analyze_stack: Vec<Lit>,
    pub analyze_toclear: Vec<Lit>,
    pub add_tmp: Vec<Lit>,

    pub max_learnts: f64,
    pub learntsize_adjust_confl: f64,
    pub learntsize_adjust_cnt: i32,

    // Resource constraints:
    pub conflict_budget: i64,    // -1 means no budget.
    pub propagation_budget: i64, // -1 means no budget.
    pub asynch_interrupt: bool,
}
