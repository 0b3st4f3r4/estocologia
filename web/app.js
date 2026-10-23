/* ==========================================================================
   Stochology Protocol v1.0 — Interactive Epistemological Apparatus (app.js)
   Zero external dependencies. Mirrors the mathematical engines in crates/*.
   ========================================================================== */

(function () {
  "use strict";

  /* ------------------------------------------------------------------------
     1. Dark Mode Toggle (modern-web-guidance: dark-mode & dark-mode-toggle)
     ------------------------------------------------------------------------ */
  const metaScheme = document.querySelector('meta[name="color-scheme"]');
  const themeBtn = document.getElementById("theme-toggle");
  const themeLabel = document.getElementById("theme-toggle-label");
  const systemDarkQuery = window.matchMedia("(prefers-color-scheme: dark)");

  function getStoredScheme() {
    try {
      const val = localStorage.getItem("color-scheme");
      return val === "light" || val === "dark" ? val : null;
    } catch {
      return null;
    }
  }

  function getSystemScheme() {
    return systemDarkQuery.matches ? "dark" : "light";
  }

  function getRenderedScheme() {
    return getStoredScheme() ?? getSystemScheme();
  }

  function syncThemeUI() {
    const stored = getStoredScheme();
    if (metaScheme) {
      metaScheme.content = stored ?? "light dark";
    }
    if (themeLabel) {
      const rendered = getRenderedScheme();
      const modeTag = stored ? (rendered === "dark" ? "DARK*" : "LIGHT*") : (rendered === "dark" ? "AUTO·DARK" : "AUTO·LIGHT");
      themeLabel.textContent = modeTag;
    }
  }

  if (themeBtn) {
    themeBtn.addEventListener("click", () => {
      const currentRendered = getRenderedScheme();
      const target = currentRendered === "dark" ? "light" : "dark";
      const systemScheme = getSystemScheme();

      try {
        if (target === systemScheme) {
          // Returning to system preference: remove stored override rather than pinning
          localStorage.removeItem("color-scheme");
        } else {
          localStorage.setItem("color-scheme", target);
        }
      } catch {
        // Fallback if localStorage is blocked: still update <meta> in-memory
        if (metaScheme) {
          metaScheme.content = target === systemScheme ? "light dark" : target;
        }
      }
      syncThemeUI();
    });
  }

  // Listen to OS preference changes to refresh UI label without wiping stored override
  systemDarkQuery.addEventListener("change", () => {
    syncThemeUI();
  });

  // Cross-tab synchronization
  window.addEventListener("storage", (e) => {
    if (e.key === "color-scheme" || e.key === null) {
      syncThemeUI();
    }
  });

  syncThemeUI();

  /* ------------------------------------------------------------------------
     2. Bilingual Switcher (PT / EN)
     ------------------------------------------------------------------------ */
  const langBtn = document.getElementById("lang-toggle");
  let currentLang = "pt";
  try {
    const savedLang = localStorage.getItem("stochology-lang");
    if (savedLang === "en" || savedLang === "pt") {
      currentLang = savedLang;
    }
  } catch {
    // ignore storage errors
  }

  function applyLanguage(lang) {
    currentLang = lang;
    document.documentElement.lang = lang === "pt" ? "pt-BR" : "en";
    if (langBtn) {
      langBtn.textContent = lang === "pt" ? " PT / EN" : " EN / PT";
      langBtn.setAttribute(
        "aria-label",
        lang === "pt" ? "Mudar idioma para Inglês" : "Switch language to Portuguese"
      );
    }
    document.querySelectorAll("[data-i18n-pt][data-i18n-en]").forEach((el) => {
      const text = lang === "pt" ? el.getAttribute("data-i18n-pt") : el.getAttribute("data-i18n-en");
      if (text !== null) {
        el.textContent = text;
      }
    });
    updateDecisionSimulator();
    updatePerformativeSimulator();
  }

  if (langBtn) {
    langBtn.addEventListener("click", () => {
      const next = currentLang === "pt" ? "en" : "pt";
      try {
        localStorage.setItem("stochology-lang", next);
      } catch {
        // ignore
      }
      applyLanguage(next);
    });
  }

  /* ------------------------------------------------------------------------
     3. Searchable Hidden Content (hidden="until-found" + beforematch)
     ------------------------------------------------------------------------ */
  const cardsAccordion = document.getElementById("epistemic-cards-tabs");
  if (cardsAccordion) {
    const panels = Array.from(cardsAccordion.querySelectorAll(".card-tab-panel"));
    const buttons = Array.from(cardsAccordion.querySelectorAll(".tab-btn"));

    function activatePanel(targetPanelId) {
      panels.forEach((panel) => {
        if (panel.id === targetPanelId) {
          panel.removeAttribute("hidden");
        } else {
          panel.setAttribute("hidden", "until-found");
        }
      });
      buttons.forEach((btn) => {
        const controls = btn.getAttribute("aria-controls");
        btn.setAttribute("aria-expanded", controls === targetPanelId ? "true" : "false");
      });
    }

    buttons.forEach((btn) => {
      btn.addEventListener("click", () => {
        const targetId = btn.getAttribute("aria-controls");
        if (targetId) {
          activatePanel(targetId);
        }
      });
    });

    // Synchronize UI state when Find-in-Page matches text inside a hidden="until-found" panel
    cardsAccordion.addEventListener("beforematch", (e) => {
      const matchedPanel = e.target.closest(".card-tab-panel");
      if (!matchedPanel) return;
      panels.forEach((panel) => {
        if (panel !== matchedPanel) {
          panel.setAttribute("hidden", "until-found");
        }
      });
      buttons.forEach((btn) => {
        const controls = btn.getAttribute("aria-controls");
        btn.setAttribute("aria-expanded", controls === matchedPanel.id ? "true" : "false");
      });
    });

    // Fallback for browsers that do not support onbeforematch / hidden="until-found"
    if (!("onbeforematch" in HTMLElement.prototype)) {
      document.querySelectorAll('[hidden="until-found"]').forEach((el) => {
        el.removeAttribute("hidden");
      });
      buttons.forEach((btn) => {
        btn.setAttribute("aria-expanded", "true");
      });
    }
  }

  /* ------------------------------------------------------------------------
     4. Simulator 1: Cost-Loss Boundary p* = C / (eta * L), EVPI & Natural Tree
     ------------------------------------------------------------------------ */
  const inputCost = document.getElementById("sim-cost");
  const inputLoss = document.getElementById("sim-loss");
  const inputEta = document.getElementById("sim-eta");
  const inputProb = document.getElementById("sim-prob");

  const valCost = document.getElementById("val-cost");
  const valLoss = document.getElementById("val-loss");
  const valEta = document.getElementById("val-eta");
  const valProb = document.getElementById("val-prob");

  const outPStar = document.getElementById("out-pstar");
  const outEvpi = document.getElementById("out-evpi");
  const outAct = document.getElementById("out-act");
  const outCromwell = document.getElementById("out-cromwell");
  const outNaturalMemo = document.getElementById("out-natural-memo");
  const costLossSvg = document.getElementById("cost-loss-svg");

  const CARD_PRESETS = {
    sepsis: { cost: 1800, loss: 34000, eta: 1.0, prob: 0.05 },
    queue: { cost: 2400, loss: 30000, eta: 1.0, prob: 0.145 },
    credit: { cost: 1500, loss: 25000, eta: 1.0, prob: 0.095 },
  };

  document.querySelectorAll("[data-card-preset]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const key = btn.getAttribute("data-card-preset");
      const preset = CARD_PRESETS[key];
      if (!preset) return;
      document.querySelectorAll("[data-card-preset]").forEach((b) =>
        b.setAttribute("aria-pressed", b === btn ? "true" : "false")
      );
      if (inputCost) inputCost.value = String(preset.cost);
      if (inputLoss) inputLoss.value = String(preset.loss);
      if (inputEta) inputEta.value = String(preset.eta);
      if (inputProb) inputProb.value = String(preset.prob);
      updateDecisionSimulator();
    });
  });

  function updateDecisionSimulator() {
    if (!inputCost || !inputLoss || !inputEta || !inputProb) return;
    const C = parseFloat(inputCost.value);
    const L = parseFloat(inputLoss.value);
    const eta = parseFloat(inputEta.value);
    const p = parseFloat(inputProb.value);

    if (valCost) valCost.textContent = C.toFixed(0);
    if (valLoss) valLoss.textContent = L.toFixed(0);
    if (valEta) valEta.textContent = eta.toFixed(2);
    if (valProb) valProb.textContent = p.toFixed(3);

    const pStar = C / (eta * L);
    const protectAction = p > pStar;
    // Expected Value of Perfect Information at prior p
    const evpi = protectAction ? (1 - p) * C : p * eta * L;
    const cromwellValid = p >= 0.001 && p <= 0.999 && pStar > 0 && pStar < 1;

    if (outPStar) outPStar.textContent = pStar.toFixed(5);
    if (outEvpi) outEvpi.textContent = evpi.toFixed(1);
    if (outAct) {
      outAct.textContent = protectAction
        ? currentLang === "pt"
          ? "PROTEGER (p > p*)"
          : "PROTECT (p > p*)"
        : currentLang === "pt"
          ? "NÃO PROTEGER (p ≤ p*)"
          : "DO NOT PROTECT (p ≤ p*)";
      outAct.style.color = protectAction ? "var(--status-warn)" : "var(--status-ok)";
    }
    if (outCromwell) {
      outCromwell.textContent = cromwellValid ? "APPROVED (Φ4)" : "BLOCKED (Cromwell/Φ4)";
      outCromwell.style.color = cromwellValid ? "var(--status-ok)" : "var(--status-err)";
    }

    // Gigerenzer Natural Frequency tree (N = 1000, sensitivity = 0.85, FPR = 0.08)
    const N = 1000;
    const eventCount = Math.round(N * p);
    const nonEventCount = N - eventCount;
    const tp = Math.round(eventCount * 0.85);
    const fp = Math.round(nonEventCount * 0.08);
    const totalAlerts = Math.max(1, tp + fp);
    const ppvPct = ((tp / totalAlerts) * 100).toFixed(1);

    if (outNaturalMemo) {
      outNaturalMemo.textContent =
        currentLang === "pt"
          ? `Árvore de Frequências Naturais (Gigerenzer, N = ${N} unidades independentes): Em ${N} turnos/coortes, ocorrem ${eventCount} eventos críticos (${tp} detectados) e ${nonEventCount} não-eventos (${fp} falsos alarmes). De ${totalAlerts} alertas positivos, ${tp} são eventos reais (VPP = ${ppvPct}%). Regra econômica: agir se p (${p.toFixed(3)}) > p* (${pStar.toFixed(5)}).`
          : `Gigerenzer Natural Frequency Tree (N = ${N} independent units): Out of ${N} shifts/cohorts, ${eventCount} critical events occur (${tp} detected) and ${nonEventCount} non-events occur (${fp} false alarms). Out of ${totalAlerts} positive alerts, ${tp} are true events (PPV = ${ppvPct}%). Economic rule: act iff p (${p.toFixed(3)}) > p* (${pStar.toFixed(5)}).`;
    }

    if (costLossSvg) {
      const w = 520;
      const h = 210;
      const padL = 48;
      const padR = 24;
      const padT = 22;
      const padB = 34;
      const plotW = w - padL - padR;
      const plotH = h - padT - padB;

      const pMaxPlot = Math.min(1.0, Math.max(0.30, p * 1.6, pStar * 1.8));
      const maxCostPlot = Math.max(C * 2.2, pMaxPlot * eta * L * 1.15);

      const xMap = (pv) => padL + (Math.min(pv, pMaxPlot) / pMaxPlot) * plotW;
      const yMap = (cv) => padT + plotH - (Math.min(cv, maxCostPlot) / maxCostPlot) * plotH;

      const xPStar = xMap(pStar);
      const xP = xMap(p);
      const yCostLine = yMap(C);
      const yLossAtP = yMap(p * eta * L);
      const yLossEnd = yMap(pMaxPlot * eta * L);

      costLossSvg.innerHTML = `
        <line x1="${padL}" y1="${padT + plotH}" x2="${w - padR}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <line x1="${padL}" y1="${padT}" x2="${padL}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <!-- Protection Cost C horizontal line -->
        <line x1="${padL}" y1="${yCostLine}" x2="${w - padR}" y2="${yCostLine}" stroke="#38bdf8" stroke-width="2.2" stroke-dasharray="6 4" />
        <text x="${w - padR - 4}" y="${yCostLine - 6}" text-anchor="end" fill="#38bdf8" font-size="11" font-family="monospace">C = ${C.toFixed(0)}</text>
        <!-- Expected Mitigated Loss p * eta * L line -->
        <line x1="${padL}" y1="${yMap(0)}" x2="${w - padR}" y2="${yLossEnd}" stroke="#a78bfa" stroke-width="2.5" />
        <text x="${w - padR - 4}" y="${Math.max(padT + 12, yLossEnd - 8)}" text-anchor="end" fill="#a78bfa" font-size="11" font-family="monospace">p·η·L</text>
        <!-- Threshold p* vertical marker -->
        <line x1="${xPStar}" y1="${padT}" x2="${xPStar}" y2="${padT + plotH}" stroke="#fbbf24" stroke-width="1.8" stroke-dasharray="3 3" />
        <text x="${xPStar + 4}" y="${padT + 12}" fill="#fbbf24" font-size="11" font-family="monospace">p*=${pStar.toFixed(4)}</text>
        <!-- Forecast p marker -->
        <circle cx="${xP}" cy="${yLossAtP}" r="5.5" fill="${protectAction ? "#f87171" : "#34d399"}" />
        <text x="${Math.min(w - padR - 55, xP + 8)}" y="${Math.max(padT + 24, yLossAtP - 8)}" fill="currentColor" font-size="11" font-family="monospace">p=${p.toFixed(3)}</text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Probability p (0 → ${pMaxPlot.toFixed(2)})</text>
      `;
    }
  }

  [inputCost, inputLoss, inputEta, inputProb].forEach((el) => {
    if (el) el.addEventListener("input", updateDecisionSimulator);
  });

  /* ------------------------------------------------------------------------
     5. Simulator 2: Bridge L3 -> L1 Performative Prediction & Stackelberg
     ------------------------------------------------------------------------ */
  const inputEps = document.getElementById("sim-eps");
  const inputBeta = document.getElementById("sim-beta");
  const inputGamma = document.getElementById("sim-gamma");

  const valEps = document.getElementById("val-eps");
  const valBeta = document.getElementById("val-beta");
  const valGamma = document.getElementById("val-gamma");

  const outThetaSL = document.getElementById("out-theta-sl");
  const outThetaPS = document.getElementById("out-theta-ps");
  const outThetaPO = document.getElementById("out-theta-po");
  const outRrmRadius = document.getElementById("out-rrm-radius");
  const outRrmStatus = document.getElementById("out-rrm-status");
  const perfSvg = document.getElementById("performative-svg");

  function updatePerformativeSimulator() {
    if (!inputEps || !inputBeta || !inputGamma) return;
    const mu = 1.0;
    const alpha = 0.5;
    const eps = parseFloat(inputEps.value);
    const beta = parseFloat(inputBeta.value);
    const gamma = parseFloat(inputGamma.value);

    if (valEps) valEps.textContent = eps.toFixed(2);
    if (valBeta) valBeta.textContent = beta.toFixed(2);
    if (valGamma) valGamma.textContent = gamma.toFixed(2);

    const denom = Math.max(0.08, 1.0 - eps);
    const thetaSL = mu + alpha / gamma;
    const thetaPS = (mu + alpha / gamma) / denom;
    const thetaPO = mu / denom + alpha / (gamma * denom * denom);
    const rrmRadius = (eps * beta) / gamma;
    const converges = rrmRadius < 1.0;

    if (outThetaSL) outThetaSL.textContent = thetaSL.toFixed(4);
    if (outThetaPS) outThetaPS.textContent = thetaPS.toFixed(4);
    if (outThetaPO) outThetaPO.textContent = thetaPO.toFixed(4);
    if (outRrmRadius) outRrmRadius.textContent = rrmRadius.toFixed(3);
    if (outRrmStatus) {
      outRrmStatus.textContent = converges
        ? currentLang === "pt"
          ? "CONTRAI (r < 1)"
          : "CONTRACTS (r < 1)"
        : currentLang === "pt"
          ? "DIVERGE! (Bloqueio G6)"
          : "DIVERGES! (G6 Block)";
      outRrmStatus.style.color = converges ? "var(--status-ok)" : "var(--status-err)";
    }

    if (perfSvg) {
      const w = 520;
      const h = 210;
      const padL = 44;
      const padR = 24;
      const padT = 24;
      const padB = 34;
      const plotW = w - padL - padR;
      const plotH = h - padT - padB;

      const tMin = 0.8;
      const tMax = Math.max(3.2, thetaPO * 1.25);
      const pr = (t) => 0.5 * gamma * Math.pow((1.0 - eps) * t - mu, 2) + 0.12 - alpha * t;

      const samples = [];
      let yMin = Infinity;
      let yMax = -Infinity;
      for (let i = 0; i <= 60; i++) {
        const t = tMin + (i / 60) * (tMax - tMin);
        const y = pr(t);
        if (y < yMin) yMin = y;
        if (y > yMax) yMax = y;
        samples.push({ t, y });
      }
      const ySpan = Math.max(0.2, yMax - yMin);
      const xMap = (t) => padL + ((t - tMin) / (tMax - tMin)) * plotW;
      const yMap = (y) => padT + plotH - ((y - yMin) / (ySpan * 1.15)) * plotH;

      const pathD = samples
        .map((pt, idx) => `${idx === 0 ? "M" : "L"}${xMap(pt.t).toFixed(1)},${yMap(pt.y).toFixed(1)}`)
        .join(" ");

      const xSL = xMap(thetaSL);
      const ySL = yMap(pr(thetaSL));
      const xPS = xMap(thetaPS);
      const yPS = yMap(pr(thetaPS));
      const xPO = xMap(thetaPO);
      const yPO = yMap(pr(thetaPO));

      perfSvg.innerHTML = `
        <line x1="${padL}" y1="${padT + plotH}" x2="${w - padR}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <line x1="${padL}" y1="${padT}" x2="${padL}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <path d="${pathD}" fill="none" stroke="#38bdf8" stroke-width="2.4" />
        <!-- Passive SL -->
        <circle cx="${xSL}" cy="${ySL}" r="4.5" fill="#94a3b8" />
        <text x="${xSL}" y="${Math.max(padT + 12, ySL - 9)}" text-anchor="middle" fill="#94a3b8" font-size="10.5" font-family="monospace">θ_SL</text>
        <!-- Performative Stability PS -->
        <circle cx="${xPS}" cy="${yPS}" r="5" fill="#fbbf24" />
        <text x="${xPS}" y="${Math.max(padT + 12, yPS - 10)}" text-anchor="middle" fill="#fbbf24" font-size="10.5" font-family="monospace">θ_PS=${thetaPS.toFixed(2)}</text>
        <!-- Stackelberg Optimum PO -->
        <circle cx="${xPO}" cy="${yPO}" r="5.5" fill="#34d399" />
        <text x="${Math.min(w - padR - 35, xPO + 6)}" y="${Math.min(padT + plotH - 6, yPO + 14)}" fill="#34d399" font-size="10.5" font-family="monospace">θ_PO=${thetaPO.toFixed(2)}</text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Performative Risk PR(θ) — Bridge L3 → L1</text>
      `;
    }
  }

  [inputEps, inputBeta, inputGamma].forEach((el) => {
    if (el) el.addEventListener("input", updatePerformativeSimulator);
  });

  /* ------------------------------------------------------------------------
     6. Simulator 3: Epistemological Gates (G1-G7) & Negative Defect Proofs
     ------------------------------------------------------------------------ */
  const gateConsole = document.getElementById("gate-console-output");
  const gateExitBadge = document.getElementById("gate-exit-badge");

  const GATE_OUTPUTS = {
    positive: {
      exitCode: 0,
      cmd: "cargo run --release --bin gates",
      lines: [
        "=== EPISTEMOLOGICAL GATES G1-G7 (STOCHOLOGY PROTOCOL v1.0) ===",
        "  [G1-G5] YAML Cards v1.0 (3/3): Phi1 Simulability, Phi2 Moulton-Campbell/Exceedance, Phi3 Null/Missingness, Phi4 Chouldechova/Exit, Phi5 Resolution -> APPROVED",
        "  [G6] Prohibitions 1-4, Performativity (PS vs PO), Info-Gap + Sniedovich, SDPI, and CORP-PAV -> APPROVED",
        "  [G7] Bitemporal Clock (t_disp), Canary Column, Single-Opening Vault, and Murphy-CORP Circuit Breaker -> APPROVED",
        "=== ALL 7 EPISTEMOLOGICAL GATES APPROVED (EXIT CODE 0) ===",
      ],
    },
    cromwell: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect cromwell",
      lines: [
        "NEGATIVE PROOF [cromwell] BLOCKED SUCCESSFULLY:",
        '  Violation: ["Prohibition 4 / Cromwell\'s Rule violated: event_probability_p = 0.000000 outside [0.001000, 0.999000]"]',
        "  Gate Action: Dogmatic probability (0% or 100%) rejected before model deployment.",
      ],
    },
    phi5: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect phi5",
      lines: [
        "NEGATIVE PROOF [phi5] BLOCKED SUCCESSFULLY:",
        "  Violation: Coarse annual aggregate action against daily operational unit (R_data < R_decision).",
        "  Gate Action: Boundary Invariant Phi5 blocked answering a daily/shift decision with an annual mean.",
      ],
    },
    moulton: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect moulton",
      lines: [
        "NEGATIVE PROOF [moulton] BLOCKED SUCCESSFULLY:",
        "  Violation: Nominal pseudoreplication n = 25028 reduced to n_eff(0.90) = 1297 via Moulton-Campbell m_tilde = 119.05.",
        "  Gate Action: Boundary Invariant Phi2 rejected naive unclustered standard errors (DEFF_exceedance = 19.30).",
      ],
    },
    performativity: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect performativity",
      lines: [
        "NEGATIVE PROOF [performativity] BLOCKED SUCCESSFULLY:",
        "  Violation: RRM contraction radius eps * beta / gamma = 1.20 >= 1 diverges/oscillates [1.0, 0.0, 1.0, 0.0, 1.0].",
        "  Gate Action: Prohibition 3 blocked unstable Repeated Risk Minimization on Bridge L3 -> L1.",
      ],
    },
    sniedovich: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect sniedovich",
      lines: [
        "NEGATIVE PROOF [sniedovich] BLOCKED SUCCESSFULLY:",
        "  Violation: Collapse in No-Man's Land (Wald global Maximin = -500.0, irreversible).",
        "  Gate Action: Sniedovich Safeguard rejected local Info-Gap robustness around a fragile nominal model.",
      ],
    },
    clock: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect clock",
      lines: [
        "NEGATIVE PROOF [clock] BLOCKED SUCCESSFULLY:",
        "  Violation: Synthetic canary column mutual information I(Canary; Y) = 0.6931 nats > 0 (t_disp > tau).",
        "  Gate Action: Bitemporal Clock triggered immediate look-ahead temporal intrusion alarm.",
      ],
    },
    vault: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect vault",
      lines: [
        "NEGATIVE PROOF [vault] BLOCKED SUCCESSFULLY:",
        "  Violation: Reopening refused (VaultAlreadyOpened { vault_id: \"violated_vault\", opened_count: 1 }).",
        "  Gate Action: Single-Opening Temporal Vault prevented post-hoc HARKing / multiple testing on holdout.",
      ],
    },
    concept_drift: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect concept_drift",
      lines: [
        "NEGATIVE PROOF [concept_drift] BLOCKED SUCCESSFULLY:",
        "  Violation: Resolution collapsed (Delta RES_CORP = -0.35 < -tau_RES) under Concept Drift.",
        "  Gate Action: Murphy-CORP Circuit Breaker tripped; automated predictions halted.",
      ],
    },
    chouldechova: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect chouldechova",
      lines: [
        "NEGATIVE PROOF [chouldechova] BLOCKED SUCCESSFULLY:",
        "  Violation: Algebraic impossibility FPR_A / FPR_B = 2.6694 != 1 under unequal base rates (p_A=0.30, p_B=0.15).",
        "  Gate Action: Boundary Invariant Phi4 rejected uncalibrated simultaneous fairness claim.",
      ],
    },
  };

  document.querySelectorAll("[data-gate-defect]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const key = btn.getAttribute("data-gate-defect");
      const entry = GATE_OUTPUTS[key];
      if (!entry || !gateConsole) return;

      document.querySelectorAll("[data-gate-defect]").forEach((b) =>
        b.setAttribute("aria-pressed", b === btn ? "true" : "false")
      );

      gateConsole.innerHTML = `<span class="t-cmd">$ ${entry.cmd}</span>\n` +
        entry.lines
          .map((l) =>
            entry.exitCode === 0
              ? `<span class="t-ok">${l}</span>`
              : `<span class="t-err">${l}</span>`
          )
          .join("\n");

      if (gateExitBadge) {
        gateExitBadge.textContent =
          entry.exitCode === 0 ? "EXIT CODE: 0 (APPROVED)" : "EXIT CODE: 1 (DEFECT BLOCKED)";
        gateExitBadge.className = entry.exitCode === 0 ? "badge ok" : "badge err";
      }
    });
  });

  /* ------------------------------------------------------------------------
     7. Copy-to-Clipboard Buttons
     ------------------------------------------------------------------------ */
  document.querySelectorAll("[data-copy-target]").forEach((btn) => {
    btn.addEventListener("click", async () => {
      const targetId = btn.getAttribute("data-copy-target");
      const targetEl = targetId ? document.getElementById(targetId) : null;
      if (!targetEl) return;
      const original = btn.textContent;
      try {
        await navigator.clipboard.writeText(targetEl.textContent.trim());
        btn.textContent = currentLang === "pt" ? "Copiado!" : "Copied!";
        setTimeout(() => {
          btn.textContent = original;
        }, 1600);
      } catch {
        btn.textContent = "OK";
      }
    });
  });

  // Initialize language & simulators
  applyLanguage(currentLang);
})();

