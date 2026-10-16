//! Capítulo 3 — probabilidade sem mistério: árvores em frequências naturais e decibans.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arvore {
    pub pessoas: u64,
    pub doentes: u64,
    pub positivos_doentes: u64,
    pub saudaveis: u64,
    pub falsos_positivos: u64,
    pub positivos: u64,
}

impl Arvore {
    /// Fração de doentes entre os positivos: a resposta que importa ao paciente.
    pub fn fracao_doentes_entre_positivos(&self) -> f64 {
        self.positivos_doentes as f64 / self.positivos as f64
    }

    /// "uma em cada quantas positivas" está de fato doente.
    pub fn uma_em_cada(&self) -> f64 {
        self.positivos as f64 / self.positivos_doentes as f64
    }
}

/// Constrói a árvore de 10 mil (ou n) pessoas em frequências naturais.
pub fn arvore(pessoas: u64, prevalencia: f64, sensibilidade: f64, falso_positivo: f64) -> Arvore {
    let doentes = (pessoas as f64 * prevalencia).round() as u64;
    let saudaveis = pessoas - doentes;
    let positivos_doentes = (doentes as f64 * sensibilidade).round() as u64;
    let falsos_positivos = (saudaveis as f64 * falso_positivo).round() as u64;
    Arvore {
        pessoas,
        doentes,
        positivos_doentes,
        saudaveis,
        falsos_positivos,
        positivos: positivos_doentes + falsos_positivos,
    }
}

/// Força da evidência em decibans: dez vezes o logaritmo decimal da razão.
pub fn decibans(razao: f64) -> f64 {
    10.0 * razao.log10()
}

/// Atualiza uma crença anterior aplicando razões de verossimilhança.
pub fn posterior_por_chances(priori: f64, razoes: &[f64]) -> f64 {
    let mut chances = priori / (1.0 - priori);
    for r in razoes {
        chances *= r;
    }
    chances / (1.0 + chances)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn mamografia_da_uma_em_dez() {
        let a = arvore(10_000, 0.01, 0.90, 0.09);
        assert_eq!(a.doentes, 100);
        assert_eq!(a.positivos_doentes, 90);
        assert_eq!(a.falsos_positivos, 891);
        assert_eq!(a.positivos, 981);
        assert!((a.uma_em_cada() - 10.9).abs() < 0.05, "{}", a.uma_em_cada());
    }

    #[test]
    fn dez_de_razao_valem_dez_decibans() {
        assert!((decibans(10.0) - 10.0).abs() < 1e-12);
        assert!((decibans(4.2) - 6.2324).abs() < 1e-3);
    }

    #[test]
    fn dois_exames_independentes_contra_exames_correlacionados() {
        let independentes = posterior_por_chances(0.01, &[10.0, 10.0]);
        let correlacionados = posterior_por_chances(0.01, &[10.0, 4.2]);
        assert!((independentes - 0.5025).abs() < 0.002, "{}", independentes);
        assert!((correlacionados - 0.2977).abs() < 0.002, "{}", correlacionados);
    }
}
