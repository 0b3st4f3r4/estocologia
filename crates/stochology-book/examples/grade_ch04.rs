//! Parameter grid of the chapter 4 laboratory (internal use).
use stochology_book::ch04;

fn main() {
    for steps in [1_600usize, 3_200, 6_400] {
        for discount in [0.98_f64, 0.99, 0.995] {
            let c = ch04::moving_coin(2026, steps, 200, discount);
            println!(
                "steps={} discount={} fixed={:.4} forgetting={:.4} final_fixed_width={:.4} discounted_width={:.4} ratio={:.2}",
                steps,
                discount,
                c.static_coverage,
                c.discounted_coverage,
                c.static_width_final,
                c.discounted_width_final,
                c.discounted_width_final / c.static_width_final
            );
        }
    }
}
