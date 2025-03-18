use rand::{Rng, SeedableRng};

use super::{
    clause::{Clause, UNDEF_CLAUSE},
    lbool::LBool,
    lit::Lit,
    var::Var,
    var_data::VarData,
};

pub struct Solver<'a> {
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

    // TODO: Implement the stuff below
    // Solver state:
    ok: bool, // If FALSE, the constraints are already unsatisfiable. No part of the solver state may be used!
    // clauses: Vec<CRef>, // List of problem clauses.
    // learnts: Vec<CRef>, // List of learnt clauses.
    // #[cfg(not(feature = "lbd_based_clause_deletion"))]
    cla_inc: f64,       // Amount to bump next clause with.
    activity: Vec<f64>, // A heuristic measurement of the activity of a variable.
    var_inc: f64,       // Amount to bump next variable with.
    // watches: OccLists<Lit, Vec<Watcher>, WatcherDeleted>, // 'watches[lit]' is a list of constraints watching 'lit' (will go there if literal becomes true).
    assigns: Vec<LBool>,       // The current assignments.
    polarity: Vec<bool>,       // The preferred polarity of each variable.
    decision: Vec<bool>, // Declares if a variable is eligible for selection in the decision heuristic.
    trail: Vec<Lit>, // Assignment stack; stores all assignments made in the order they were made.
    trail_lim: Vec<i32>, // Separator indices for different decision levels in 'trail'.
    vardata: Vec<VarData<'a>>, // Stores reason and level for each variable.
    qhead: i32, // Head of queue (as index into the trail -- no more explicit propagation queue in MiniSat).
    simpDB_assigns: i32, // Number of top-level assignments since last execution of 'simplify()'.
    simpDB_props: i64, // Remaining number of propagations that must be made before next execution of 'simplify()'.
    assumptions: Vec<Lit>, // Current set of assumptions provided to solve by the user.
    // order_heap: Heap<VarOrderLt>, // A priority queue of variables ordered with respect to the variable activity.
    progress_estimate: f64, // Set by 'search()'.
    remove_satisfied: bool, // Indicates whether possibly inefficient linear scan for satisfied clauses should be performed in 'simplify'.

    // ClauseAllocator     ca;

    // Temporaries (to reduce allocation overhead). Each variable is prefixed by the method in which it is
    // used, exept 'seen' wich is used in several places.
    seen: Vec<char>,
    analyze_stack: Vec<Lit>,
    analyze_toclear: Vec<Lit>,
    add_tmp: Vec<Lit>,

    max_learnts: f64,
    learntsize_adjust_confl: f64,
    learntsize_adjust_cnt: i32,

    // Resource constraints:
    conflict_budget: i64,    // -1 means no budget.
    propagation_budget: i64, // -1 means no budget.
    asynch_interrupt: bool,
}

impl<'a> Solver<'a> {
    fn decision_level(&self) -> i32 {
        return self.trail_lim.len() as i32;
    }

    // TODO: Compare with MapleSAT's efficiency
    fn lit_value(&self, p: &Lit) -> LBool {
        let value = p.var().value(&self.assigns);
        if value == LBool::Undefined {
            LBool::Undefined
        } else {
            let value = value == LBool::True;
            if !p.sign() {
                LBool::from(!value)
            } else {
                LBool::from(value)
            }
        }
    }

    fn n_vars(&self) -> usize {
        self.vardata.len()
    }

    fn set_decision_var(&mut self, v: Var, b: bool) {
        let v_usize: usize = v.into();
        if b && !self.decision[v_usize] {
            self.dec_vars += 1;
        } else if !b && self.decision[v_usize] {
            self.dec_vars -= 1;
        }

        self.decision[v_usize] = b;
        // TODO: self.insert_var_order(v);
    }

    fn new_var(&mut self, sign: bool, dvar: bool) -> Var {
        let new_var = Var::from(self.n_vars());
        // TODO: Add watches
        // watches  .init(mkLit(v, false));
        // watches  .init(mkLit(v, true ));
        self.assigns.push(LBool::Undefined);
        self.vardata.push(VarData {
            reason: &UNDEF_CLAUSE,
            level: 0,
        });
        // Note: Behaves differently from MapleSAT
        self.activity.push(if self.rnd_init_act {
            let mut rng = rand::rngs::StdRng::seed_from_u64(self.random_seed as u64);
            rng.random_range(0.0..=1.0) * 0.00001
        } else {
            0.0
        });
        self.seen.push('\0');
        self.polarity.push(sign);
        self.decision.push(false);
        self.trail.reserve(1);
        self.lbd_seen.push(0);
        self.picked.push(0);
        self.conflicted.push(0);
        #[cfg(feature = "almost_conflict")]
        self.almost_conflicted.push(0);
        #[cfg(feature = "anti_exploration")]
        self.canceled.push(0);
        #[cfg(feature = "bh_chb")]
        self.last_conflict.push(0);
        self.total_actual_rewards.push(0.0);
        self.total_actual_count.push(0);
        self.set_decision_var(new_var.clone(), dvar);

        new_var
    }

    fn satisfied(&self, c: &Clause) -> bool {
        c.iter().any(|lit| self.lit_value(lit) == LBool::True)
    }

    // TODO: Finish
    // fn unchecked_enqueue(&mut self, p: Lit, from: CRef) {
    //     assert!(self.value(p) == LBool::Undef);
    //     self.picked[p.var() as usize] = self.conflicts;

    //     #[cfg(feature = "anti_exploration")]
    //     {
    //         let age = self.conflicts - self.canceled[p.var() as usize];
    //         if age > 0 {
    //             let decay = 0.95f64.powi(age as i32);
    //             self.activity[p.var() as usize] *= decay;
    //             if self.order_heap.in_heap(p.var() as usize) {
    //                 self.order_heap.increase(p.var() as usize);
    //             }
    //         }
    //     }

    //     self.conflicted[p.var() as usize] = 0;

    //     #[cfg(feature = "almost_conflict")]
    //     {
    //         self.almost_conflicted[p.var() as usize] = 0;
    //     }

    //     self.assigns[p.var() as usize] = LBool::from(!p.sign());
    //     self.vardata[p.var() as usize] = VarData::new(from, self.decision_level());
    //     self.trail.push(p);
    // }

    // TODO: Finish
    // fn add_clause(&self, ps: Vec<Lit>) -> bool {
    //     assert!(self.decision_level() == 0);
    //     if !self.ok {
    //         return false;
    //     }

    //     // Check if clause is satisfied and remove false/duplicate literals:
    //     // TODO: ps.sort();
    //     let mut p = Lit::default();
    //     let mut j = 0;
    //     for i in 0..ps.len() {
    //         if self.value(ps[i]) == LBool::True || ps[i] == !p {
    //             return true;
    //         } else if self.value(ps[i]) != LBool::False && ps[i] != p {
    //             ps[j] = ps[i];
    //             p = ps[i];
    //             j += 1;
    //         }
    //     }
    //     ps.truncate(j);

    //     if ps.is_empty() {
    //         self.ok = false;
    //         return false;
    //     } else if ps.len() == 1 {
    //         self.unchecked_enqueue(ps[0]);
    //         self.ok = self.propagate() == CRef::Undef;
    //         return self.ok;
    //     } else {
    //         let cr = self.ca.alloc(&ps, false);
    //         self.clauses.push(cr);
    //         self.attach_clause(cr);
    //     }

    //     true
    // }
}
