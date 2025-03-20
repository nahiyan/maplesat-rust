use rand::{Rng, SeedableRng};

use crate::models::{
    clause::{Clause, UNDEF_CLAUSE},
    lbool::LBool,
    lit::Lit,
    solver::Solver,
    var::Var,
    var_data::VarData,
};

impl<'a> Solver<'a> {
    pub fn new() -> Self {
        Self {
            model: vec![],
            conflict: vec![],
            verbosity: 0,
            step_size: 0.40,
            step_size_dec: 0.000001,
            min_step_size: 0.06,
            random_var_freq: 0.02,
            random_seed: 0.0,
            luby_restart: true,
            ccmin_mode: 2,
            phase_saving: 2,
            rnd_pol: false,
            rnd_init_act: false,
            garbage_frac: 0.20,
            restart_first: 100,
            restart_inc: 2.0,
            learntsize_factor: 1.0 / 3.0,
            learntsize_inc: 1.1,
            learntsize_adjust_start_confl: 100,
            learntsize_adjust_inc: 1.5,
            solves: 0,
            starts: 0,
            decisions: 0,
            rnd_decisions: 0,
            propagations: 0,
            conflicts: 0,
            dec_vars: 0,
            clauses_literals: 0,
            learnts_literals: 0,
            max_literals: 0,
            tot_literals: 0,
            lbd_calls: 0,
            lbd_seen: vec![],
            picked: vec![],
            conflicted: vec![],
            almost_conflicted: vec![],
            canceled: vec![],
            total_actual_rewards: vec![],
            total_actual_count: vec![],
            ok: true,
            cla_inc: 1.0,
            activity: vec![],
            var_inc: 1.0,
            assigns: vec![],
            polarity: vec![],
            decision: vec![],
            trail: vec![],
            trail_lim: vec![],
            vardata: vec![],
            qhead: 0,
            simpDB_assigns: -1,
            simpDB_props: 0,
            assumptions: vec![],
            progress_estimate: 0.0,
            remove_satisfied: true,
            seen: vec![],
            analyze_stack: vec![],
            analyze_toclear: vec![],
            add_tmp: vec![],
            max_learnts: 0.0,
            learntsize_adjust_confl: 0.0,
            learntsize_adjust_cnt: 0,
            conflict_budget: -1,
            propagation_budget: -1,
            asynch_interrupt: false,
        }
    }

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

    pub fn num_vars(&self) -> usize {
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
        let new_var = Var::from(self.num_vars());
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
    pub fn add_clause(&mut self, mut ps: Vec<Lit>) -> bool {
        assert!(self.decision_level() == 0);
        if !self.ok {
            return false;
        }

        // Check if clause is satisfied and remove false/duplicate literals:
        // TODO: ps.sort();
        let mut p = Lit::default();
        let mut j = 0;
        for i in 0..ps.len() {
            if ps[i].value(&self.assigns) == LBool::True || ps[i] == !p {
                return true;
            } else if ps[i].value(&self.assigns) != LBool::False && ps[i] != p {
                ps[j] = ps[i];
                p = ps[i];
                j += 1;
            }
        }
        ps.truncate(j);

        if ps.is_empty() {
            self.ok = false;
            return false;
        } else if ps.len() == 1 {
            // TODO: self.unchecked_enqueue(ps[0]);
            // TODO: self.ok = self.propagate() == CRef::Undef;
            return self.ok;
        } else {
            // TODO: let cr = self.ca.alloc(&ps, false);
            // TODO: self.clauses.push(cr);
            // TODO: self.attach_clause(cr);
        }

        true
    }
}
