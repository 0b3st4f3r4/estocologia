(function () {
  "use strict";

  function normalizeMathText(text) {
    if (text.indexOf("$") === -1) return text;
    return text.replace(/(\$\$[\s\S]*?\$\$|\$[^$\n]+\$)/g, function (seg) {
      return seg
        .replace(/’/g, "'")
        .replace(/(^|[^\\])%/g, "$1\\%");
    });
  }

  function prepareDocumentForMathAndTables() {
    var root = document.querySelector("main") || document.body;
    if (!root) return;

    var walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT, {
      acceptNode: function (node) {
        var parent = node.parentElement;
        if (!parent) return NodeFilter.FILTER_REJECT;
        var tag = parent.tagName;
        if (tag === "CODE" || tag === "PRE" || tag === "SCRIPT" || tag === "STYLE") {
          return NodeFilter.FILTER_REJECT;
        }
        return node.nodeValue && node.nodeValue.indexOf("$") !== -1
          ? NodeFilter.FILTER_ACCEPT
          : NodeFilter.FILTER_SKIP;
      }
    });

    var nodes = [];
    while (walker.nextNode()) {
      nodes.push(walker.currentNode);
    }
    for (var i = 0; i < nodes.length; i++) {
      nodes[i].nodeValue = normalizeMathText(nodes[i].nodeValue);
    }

    var wrappers = root.querySelectorAll(".table-wrapper");
    for (var j = 0; j < wrappers.length; j++) {
      var wrapper = wrappers[j];
      if (!wrapper.hasAttribute("tabindex")) {
        wrapper.setAttribute("tabindex", "0");
      }
      if (!wrapper.hasAttribute("role")) {
        wrapper.setAttribute("role", "region");
      }
      if (!wrapper.hasAttribute("aria-label") && !wrapper.hasAttribute("aria-labelledby")) {
        var prev = wrapper.previousElementSibling;
        while (prev && !/^H[1-6]$/.test(prev.tagName)) {
          prev = prev.previousElementSibling;
        }
        var label = prev ? prev.textContent.trim() : "Protocol specification table";
        wrapper.setAttribute("aria-label", label);
      }
    }

    var svgImgs = root.querySelectorAll('p > img[src*="figures/"][src$=".svg"]');
    for (var k = 0; k < svgImgs.length; k++) {
      var img = svgImgs[k];
      var parentP = img.parentElement;
      if (!parentP || parentP.children.length !== 1) continue;
      var altText = (img.getAttribute("alt") || "").trim();
      var fig = document.createElement("figure");
      fig.className = "stoch-figure";
      parentP.parentNode.replaceChild(fig, parentP);
      fig.appendChild(img);
      if (altText) {
        var cap = document.createElement("figcaption");
        var strong = document.createElement("strong");
        strong.textContent = altText;
        cap.appendChild(strong);
        fig.appendChild(cap);
      }
    }

    setupMermaidDiagrams(root);
  }

  function isDarkThemeActive() {
    var cl = document.documentElement.classList;
    if (cl.contains("navy") || cl.contains("coal") || cl.contains("ayu")) {
      return true;
    }
    if (cl.contains("light") || cl.contains("rust")) {
      return false;
    }
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches;
  }

  function getMermaidConfig(dark) {
    return {
      startOnLoad: false,
      securityLevel: "strict",
      theme: "base",
      fontFamily: '"Inter", "Open Sans", system-ui, -apple-system, "Segoe UI", Roboto, sans-serif',
      flowchart: {
        htmlLabels: false,
        curve: "basis",
        padding: 16,
        nodeSpacing: 36,
        rankSpacing: 46
      },
      state: {
        dividerMargin: 10,
        sizeUnit: 5,
        padding: 12
      },
      themeVariables: dark
        ? {
            darkMode: true,
            background: "#0f1626",
            primaryColor: "#162036",
            primaryTextColor: "#e2e8f0",
            primaryBorderColor: "#38bdf8",
            secondaryColor: "#1e1b4b",
            secondaryTextColor: "#e2e8f0",
            secondaryBorderColor: "#a78bfa",
            tertiaryColor: "#064e3b",
            tertiaryTextColor: "#e2e8f0",
            tertiaryBorderColor: "#34d399",
            lineColor: "#64748b",
            textColor: "#e2e8f0",
            mainBkg: "#162036",
            nodeBorder: "#38bdf8",
            clusterBkg: "#0b1120",
            clusterBorder: "#1e2d4a",
            titleColor: "#f8fafc",
            edgeLabelBackground: "#0f1626"
          }
        : {
            darkMode: false,
            background: "#ffffff",
            primaryColor: "#f1f5f9",
            primaryTextColor: "#0f172a",
            primaryBorderColor: "#0284c7",
            secondaryColor: "#f5f3ff",
            secondaryTextColor: "#0f172a",
            secondaryBorderColor: "#6d28d9",
            tertiaryColor: "#ecfdf5",
            tertiaryTextColor: "#0f172a",
            tertiaryBorderColor: "#059669",
            lineColor: "#475569",
            textColor: "#0f172a",
            mainBkg: "#f1f5f9",
            nodeBorder: "#0284c7",
            clusterBkg: "#f8fafc",
            clusterBorder: "#cbd5e1",
            titleColor: "#0f172a",
            edgeLabelBackground: "#ffffff"
          }
    };
  }

  var lastMermaidDarkState = null;

  function renderMermaidDiagrams(force) {
    if (!window.mermaid) return;
    var dark = isDarkThemeActive();
    if (!force && lastMermaidDarkState === dark) return;
    lastMermaidDarkState = dark;

    window.mermaid.initialize(getMermaidConfig(dark));
    var containers = document.querySelectorAll(".mermaid[data-mermaid-src]");
    for (var i = 0; i < containers.length; i++) {
      var el = containers[i];
      el.removeAttribute("data-processed");
      el.textContent = el.getAttribute("data-mermaid-src");
    }
    if (containers.length > 0 && typeof window.mermaid.run === "function") {
      window.mermaid.run({ querySelector: ".mermaid" });
    }
  }

  function setupMermaidDiagrams(root) {
    var codeBlocks = root.querySelectorAll("pre > code.language-mermaid");
    if (!codeBlocks.length) return;

    for (var i = 0; i < codeBlocks.length; i++) {
      var code = codeBlocks[i];
      var pre = code.parentElement;
      if (!pre) continue;
      var src = (code.textContent || "").trim();
      var fig = document.createElement("figure");
      fig.className = "stoch-figure stoch-mermaid-figure";
      fig.setAttribute("role", "group");

      var prev = pre.previousElementSibling;
      while (prev && !/^H[1-6]$/.test(prev.tagName)) {
        prev = prev.previousElementSibling;
      }
      fig.setAttribute(
        "aria-label",
        prev ? prev.textContent.trim() + " — Diagram" : "Stochology Protocol Diagram"
      );

      var div = document.createElement("div");
      div.className = "mermaid";
      div.setAttribute("data-mermaid-src", src);
      div.textContent = src;
      fig.appendChild(div);
      pre.parentNode.replaceChild(fig, pre);
    }

    var mScript = document.createElement("script");
    mScript.src = "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js";
    mScript.async = true;
    mScript.onload = function () {
      renderMermaidDiagrams(true);
      if (typeof MutationObserver !== "undefined") {
        var obs = new MutationObserver(function () {
          renderMermaidDiagrams(false);
        });
        obs.observe(document.documentElement, {
          attributes: true,
          attributeFilter: ["class"]
        });
      }
    };
    document.head.appendChild(mScript);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", prepareDocumentForMathAndTables);
  } else {
    prepareDocumentForMathAndTables();
  }

  window.MathJax = {
    tex: {
      inlineMath: [["$", "$"], ["\\(", "\\)"]],
      displayMath: [["$$", "$$"], ["\\[", "\\]"]],
      processEscapes: true
    },
    options: {
      skipHtmlTags: ["script", "noscript", "style", "textarea", "pre", "code"]
    }
  };

  var s = document.createElement("script");
  s.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-chtml.js";
  s.async = true;
  document.head.appendChild(s);
})();
