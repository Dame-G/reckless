//! Search constants exposed for SPSA tuning.
//!
//! Each entry is `type name: default, min, max, c_end;`. Without the `spsa`
//! feature every parameter is a `const fn` returning its default, so the
//! engine is identical to one with the literal written in place. With the
//! feature, each parameter becomes a UCI spin option, and the `spsa` command
//! prints the list in OpenBench's input format:
//! `name, int, default, min, max, c_end, r_end`.

#[allow(unused_macros)]
#[cfg(not(feature = "spsa"))]
macro_rules! define {
    {$($type:ident $name:ident: $value:expr, $min:expr, $max:expr, $c_end:expr; )*} => {
        $(pub const fn $name() -> $type {
            $value
        })*
    };
}

#[cfg(feature = "spsa")]
macro_rules! define {
    {$($type:ident $name:ident: $value:expr, $min:expr, $max:expr, $c_end:expr; )*} => {
        /// Sets a parameter by name. Returns false for a name that is not a tunable
        /// or a value that does not parse.
        pub fn set_parameter(name: &str, value: &str) -> bool {
            match name {
                $(stringify!($name) => match value.parse::<f64>() {
                    // OpenBench and fastchess may send fractional values; round them.
                    Ok(v) => {
                        unsafe { parameters::$name = (v.round() as $type).clamp($min, $max) };
                        true
                    }
                    Err(_) => false,
                },)*
                _ => false,
            }
        }

        pub fn print_options() {
            $(println!("option name {} type spin default {} min {} max {}", stringify!($name), $value, $min, $max);)*
        }

        /// The tunables in OpenBench's SPSA input format, with the current values as defaults.
        pub fn print_openbench() {
            $(println!("{}, int, {}, {}, {}, {}, 0.002", stringify!($name), unsafe { parameters::$name }, $min, $max, $c_end);)*
        }

        $(pub fn $name() -> $type {
            unsafe { parameters::$name }
        })*

        #[allow(non_upper_case_globals)]
        mod parameters {
            $(pub static mut $name: $type = $value;)*
        }
    };
}

define! {
    // Aspiration windows
    i32 asp_delta: 23, 10, 40, 1.5;
    i32 asp_fail_low_widen: 26, 10, 50, 2.0;
    i32 asp_fail_high_widen: 60, 30, 100, 3.0;

    // History bonus on a TT cutoff
    i32 tt_cut_bonus_mul: 190, 100, 300, 10.0;
    i32 tt_cut_bonus_max: 1691, 800, 2500, 80.0;

    // Quiet ordering from the static eval difference
    i32 eval_order_mul: 812, 400, 1200, 40.0;

    // Hindsight reductions
    i32 hindsight_reduction: 2249, 1200, 3400, 110.0;
    i32 hindsight_eval_delta: 57, 25, 90, 3.0;

    // Razoring
    i32 razor_base: 237, 100, 400, 12.0;
    i32 razor_quad: 254, 120, 400, 12.0;

    // Reverse futility pruning
    i32 rfp_quad: 1140, 600, 1700, 55.0;
    i32 rfp_lin: 22, 0, 50, 2.0;
    i32 rfp_improvement: 120, 50, 200, 6.0;
    i32 rfp_correction: 669, 300, 1000, 33.0;

    // Null move pruning
    i32 nmp_margin: 337, 170, 500, 17.0;
    i32 nmp_depth_mul: 9, 0, 20, 1.0;
    i32 nmp_reduction_base: 4407, 2500, 6500, 220.0;
    i32 nmp_reduction_depth: 265, 130, 400, 13.0;

    // ProbCut
    i32 probcut_margin: 254, 130, 400, 12.0;
    i32 probcut_improving: 85, 30, 150, 4.0;

    // Singular extensions
    i32 se_double_margin: 195, 100, 300, 10.0;
    i32 se_triple_margin: 230, 120, 350, 11.0;
    i32 ldse_margin: 25, 5, 50, 2.0;

    // Late move pruning
    i32 lmp_base: 2818, 1400, 4200, 140.0;
    i32 lmp_quad: 1351, 700, 2000, 67.0;

    // Futility pruning
    i32 fp_mul: 79, 40, 120, 4.0;
    i32 fp_offset: 127, 50, 200, 6.0;
    i32 bnfp_mul: 84, 40, 130, 4.0;

    // History pruning
    i32 history_prune: 948, 500, 1500, 47.0;

    // Late move reductions
    i32 lmr_base: 269, 130, 400, 13.0;
    i32 lmr_quiet: 2432, 1200, 3600, 120.0;
    i32 lmr_noisy: 1687, 800, 2500, 84.0;
    i32 lmr_quiet_history: 179, 90, 270, 9.0;
    i32 lmr_cut_node: 1852, 900, 2800, 92.0;
    i32 lmr_tt_pv: 333, 150, 500, 16.0;

    // History updates after a best move
    i32 quiet_bonus_mul: 184, 90, 280, 9.0;
    i32 quiet_bonus_max: 1742, 900, 2600, 87.0;
    i32 quiet_malus_mul: 171, 90, 260, 8.0;
}
