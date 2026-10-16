//! Grade de parâmetros do laboratório do capítulo 4 (uso interno).
use stochology_book::cap04;

fn main() {
    for passos in [1_600usize, 3_200, 6_400] {
        for desconto in [0.98_f64, 0.99, 0.995] {
            let c = cap04::moeda_movel(2026, passos, 200, desconto);
            println!(
                "passos={} desconto={} fixo={:.4} esquecimento={:.4} largura_final_fixo={:.4} largura_esq={:.4} razao={:.2}",
                passos,
                desconto,
                c.cobertura_estatica,
                c.cobertura_descontada,
                c.largura_estatica_final,
                c.largura_descontada_final,
                c.largura_descontada_final / c.largura_estatica_final
            );
        }
    }
}
