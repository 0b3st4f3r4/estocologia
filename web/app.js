/* ==========================================================================
   Stochology Protocol v1.0 — Interactive Course Textbook & Apparatus (app.js)
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
      const modeTag = stored
        ? rendered === "dark"
          ? "DARK*"
          : "LIGHT*"
        : rendered === "dark"
          ? "AUTO·DARK"
          : "AUTO·LIGHT";
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
          localStorage.removeItem("color-scheme");
        } else {
          localStorage.setItem("color-scheme", target);
        }
      } catch {
        if (metaScheme) {
          metaScheme.content = target === systemScheme ? "light dark" : target;
        }
      }
      syncThemeUI();
    });
  }

  systemDarkQuery.addEventListener("change", syncThemeUI);
  window.addEventListener("storage", (e) => {
    if (e.key === "color-scheme" || e.key === null) {
      syncThemeUI();
    }
  });
  syncThemeUI();

  /* ------------------------------------------------------------------------
     2. Scroll Progress Indicator Fallback (scroll-progress-indicator)
     ------------------------------------------------------------------------ */
  const readingScrollBar = document.getElementById("reading-scroll-progress");
  if (readingScrollBar && !CSS.supports("animation-timeline", "scroll()")) {
    window.addEventListener(
      "scroll",
      () => {
        const scrollable = Math.max(1, document.documentElement.scrollHeight - window.innerHeight);
        const ratio = Math.min(1, Math.max(0, window.scrollY / scrollable));
        readingScrollBar.style.transform = `scaleX(${ratio})`;
      },
      { passive: true }
    );
  }

  /* ------------------------------------------------------------------------
     3. Interactive Book Chapter Navigation (hidden="until-found" + View Transitions)
     ------------------------------------------------------------------------ */
  const CHAPTER_ORDER = ["ch-0", "ch-1", "ch-2", "ch-3", "ch-4", "ch-5", "ch-6", "ch-7"];
  const readerRoot = document.getElementById("book-reader-root");
  const chapters = CHAPTER_ORDER.map((id) => document.getElementById(id)).filter(Boolean);
  const tocButtons = Array.from(document.querySelectorAll("[data-chapter-btn]"));
  const bookSidebar = document.getElementById("book-sidebar");
  const mobileTocBtn = document.getElementById("mobile-toc-btn");
  let activeChapterId = "ch-0";

  if (mobileTocBtn && bookSidebar) {
    bookSidebar.classList.add("mobile-collapsed");
    mobileTocBtn.addEventListener("click", () => {
      const isCollapsed = bookSidebar.classList.toggle("mobile-collapsed");
      mobileTocBtn.setAttribute("aria-expanded", isCollapsed ? "false" : "true");
    });
  }

  function applyChapterDOMState(targetChapterId, moveFocus) {
    activeChapterId = targetChapterId;
    chapters.forEach((ch) => {
      if (ch.id === targetChapterId) {
        ch.removeAttribute("hidden");
      } else {
        ch.setAttribute("hidden", "until-found");
      }
    });
    tocButtons.forEach((btn) => {
      const btnTarget = btn.getAttribute("data-chapter-btn");
      btn.setAttribute("aria-current", btnTarget === targetChapterId ? "page" : "false");
    });
    document.querySelectorAll(".site-nav a[data-jump-chapter]").forEach((link) => {
      const linkTarget = link.getAttribute("data-jump-chapter");
      if (linkTarget === targetChapterId) {
        link.setAttribute("aria-current", "page");
      } else {
        link.removeAttribute("aria-current");
      }
    });
    if (bookSidebar && !bookSidebar.classList.contains("mobile-collapsed")) {
      bookSidebar.classList.add("mobile-collapsed");
      if (mobileTocBtn) mobileTocBtn.setAttribute("aria-expanded", "false");
    }
    try {
      localStorage.setItem("stochology-active-chapter", targetChapterId);
    } catch {
      // ignore storage errors
    }
    if (moveFocus) {
      const targetEl = document.getElementById(targetChapterId);
      if (targetEl) {
        targetEl.focus({ preventScroll: true });
      }
      window.scrollTo({ top: 0, behavior: "smooth" });
    }
    refreshSearchHighlightsInActiveChapter();
  }

  function navigateToChapter(targetChapterId, moveFocus = true) {
    if (!CHAPTER_ORDER.includes(targetChapterId)) return;
    if (targetChapterId === activeChapterId) {
      if (moveFocus) window.scrollTo({ top: 0, behavior: "smooth" });
      return;
    }
    const oldIndex = CHAPTER_ORDER.indexOf(activeChapterId);
    const newIndex = CHAPTER_ORDER.indexOf(targetChapterId);
    const direction = newIndex >= oldIndex ? "forward" : "backward";

    const updateDOM = () => {
      applyChapterDOMState(targetChapterId, moveFocus);
    };

    // Feature detect View Transitions API (directional-navigation-transitions)
    if (!document.startViewTransition) {
      updateDOM();
      return;
    }
    try {
      document.startViewTransition({
        update: updateDOM,
        types: [direction],
      });
    } catch {
      updateDOM();
    }
  }

  tocButtons.forEach((btn) => {
    btn.addEventListener("click", () => {
      const targetId = btn.getAttribute("data-chapter-btn");
      if (targetId) {
        history.replaceState(null, "", `#${targetId}`);
        navigateToChapter(targetId, true);
      }
    });
  });

  document.querySelectorAll("[data-jump-chapter]").forEach((el) => {
    el.addEventListener("click", (e) => {
      const targetId = el.getAttribute("data-jump-chapter");
      if (targetId) {
        e.preventDefault();
        history.replaceState(null, "", `#${targetId}`);
        navigateToChapter(targetId, true);
      }
    });
  });

  // Synchronize chapter UI state when native Find-in-Page matches text inside a hidden="until-found" chapter
  if (readerRoot) {
    readerRoot.addEventListener("beforematch", (e) => {
      const matchedChapter = e.target.closest(".book-chapter");
      if (matchedChapter && matchedChapter.id) {
        applyChapterDOMState(matchedChapter.id, false);
      }
    });
  }

  /* ------------------------------------------------------------------------
     4. Chapter Outline Scrollspy (scrollspy: scroll-target-group + fallback)
     ------------------------------------------------------------------------ */
  const outlineLinks = Array.from(document.querySelectorAll(".chapter-outline-nav a"));
  if (CSS.supports("scroll-target-group: auto")) {
    const syncAriaCurrent = () => {
      const currentLink = document.querySelector(".chapter-outline-nav a:target-current");
      outlineLinks.forEach((link) => {
        link.setAttribute("aria-current", link === currentLink ? "true" : "false");
      });
    };
    syncAriaCurrent();
    document.addEventListener("scrollend", syncAriaCurrent, { passive: true });
  } else if ("IntersectionObserver" in window) {
    const sectionObserver = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting && entry.target.id) {
            outlineLinks.forEach((link) => {
              const isMatch = link.getAttribute("href") === `#${entry.target.id}`;
              link.setAttribute("aria-current", isMatch ? "true" : "false");
            });
          }
        });
      },
      { rootMargin: "-15% 0px -70% 0px" }
    );
    document.querySelectorAll(".chapter-section[id]").forEach((sec) => sectionObserver.observe(sec));
  }

  /* ------------------------------------------------------------------------
     5. Searchable Hidden Content for YAML Cards Tabs (hidden="until-found")
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
        if (targetId) activatePanel(targetId);
      });
    });

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

    if (!("onbeforematch" in HTMLElement.prototype)) {
      document.querySelectorAll('[hidden="until-found"]').forEach((el) => {
        el.removeAttribute("hidden");
      });
      buttons.forEach((btn) => btn.setAttribute("aria-expanded", "true"));
    }
  }

  /* ------------------------------------------------------------------------
     6. Global Book Search with CSS Custom Highlight API & IME-Safe Submit
     ------------------------------------------------------------------------ */
  const searchForm = document.getElementById("book-search-form");
  const searchInput = document.getElementById("book-search-input");
  const searchMeta = document.getElementById("book-search-meta");
  let currentSearchQuery = "";
  let lastCompositionEndAt = null;

  function clearFallbackMarks(container) {
    container.querySelectorAll("mark.search-fallback-mark").forEach((mark) => {
      const parent = mark.parentNode;
      if (parent) {
        parent.replaceChild(document.createTextNode(mark.textContent || ""), mark);
        parent.normalize();
      }
    });
  }

  function refreshSearchHighlightsInActiveChapter() {
    const activeEl = document.getElementById(activeChapterId);
    if (!activeEl) return;

    if (window.CSS && CSS.highlights) {
      CSS.highlights.clear();
    } else {
      clearFallbackMarks(activeEl);
    }

    const query = currentSearchQuery.trim().toLowerCase();
    if (query.length < 2) return;

    const walker = document.createTreeWalker(activeEl, NodeFilter.SHOW_TEXT);
    const textNodes = [];
    let node = walker.nextNode();
    while (node) {
      textNodes.push(node);
      node = walker.nextNode();
    }

    if (window.CSS && CSS.highlights && typeof Highlight !== "undefined") {
      const ranges = [];
      for (const textNode of textNodes) {
        const text = textNode.textContent || "";
        const lower = text.toLowerCase();
        let startPos = 0;
        while (startPos < lower.length) {
          const idx = lower.indexOf(query, startPos);
          if (idx === -1) break;
          const range = new Range();
          range.setStart(textNode, idx);
          range.setEnd(textNode, idx + query.length);
          ranges.push(range);
          startPos = idx + query.length;
        }
      }
      if (ranges.length > 0) {
        const searchHighlight = new Highlight(...ranges);
        CSS.highlights.set("book-search", searchHighlight);
      }
    } else {
      // Fallback: wrap matched text in <mark> elements without innerHTML
      for (const textNode of textNodes) {
        const text = textNode.textContent || "";
        let pos = text.toLowerCase().indexOf(query);
        if (pos === -1) continue;
        const frag = document.createDocumentFragment();
        let last = 0;
        while (pos !== -1) {
          frag.append(text.slice(last, pos));
          const mark = document.createElement("mark");
          mark.className = "search-fallback-mark";
          mark.textContent = text.slice(pos, pos + query.length);
          frag.append(mark);
          last = pos + query.length;
          pos = text.toLowerCase().indexOf(query, last);
        }
        frag.append(text.slice(last));
        textNode.replaceWith(frag);
      }
    }
  }

  function runGlobalBookSearch(jumpToFirstMatch) {
    if (!searchInput || !searchMeta) return;
    currentSearchQuery = searchInput.value.trim();
    searchMeta.innerHTML = "";

    if (currentSearchQuery.length < 2) {
      refreshSearchHighlightsInActiveChapter();
      return;
    }

    const qLower = currentSearchQuery.toLowerCase();
    const hitsByChapter = [];

    chapters.forEach((ch, idx) => {
      const walker = document.createTreeWalker(ch, NodeFilter.SHOW_TEXT);
      let count = 0;
      let n = walker.nextNode();
      while (n) {
        const txt = (n.textContent || "").toLowerCase();
        let pos = 0;
        while (pos < txt.length) {
          const found = txt.indexOf(qLower, pos);
          if (found === -1) break;
          count++;
          pos = found + qLower.length;
        }
        n = walker.nextNode();
      }
      if (count > 0) {
        hitsByChapter.push({ id: ch.id, idx, count });
      }
    });

    if (hitsByChapter.length === 0) {
      searchMeta.textContent =
        currentLang === "pt" ? "Nenhuma ocorrência encontrada." : "No matches found.";
      refreshSearchHighlightsInActiveChapter();
      return;
    }

    hitsByChapter.forEach((hit) => {
      const btn = document.createElement("button");
      btn.type = "button";
      btn.className = "search-hit-pill";
      const label = hit.idx === 0 ? "Prefácio" : `Cap. ${hit.idx}`;
      btn.textContent = `${label} (${hit.count})`;
      btn.addEventListener("click", () => {
        navigateToChapter(hit.id, true);
      });
      searchMeta.appendChild(btn);
    });

    if (jumpToFirstMatch && !hitsByChapter.some((h) => h.id === activeChapterId)) {
      navigateToChapter(hitsByChapter[0].id, true);
    } else {
      refreshSearchHighlightsInActiveChapter();
    }
  }

  if (searchInput && searchForm) {
    searchInput.addEventListener("compositionend", (event) => {
      lastCompositionEndAt = event.timeStamp;
    });

    // IME-safe Enter handling (ime-safe-enter-submit)
    searchInput.addEventListener("keydown", (event) => {
      if (event.key === "Enter") {
        event.preventDefault();
        if (event.isComposing || event.keyCode === 229) {
          return;
        }
        if (lastCompositionEndAt !== null && Math.abs(event.timeStamp - lastCompositionEndAt) < 50) {
          return;
        }
        searchForm.requestSubmit();
      }
    });

    searchInput.addEventListener("input", () => {
      runGlobalBookSearch(false);
    });

    searchForm.addEventListener("submit", (event) => {
      event.preventDefault();
      runGlobalBookSearch(true);
    });
  }

  /* ------------------------------------------------------------------------
     7. Student Mastery Progress & Interactive Checkpoints (Quizzes)
     ------------------------------------------------------------------------ */
  const progressEl = document.getElementById("course-progress");
  const progressLabelEl = document.getElementById("course-progress-label");
  const progressStatusEl = document.getElementById("course-progress-status");
  const resetProgressBtn = document.getElementById("reset-progress-btn");
  let completedChapters = new Set();

  try {
    const raw = localStorage.getItem("stochology-book-progress");
    if (raw) {
      const parsed = JSON.parse(raw);
      if (Array.isArray(parsed)) {
        parsed.forEach((id) => completedChapters.add(id));
      }
    }
  } catch {
    // ignore
  }

  function syncCourseProgressUI() {
    const count = completedChapters.size;
    if (progressEl) {
      progressEl.value = count;
    }
    if (progressLabelEl) {
      progressLabelEl.textContent =
        currentLang === "pt" ? `${count} / 7 caps` : `${count} / 7 chs`;
    }
    if (progressStatusEl) {
      if (count === 7) {
        progressStatusEl.textContent =
          currentLang === "pt"
            ? "Todos os 7 capítulos concluídos e aprovados!"
            : "All 7 chapters completed and approved!";
      } else {
        progressStatusEl.textContent =
          currentLang === "pt"
            ? `${count} de 7 checkpoints aprovados.`
            : `${count} of 7 checkpoints approved.`;
      }
    }
    for (let i = 1; i <= 7; i++) {
      const chId = `ch-${i}`;
      const isDone = completedChapters.has(chId);
      const checkSpan = document.getElementById(`toc-check-${chId}`);
      if (checkSpan) {
        checkSpan.textContent = isDone ? "✓" : "○";
        checkSpan.className = isDone ? "toc-check done" : "toc-check";
      }
      const badgeEl = document.getElementById(`badge-quiz-${chId}`);
      if (badgeEl && isDone) {
        badgeEl.textContent = currentLang === "pt" ? "APROVADO ✓" : "APPROVED ✓";
        badgeEl.className = "badge ok";
      }
    }
  }

  if (resetProgressBtn) {
    resetProgressBtn.addEventListener("click", () => {
      completedChapters.clear();
      try {
        localStorage.removeItem("stochology-book-progress");
      } catch {
        // ignore
      }
      document.querySelectorAll(".quiz-feedback").forEach((fb) => {
        fb.hidden = true;
      });
      for (let i = 1; i <= 7; i++) {
        const badgeEl = document.getElementById(`badge-quiz-ch-${i}`);
        if (badgeEl) {
          badgeEl.textContent = currentLang === "pt" ? "Pendente" : "Pending";
          badgeEl.className = "badge";
        }
      }
      syncCourseProgressUI();
    });
  }

  document.querySelectorAll("[data-check-chapter]").forEach((btn) => {
    btn.addEventListener("click", () => {
      const chId = btn.getAttribute("data-check-chapter");
      const card = document.querySelector(`.checkpoint-card[data-checkpoint-chapter="${chId}"]`);
      if (!card || !chId) return;

      const blocks = Array.from(card.querySelectorAll(".quiz-question-block"));
      let allCorrect = true;

      blocks.forEach((block) => {
        const qId = block.getAttribute("data-quiz-id");
        const correctVal = block.getAttribute("data-correct");
        const selected = block.querySelector(`input[name="${qId}"]:checked`);
        const fb = block.querySelector(".quiz-feedback");
        if (!fb) return;

        fb.hidden = false;
        if (!selected) {
          allCorrect = false;
          fb.className = "quiz-feedback err";
          fb.textContent =
            currentLang === "pt"
              ? "Selecione uma alternativa antes de verificar."
              : "Please select an option before checking.";
        } else if (selected.value === correctVal) {
          fb.className = "quiz-feedback ok";
          fb.textContent =
            currentLang === "pt"
              ? "✓ Correto! Resposta alinhada ao contrato formal do Protocolo Stochology v1.0."
              : "✓ Correct! Answer aligned with the formal Stochology Protocol v1.0 contract.";
        } else {
          allCorrect = false;
          fb.className = "quiz-feedback err";
          fb.textContent =
            currentLang === "pt"
              ? "✗ Incorreto. Revise os teoremas e invariantes desta seção e tente novamente."
              : "✗ Incorrect. Review the theorems and invariants in this section and try again.";
        }
      });

      const badgeEl = document.getElementById(`badge-quiz-${chId}`);
      if (allCorrect) {
        completedChapters.add(chId);
        try {
          localStorage.setItem(
            "stochology-book-progress",
            JSON.stringify(Array.from(completedChapters))
          );
        } catch {
          // ignore
        }
        if (badgeEl) {
          badgeEl.textContent = currentLang === "pt" ? "APROVADO ✓" : "APPROVED ✓";
          badgeEl.className = "badge ok";
        }
      } else if (badgeEl) {
        badgeEl.textContent = currentLang === "pt" ? "Tente Novamente" : "Try Again";
        badgeEl.className = "badge err";
      }
      syncCourseProgressUI();
    });
  });

  /* ------------------------------------------------------------------------
     8. Simulator 1A: Bridge L1 -> L2 Meng's Identity & Manski MNAR Bounds
     ------------------------------------------------------------------------ */
  const inputMengN = document.getElementById("sim-meng-n");
  const inputMengRho = document.getElementById("sim-meng-rho");
  const inputManskiMiss = document.getElementById("sim-manski-miss");

  const valMengN = document.getElementById("val-meng-n");
  const valMengRho = document.getElementById("val-meng-rho");
  const valManskiMiss = document.getElementById("val-manski-miss");

  const outMengNominal = document.getElementById("out-meng-nominal");
  const outMengNeff = document.getElementById("out-meng-neff");
  const outMengLoss = document.getElementById("out-meng-loss");
  const outManskiBand = document.getElementById("out-manski-band");
  const mengManskiSvg = document.getElementById("meng-manski-svg");

  function updateMengManskiSimulator() {
    if (!inputMengN || !inputMengRho || !inputManskiMiss) return;
    const N = 100000;
    const n = parseFloat(inputMengN.value);
    const rho = parseFloat(inputMengRho.value);
    const miss = parseFloat(inputManskiMiss.value);

    if (valMengN) valMengN.textContent = n.toFixed(0);
    if (valMengRho) valMengRho.textContent = rho.toFixed(3);
    if (valManskiMiss) valManskiMiss.textContent = miss.toFixed(2);

    const f = n / N;
    const dropoutRatio = (1 - f) / f;
    // Effective sample size under Meng's Data Defect Correlation
    const nEff =
      rho <= 0.0001
        ? n
        : Math.min(n, 1.0 / (dropoutRatio * rho * rho + 1.0 / n));
    const lossPct = ((nEff - n) / n) * 100;

    // Non-parametric Manski bounds on [Y_min = 0, Y_max = 30000] with observed mean Y_obs = 7600
    const yObs = 7600;
    const yMin = 0;
    const yMax = 30000;
    const pObs = 1.0 - miss;
    const manskiL = pObs * yObs + miss * yMin;
    const manskiU = pObs * yObs + miss * yMax;

    if (outMengNominal) outMengNominal.textContent = n.toLocaleString("en-US");
    if (outMengNeff) outMengNeff.textContent = nEff.toFixed(1);
    if (outMengLoss) {
      outMengLoss.textContent = `${lossPct.toFixed(1)}%`;
      outMengLoss.style.color = nEff < n * 0.25 ? "var(--status-err)" : "var(--status-ok)";
    }
    if (outManskiBand) {
      outManskiBand.textContent = `[${manskiL.toFixed(0)}, ${manskiU.toFixed(0)}]`;
    }

    if (mengManskiSvg) {
      const w = 520;
      const h = 210;
      const padL = 48;
      const padR = 24;
      const padT = 28;
      const padB = 36;
      const plotW = w - padL - padR;

      const xScale = (val) => padL + (val / yMax) * plotW;
      const xL = xScale(manskiL);
      const xU = xScale(manskiU);
      const xObs = xScale(yObs);

      const neffRatio = Math.max(0.02, Math.min(1.0, nEff / n));
      const barFullW = plotW * (n / 80000);
      const barEffW = Math.max(6, barFullW * neffRatio);

      mengManskiSvg.innerHTML = `
        <text x="${padL}" y="22" fill="currentColor" font-size="11" font-family="monospace">1. Meng Effective Sample Size (n=${n} → n_eff=${nEff.toFixed(1)})</text>
        <rect x="${padL}" y="30" width="${barFullW.toFixed(1)}" height="18" rx="4" fill="#334155" opacity="0.45" />
        <rect x="${padL}" y="30" width="${barEffW.toFixed(1)}" height="18" rx="4" fill="#38bdf8" />
        <text x="${padL + barFullW + 6}" y="43" fill="#38bdf8" font-size="10.5" font-family="monospace">n_eff = ${nEff.toFixed(1)}</text>

        <text x="${padL}" y="92" fill="currentColor" font-size="11" font-family="monospace">2. Manski Non-Parametric MNAR Identification Band [0, 30000]</text>
        <line x1="${padL}" y1="125" x2="${w - padR}" y2="125" stroke="currentColor" stroke-opacity="0.3" stroke-width="2" />
        <rect x="${xL.toFixed(1)}" y="112" width="${Math.max(4, xU - xL).toFixed(1)}" height="26" rx="4" fill="#a78bfa" fill-opacity="0.35" stroke="#a78bfa" stroke-width="2" />
        <circle cx="${xObs.toFixed(1)}" cy="125" r="5.5" fill="#fbbf24" />
        <text x="${xL.toFixed(1)}" y="154" text-anchor="middle" fill="#a78bfa" font-size="10.5" font-family="monospace">L=${manskiL.toFixed(0)}</text>
        <text x="${xU.toFixed(1)}" y="154" text-anchor="middle" fill="#a78bfa" font-size="10.5" font-family="monospace">U=${manskiU.toFixed(0)}</text>
        <text x="${xObs.toFixed(1)}" y="105" text-anchor="middle" fill="#fbbf24" font-size="10.5" font-family="monospace">Y_obs=${yObs}</text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Bridge L₁ → L₂ Audit: Selection Correlation &amp; Manski Bounds</text>
      `;
    }
  }

  [inputMengN, inputMengRho, inputManskiMiss].forEach((el) => {
    if (el) el.addEventListener("input", updateMengManskiSimulator);
  });

  /* ------------------------------------------------------------------------
     9. Simulator 1B: Bridge L3 -> L1 Performative Prediction & Stackelberg
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
        <circle cx="${xSL}" cy="${ySL}" r="4.5" fill="#94a3b8" />
        <text x="${xSL}" y="${Math.max(padT + 12, ySL - 9)}" text-anchor="middle" fill="#94a3b8" font-size="10.5" font-family="monospace">θ_SL</text>
        <circle cx="${xPS}" cy="${yPS}" r="5" fill="#fbbf24" />
        <text x="${xPS}" y="${Math.max(padT + 12, yPS - 10)}" text-anchor="middle" fill="#fbbf24" font-size="10.5" font-family="monospace">θ_PS=${thetaPS.toFixed(2)}</text>
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
     10. Simulator 2: Info-Gap Robustness Horizon vs. Sniedovich/Wald Safeguard
     ------------------------------------------------------------------------ */
  const inputIgRc = document.getElementById("sim-ig-rc");
  const inputIgSlopeA = document.getElementById("sim-ig-slope-a");
  const inputIgWaldA = document.getElementById("sim-ig-wald-a");

  const valIgRc = document.getElementById("val-ig-rc");
  const valIgSlopeA = document.getElementById("val-ig-slope-a");
  const valIgWaldA = document.getElementById("val-ig-wald-a");

  const outIgAlphaA = document.getElementById("out-ig-alpha-a");
  const outIgAlphaB = document.getElementById("out-ig-alpha-b");
  const outIgSniedovich = document.getElementById("out-ig-sniedovich");
  const infogapSvg = document.getElementById("infogap-svg");

  function updateInfoGapSimulator() {
    if (!inputIgRc || !inputIgSlopeA || !inputIgWaldA) return;
    const rc = parseFloat(inputIgRc.value);
    const kA = parseFloat(inputIgSlopeA.value);
    const waldA = parseFloat(inputIgWaldA.value);

    if (valIgRc) valIgRc.textContent = rc.toFixed(0);
    if (valIgSlopeA) valIgSlopeA.textContent = kA.toFixed(0);
    if (valIgWaldA) valIgWaldA.textContent = waldA.toFixed(0);

    // Option A: nominal reward R_A(0) = 100, slope k_A; Option B: R_B(0) = 85, slope k_B = 20, Wald floor = +15
    const nomA = 100;
    const nomB = 85;
    const kB = 20;

    const alphaA = Math.max(0, (nomA - rc) / kA);
    const alphaB = Math.max(0, (nomB - rc) / kB);
    const sniedovichSafeA = waldA >= 0;

    if (outIgAlphaA) outIgAlphaA.textContent = alphaA.toFixed(2);
    if (outIgAlphaB) outIgAlphaB.textContent = alphaB.toFixed(2);
    if (outIgSniedovich) {
      outIgSniedovich.textContent = sniedovichSafeA
        ? "APPROVED (Wald ≥ 0)"
        : "BLOCKED (No-Man's Land)";
      outIgSniedovich.style.color = sniedovichSafeA ? "var(--status-ok)" : "var(--status-err)";
    }

    if (infogapSvg) {
      const w = 520;
      const h = 210;
      const padL = 48;
      const padR = 24;
      const padT = 22;
      const padB = 34;
      const plotW = w - padL - padR;
      const plotH = h - padT - padB;

      const aMax = 3.2;
      const rMin = -40;
      const rMax = 110;
      const xMap = (a) => padL + (a / aMax) * plotW;
      const yMap = (r) => padT + plotH - ((Math.max(rMin, Math.min(rMax, r)) - rMin) / (rMax - rMin)) * plotH;

      const yRc = yMap(rc);
      const xAlphaA = xMap(Math.min(aMax, alphaA));
      const xAlphaB = xMap(Math.min(aMax, alphaB));

      infogapSvg.innerHTML = `
        <line x1="${padL}" y1="${padT + plotH}" x2="${w - padR}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <line x1="${padL}" y1="${padT}" x2="${padL}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <!-- Critical requirement rc line -->
        <line x1="${padL}" y1="${yRc}" x2="${w - padR}" y2="${yRc}" stroke="#fbbf24" stroke-width="1.8" stroke-dasharray="5 4" />
        <text x="${w - padR - 4}" y="${yRc - 6}" text-anchor="end" fill="#fbbf24" font-size="10.5" font-family="monospace">r_c = ${rc}</text>
        <!-- Option A curve (Aggressive / Fragile in No-Man's Land) -->
        <line x1="${xMap(0)}" y1="${yMap(nomA)}" x2="${xMap(aMax)}" y2="${yMap(nomA - kA * aMax)}" stroke="#f87171" stroke-width="2.4" />
        <circle cx="${xAlphaA}" cy="${yRc}" r="5" fill="#f87171" />
        <text x="${xAlphaA}" y="${Math.max(padT + 14, yRc - 10)}" text-anchor="middle" fill="#f87171" font-size="10" font-family="monospace">α̂_A=${alphaA.toFixed(2)}</text>
        <!-- Option B curve (Conservative + Wald Safeguard) -->
        <line x1="${xMap(0)}" y1="${yMap(nomB)}" x2="${xMap(aMax)}" y2="${yMap(Math.max(15, nomB - kB * aMax))}" stroke="#34d399" stroke-width="2.5" />
        <circle cx="${xAlphaB}" cy="${yRc}" r="5" fill="#34d399" />
        <text x="${Math.min(w - padR - 30, xAlphaB + 8)}" y="${Math.min(padT + plotH - 8, yRc + 14)}" fill="#34d399" font-size="10" font-family="monospace">α̂_B=${alphaB.toFixed(2)}</text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Horizon of Uncertainty α (Zero-ing at α=0 vs No-Man's Land)</text>
      `;
    }
  }

  [inputIgRc, inputIgSlopeA, inputIgWaldA].forEach((el) => {
    if (el) el.addEventListener("input", updateInfoGapSimulator);
  });

  /* ------------------------------------------------------------------------
     11. Simulator 3: Lenses 2 & 3 — Itô Variance Drain, Kelly & Hill Tail
     ------------------------------------------------------------------------ */
  const inputItoMu = document.getElementById("sim-ito-mu");
  const inputItoSigma = document.getElementById("sim-ito-sigma");
  const inputItoLev = document.getElementById("sim-ito-lev");
  const inputHillAlpha = document.getElementById("sim-hill-alpha");

  const valItoMu = document.getElementById("val-ito-mu");
  const valItoSigma = document.getElementById("val-ito-sigma");
  const valItoLev = document.getElementById("val-ito-lev");
  const valHillAlpha = document.getElementById("val-hill-alpha");

  const outItoGens = document.getElementById("out-ito-gens");
  const outItoGtime = document.getElementById("out-ito-gtime");
  const outItoFstar = document.getElementById("out-ito-fstar");
  const outHillRegime = document.getElementById("out-hill-regime");
  const itoHillSvg = document.getElementById("ito-hill-svg");

  function updateItoHillSimulator() {
    if (!inputItoMu || !inputItoSigma || !inputItoLev || !inputHillAlpha) return;
    const mu = parseFloat(inputItoMu.value);
    const sigma = parseFloat(inputItoSigma.value);
    const f = parseFloat(inputItoLev.value);
    const alpha = parseFloat(inputHillAlpha.value);

    if (valItoMu) valItoMu.textContent = mu.toFixed(3);
    if (valItoSigma) valItoSigma.textContent = sigma.toFixed(3);
    if (valItoLev) valItoLev.textContent = f.toFixed(2);
    if (valHillAlpha) valHillAlpha.textContent = alpha.toFixed(2);

    const gEns = f * mu;
    const gTime = f * mu - 0.5 * f * f * sigma * sigma;
    const fStar = mu / (sigma * sigma);

    if (outItoGens) outItoGens.textContent = `${(gEns * 100).toFixed(2)}%`;
    if (outItoGtime) {
      outItoGtime.textContent = `${(gTime * 100).toFixed(2)}%`;
      outItoGtime.style.color = gTime > 0 ? "var(--status-ok)" : "var(--status-err)";
    }
    if (outItoFstar) outItoFstar.textContent = `${fStar.toFixed(2)}x`;

    if (outHillRegime) {
      if (alpha <= 1.0) {
        outHillRegime.textContent = "No Mean (α ≤ 1)";
        outHillRegime.style.color = "var(--status-err)";
      } else if (alpha <= 2.0) {
        outHillRegime.textContent = "Var = ∞ (1 < α ≤ 2)";
        outHillRegime.style.color = "var(--status-warn)";
      } else {
        outHillRegime.textContent = "Finite Var (α > 2)";
        outHillRegime.style.color = "var(--status-ok)";
      }
    }

    if (itoHillSvg) {
      const w = 520;
      const h = 210;
      const padL = 48;
      const padR = 24;
      const padT = 22;
      const padB = 34;
      const plotW = w - padL - padR;
      const plotH = h - padT - padB;

      const fMax = Math.max(4.5, fStar * 2.3);
      const gCurve = (fv) => fv * mu - 0.5 * fv * fv * sigma * sigma;
      const gPeak = Math.max(0.02, gCurve(fStar));
      const gMin = Math.min(-0.08, gCurve(fMax));

      const xMap = (fv) => padL + (fv / fMax) * plotW;
      const yMap = (gv) => padT + plotH - ((gv - gMin) / (gPeak * 1.25 - gMin)) * plotH;

      const pts = [];
      for (let i = 0; i <= 60; i++) {
        const fv = (i / 60) * fMax;
        pts.push(`${i === 0 ? "M" : "L"}${xMap(fv).toFixed(1)},${yMap(gCurve(fv)).toFixed(1)}`);
      }
      const zeroY = yMap(0);
      const xFStar = xMap(Math.min(fMax, fStar));
      const yFStar = yMap(gCurve(fStar));
      const xCur = xMap(Math.min(fMax, f));
      const yCur = yMap(gTime);

      itoHillSvg.innerHTML = `
        <line x1="${padL}" y1="${zeroY}" x2="${w - padR}" y2="${zeroY}" stroke="currentColor" stroke-opacity="0.35" stroke-dasharray="4 4" />
        <line x1="${padL}" y1="${padT}" x2="${padL}" y2="${padT + plotH}" stroke="currentColor" stroke-opacity="0.3" />
        <path d="${pts.join(" ")}" fill="none" stroke="#38bdf8" stroke-width="2.5" />
        <circle cx="${xFStar}" cy="${yFStar}" r="5" fill="#34d399" />
        <text x="${xFStar}" y="${Math.max(padT + 12, yFStar - 8)}" text-anchor="middle" fill="#34d399" font-size="10.5" font-family="monospace">f*=${fStar.toFixed(2)}</text>
        <circle cx="${xCur}" cy="${yCur}" r="5.5" fill="${gTime >= 0 ? "#fbbf24" : "#f87171"}" />
        <text x="${Math.min(w - padR - 45, xCur + 8)}" y="${Math.max(padT + 16, yCur - 6)}" fill="currentColor" font-size="10.5" font-family="monospace">f=${f.toFixed(2)}</text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Kelly Parabola g_time(f) = fμ − ½f²σ² (Ruin Zone for f &gt; 2f*)</text>
      `;
    }
  }

  [inputItoMu, inputItoSigma, inputItoLev, inputHillAlpha].forEach((el) => {
    if (el) el.addEventListener("input", updateItoHillSimulator);
  });

  /* ------------------------------------------------------------------------
     12. Simulator 4: Moulton-Campbell (Phi2) & Chouldechova (Phi4)
     ------------------------------------------------------------------------ */
  const inputMcMbar = document.getElementById("sim-mc-mbar");
  const inputMcCv = document.getElementById("sim-mc-cv");
  const inputMcIcc = document.getElementById("sim-mc-icc");
  const inputChoulPa = document.getElementById("sim-choul-pa");

  const valMcMbar = document.getElementById("val-mc-mbar");
  const valMcCv = document.getElementById("val-mc-cv");
  const valMcIcc = document.getElementById("val-mc-icc");
  const valChoulPa = document.getElementById("val-choul-pa");

  const outMcMtilde = document.getElementById("out-mc-mtilde");
  const outMcDeff = document.getElementById("out-mc-deff");
  const outMcNeff = document.getElementById("out-mc-neff");
  const outChoulRatio = document.getElementById("out-choul-ratio");
  const moultonChoulSvg = document.getElementById("moulton-choul-svg");

  function updateMoultonChoulSimulator() {
    if (!inputMcMbar || !inputMcCv || !inputMcIcc || !inputChoulPa) return;
    const nNominal = 20778;
    const mBar = parseFloat(inputMcMbar.value);
    const cv = parseFloat(inputMcCv.value);
    const icc = parseFloat(inputMcIcc.value);
    const pA = parseFloat(inputChoulPa.value);
    const pB = 0.1119;

    if (valMcMbar) valMcMbar.textContent = mBar.toFixed(1);
    if (valMcCv) valMcCv.textContent = cv.toFixed(2);
    if (valMcIcc) valMcIcc.textContent = icc.toFixed(3);
    if (valChoulPa) valChoulPa.textContent = pA.toFixed(3);

    const mTilde = mBar * (1.0 + cv * cv);
    const deff = 1.0 + (mTilde - 1.0) * icc;
    const nEff = nNominal / deff;

    // Chouldechova ratio under equal PPV and FNR: FPR_A / FPR_B = [p_A/(1-p_A)] / [p_B/(1-p_B)]
    const oddsA = pA / (1.0 - pA);
    const oddsB = pB / (1.0 - pB);
    const fprRatio = oddsA / oddsB;

    if (outMcMtilde) outMcMtilde.textContent = mTilde.toFixed(2);
    if (outMcDeff) outMcDeff.textContent = deff.toFixed(2);
    if (outMcNeff) outMcNeff.textContent = Math.round(nEff).toLocaleString("en-US");
    if (outChoulRatio) {
      outChoulRatio.textContent = `${fprRatio.toFixed(2)}x`;
      outChoulRatio.style.color = fprRatio > 1.25 ? "var(--status-warn)" : "var(--status-ok)";
    }

    if (moultonChoulSvg) {
      const w = 520;
      const h = 210;
      const padL = 48;
      const padR = 24;
      const plotW = w - padL - padR;

      const wNom = plotW * 0.85;
      const wEff = Math.max(8, wNom * (nEff / nNominal));
      const wFprB = 55;
      const wFprA = Math.min(plotW * 0.85, wFprB * fprRatio);

      moultonChoulSvg.innerHTML = `
        <text x="${padL}" y="28" fill="currentColor" font-size="11" font-family="monospace">Φ₂ Moulton-Campbell Sample Collapse (n = 20,778 → n_eff = ${Math.round(nEff)})</text>
        <rect x="${padL}" y="38" width="${wNom.toFixed(1)}" height="18" rx="4" fill="#334155" opacity="0.45" />
        <rect x="${padL}" y="38" width="${wEff.toFixed(1)}" height="18" rx="4" fill="#34d399" />
        <text x="${padL + wEff + 8}" y="51" fill="#34d399" font-size="10.5" font-family="monospace">n_eff = ${Math.round(nEff)} (DEFF=${deff.toFixed(1)})</text>

        <text x="${padL}" y="102" fill="currentColor" font-size="11" font-family="monospace">Φ₄ Chouldechova False Positive Disparity (under equal PPV &amp; FNR)</text>
        <rect x="${padL}" y="114" width="${wFprB}" height="16" rx="3" fill="#38bdf8" />
        <text x="${padL + wFprB + 8}" y="126" fill="#38bdf8" font-size="10.5" font-family="monospace">Group B FPR (1.00x, p_B=0.112)</text>
        <rect x="${padL}" y="138" width="${wFprA.toFixed(1)}" height="16" rx="3" fill="#f87171" />
        <text x="${padL + wFprA + 8}" y="150" fill="#f87171" font-size="10.5" font-family="monospace">Group A FPR (${fprRatio.toFixed(2)}x, p_A=${pA.toFixed(3)})</text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Boundary Invariants Φ₂ &amp; Φ₄ Audit</text>
      `;
    }
  }

  [inputMcMbar, inputMcCv, inputMcIcc, inputChoulPa].forEach((el) => {
    if (el) el.addEventListener("input", updateMoultonChoulSimulator);
  });

  /* ------------------------------------------------------------------------
     13. Simulator 5: Cost-Loss Boundary p* = C / (eta * L), EVPI & Natural Tree
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
    storms: { cost: 3600, loss: 30000, eta: 1.0, prob: 0.098 },
    queue: { cost: 2400, loss: 30000, eta: 1.0, prob: 0.099 },
    flights: { cost: 1500, loss: 25000, eta: 1.0, prob: 0.098 },
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

      const pMaxPlot = Math.min(1.0, Math.max(0.3, p * 1.6, pStar * 1.8));
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
        <line x1="${padL}" y1="${yCostLine}" x2="${w - padR}" y2="${yCostLine}" stroke="#38bdf8" stroke-width="2.2" stroke-dasharray="6 4" />
        <text x="${w - padR - 4}" y="${yCostLine - 6}" text-anchor="end" fill="#38bdf8" font-size="11" font-family="monospace">C = ${C.toFixed(0)}</text>
        <line x1="${padL}" y1="${yMap(0)}" x2="${w - padR}" y2="${yLossEnd}" stroke="#a78bfa" stroke-width="2.5" />
        <text x="${w - padR - 4}" y="${Math.max(padT + 12, yLossEnd - 8)}" text-anchor="end" fill="#a78bfa" font-size="11" font-family="monospace">p·η·L</text>
        <line x1="${xPStar}" y1="${padT}" x2="${xPStar}" y2="${padT + plotH}" stroke="#fbbf24" stroke-width="1.8" stroke-dasharray="3 3" />
        <text x="${xPStar + 4}" y="${padT + 12}" fill="#fbbf24" font-size="11" font-family="monospace">p*=${pStar.toFixed(4)}</text>
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
     14. Simulator 6: Bitemporal Governance — Murphy-CORP Breaker & Lei-Candès
     ------------------------------------------------------------------------ */
  const inputDriftDtv = document.getElementById("sim-drift-dtv");
  const inputDriftRel = document.getElementById("sim-drift-rel");
  const inputDriftRes = document.getElementById("sim-drift-res");

  const valDriftDtv = document.getElementById("val-drift-dtv");
  const valDriftRel = document.getElementById("val-drift-rel");
  const valDriftRes = document.getElementById("val-drift-res");

  const outDriftLc = document.getElementById("out-drift-lc");
  const outDriftRegime = document.getElementById("out-drift-regime");
  const outDriftAction = document.getElementById("out-drift-action");
  const outDriftBreaker = document.getElementById("out-drift-breaker");
  const driftSvg = document.getElementById("drift-governance-svg");

  function updateDriftSimulator() {
    if (!inputDriftDtv || !inputDriftRel || !inputDriftRes) return;
    const alpha = 0.1;
    const dtv = parseFloat(inputDriftDtv.value);
    const dRel = parseFloat(inputDriftRel.value);
    const dRes = parseFloat(inputDriftRes.value);

    if (valDriftDtv) valDriftDtv.textContent = dtv.toFixed(3);
    if (valDriftRel) valDriftRel.textContent = `+${dRel.toFixed(3)}`;
    if (valDriftRes) valDriftRes.textContent = `${dRes >= 0 ? "+" : ""}${dRes.toFixed(3)}`;

    const lcFloor = Math.max(0, 1.0 - alpha - dtv);
    const isConceptDrift = dRes < -0.05;
    const isCovariateOrLabelShift = !isConceptDrift && (dRel > 0.05 || dtv > 0.12);

    if (outDriftLc) outDriftLc.textContent = `${(lcFloor * 100).toFixed(1)}%`;

    if (outDriftRegime && outDriftAction && outDriftBreaker) {
      if (isConceptDrift) {
        outDriftRegime.textContent = "CONCEPT DRIFT";
        outDriftRegime.style.color = "var(--status-err)";
        outDriftAction.textContent =
          currentLang === "pt" ? "PARAR AUTOMAÇÃO" : "HALT AUTOMATION";
        outDriftBreaker.textContent = "TRIPPED (BLOCK G7)";
        outDriftBreaker.style.color = "var(--status-err)";
      } else if (isCovariateOrLabelShift) {
        outDriftRegime.textContent = "COV / LABEL SHIFT";
        outDriftRegime.style.color = "var(--status-warn)";
        outDriftAction.textContent = "ExTRA-WCP-T / OTTER";
        outDriftBreaker.textContent = "ADAPTING (G7)";
        outDriftBreaker.style.color = "var(--status-warn)";
      } else {
        outDriftRegime.textContent = "STATIONARY";
        outDriftRegime.style.color = "var(--status-ok)";
        outDriftAction.textContent =
          currentLang === "pt" ? "OPERAR NORMAL" : "NORMAL OPERATION";
        outDriftBreaker.textContent = "CLOSED (OK)";
        outDriftBreaker.style.color = "var(--status-ok)";
      }
    }

    if (driftSvg) {
      const w = 520;
      const h = 210;
      const padL = 48;
      const padR = 24;
      const plotW = w - padL - padR;

      const wTarget = plotW * 0.9;
      const wActual = Math.max(8, plotW * lcFloor);

      driftSvg.innerHTML = `
        <text x="${padL}" y="30" fill="currentColor" font-size="11" font-family="monospace">1. Lei &amp; Candès Conformal Coverage Bound: 1 − α − d_TV = ${(lcFloor * 100).toFixed(1)}%</text>
        <rect x="${padL}" y="40" width="${wTarget.toFixed(1)}" height="18" rx="4" fill="#334155" opacity="0.4" />
        <rect x="${padL}" y="40" width="${wActual.toFixed(1)}" height="18" rx="4" fill="${lcFloor >= 0.75 ? "#38bdf8" : "#fbbf24"}" />
        <text x="${padL + wActual + 8}" y="53" fill="currentColor" font-size="10.5" font-family="monospace">≥ ${(lcFloor * 100).toFixed(1)}%</text>

        <text x="${padL}" y="104" fill="currentColor" font-size="11" font-family="monospace">2. Murphy-CORP Breaker Status: ΔREL = +${dRel.toFixed(3)} | ΔRES = ${dRes.toFixed(3)}</text>
        <rect x="${padL}" y="116" width="${plotW}" height="38" rx="6" fill="${isConceptDrift ? "rgba(239,68,68,0.18)" : isCovariateOrLabelShift ? "rgba(245,158,11,0.18)" : "rgba(16,185,129,0.16)"}" stroke="${isConceptDrift ? "#f87171" : isCovariateOrLabelShift ? "#fbbf24" : "#34d399"}" stroke-width="2" />
        <text x="${padL + plotW / 2}" y="140" text-anchor="middle" fill="${isConceptDrift ? "#f87171" : isCovariateOrLabelShift ? "#fbbf24" : "#34d399"}" font-size="12" font-weight="bold" font-family="monospace">
          ${isConceptDrift ? "CIRCUIT BREAKER OPEN: RESOLUTION COLLAPSED (--defect concept_drift)" : isCovariateOrLabelShift ? "SHIFT DETECTED: APPLY ExTRA-WCP-T &amp; BBSE/MLLS/DFM/OTTER" : "CIRCUIT BREAKER CLOSED: CALIBRATED &amp; RESOLUTION STABLE"}
        </text>
        <text x="${padL + plotW / 2}" y="${h - 8}" text-anchor="middle" fill="currentColor" opacity="0.75" font-size="11" font-family="monospace">Production Drift Governance (stochology_governance::drift)</text>
      `;
    }
  }

  [inputDriftDtv, inputDriftRel, inputDriftRes].forEach((el) => {
    if (el) el.addEventListener("input", updateDriftSimulator);
  });

  /* ------------------------------------------------------------------------
     15. Simulator 7: Epistemological Gates (G1-G8) & Negative Defect Proofs
     ------------------------------------------------------------------------ */
  const gateConsole = document.getElementById("gate-console-output");
  const gateExitBadge = document.getElementById("gate-exit-badge");

  const GATE_OUTPUTS = {
    positive: {
      exitCode: 0,
      cmd: "cargo run --release --bin gates",
      lines: [
        "=== EPISTEMOLOGICAL GATES G1-G8 (STOCHOLOGY PROTOCOL v1.0) ===",
        "  [G1-G5] YAML Cards v1.0 (3/3 from disk, verified against CSVs): Phi1 Simulability, Phi2 Moulton-Campbell/Exceedance, Phi3 Null/Missingness, Phi4 Chouldechova/Exit, Phi5 Resolution -> APPROVED",
        "  [G6] Prohibitions 1-4, Performativity (PS vs PO), Info-Gap + Sniedovich, SDPI/DPI Chain, and CORP-PAV -> APPROVED",
        "  [G7] Bitemporal Clock (t_disp), Canary Column, Single-Opening Vault, and Murphy-CORP Circuit Breaker -> APPROVED",
        "  [G8] Almost-Perfect Machine M=(T,Phi,theta) (P1-P5), Authentic Form (delta(x) + S*), Nonlinear NILE/IRM, and Coin Benchmark -> APPROVED",
        "=== ALL 8 EPISTEMOLOGICAL GATES APPROVED (EXIT CODE 0) ===",
      ],
    },
    cromwell: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect cromwell",
      lines: [
        "NEGATIVE PROOF [cromwell] BLOCKED SUCCESSFULLY:",
        '  Violation: ["Prohibition 4 / Cromwell\'s Rule violated: event_probability_p = 0.000000 outside [0.001000, 0.999000]"]',
        "  Gate Action: Dogmatic probability (0% or 100%) rejected by both Card::audit() and empirical CSV cross-check.",
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
        "  Violation: Nominal pseudoreplication n = 27716 reduced to n_eff(0.90) = 981 via Moulton-Campbell m_tilde = 169.00.",
        "  Gate Action: Boundary Invariant Phi2 rejected naive unclustered standard errors (DEFF_exceedance = 28.26).",
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
        '  Violation: Reopening refused (VaultAlreadyOpened { vault_id: "violated_vault", opened_count: 1 }).',
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
        "  Violation: Algebraic impossibility FPR_A / FPR_B = 5.1342 != 1 under unequal Toronto base rates (p_A=0.3926, p_B=0.1119).",
        "  Gate Action: Boundary Invariant Phi4 rejected uncalibrated simultaneous fairness claim.",
      ],
    },
    machine: {
      exitCode: 1,
      cmd: "cargo run --release --bin gates -- --defect machine",
      lines: [
        "NEGATIVE PROOF [machine] BLOCKED SUCCESSFULLY:",
        "  Violation: DPI gap 0.5625 nats > delta=0.05 (P2=false) and silently dropped spurious atoms rejected (findings_closed=false).",
        "  Gate Action: Gate G8 rejected lossy encoder T(X) and unauthentic model form hiding discarded variables.",
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

      gateConsole.innerHTML =
        `<span class="t-cmd">$ ${entry.cmd}</span>\n` +
        entry.lines
          .map((l) =>
            entry.exitCode === 0 ? `<span class="t-ok">${l}</span>` : `<span class="t-err">${l}</span>`
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
     16. Bilingual Switcher (PT / EN) & Copy-to-Clipboard
     ------------------------------------------------------------------------ */
  const langBtn = document.getElementById("lang-toggle");
  let currentLang = "pt";
  try {
    const savedLang = localStorage.getItem("stochology-lang");
    if (savedLang === "en" || savedLang === "pt") {
      currentLang = savedLang;
    }
  } catch {
    // ignore
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
    document.querySelectorAll("table.data-table").forEach((tbl) => {
      const headers = tbl.querySelectorAll("thead th");
      headers.forEach((th, idx) => {
        if (idx >= 1) {
          tbl.style.setProperty(`--label-${idx + 1}`, `"${th.textContent.trim()}"`);
        }
      });
    });

    syncCourseProgressUI();
    updateMengManskiSimulator();
    updatePerformativeSimulator();
    updateInfoGapSimulator();
    updateItoHillSimulator();
    updateMoultonChoulSimulator();
    updateDecisionSimulator();
    updateDriftSimulator();
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

  // Restore initial chapter from URL hash or localStorage
  const initialHash = window.location.hash.replace("#", "");
  let initialChapter = "ch-0";
  if (CHAPTER_ORDER.includes(initialHash)) {
    initialChapter = initialHash;
  } else if (initialHash.startsWith("sec-")) {
    const match = initialHash.match(/^sec-(\d)-/);
    if (match && CHAPTER_ORDER.includes(`ch-${match[1]}`)) {
      initialChapter = `ch-${match[1]}`;
    }
  } else {
    try {
      const savedCh = localStorage.getItem("stochology-active-chapter");
      if (savedCh && CHAPTER_ORDER.includes(savedCh)) {
        initialChapter = savedCh;
      }
    } catch {
      // ignore
    }
  }

  applyChapterDOMState(initialChapter, false);
  applyLanguage(currentLang);
})();
