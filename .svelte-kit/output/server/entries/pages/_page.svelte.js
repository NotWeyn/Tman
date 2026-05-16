import { P as ensure_array_like, J as attr_class, a8 as stringify, Q as escape_html, G as attr } from "../../chunks/renderer.js";
import "@tauri-apps/api/core";
function html(value) {
  var html2 = String(value ?? "");
  var open = "<!---->";
  return open + html2 + "<!---->";
}
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let activeTab = "yakalama";
    const icons = {
      camera: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/><circle cx="12" cy="13" r="3"/></svg>`,
      type: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7"/><line x1="9" x2="15" y1="20" y2="20"/><line x1="12" x2="12" y1="4" y2="20"/></svg>`,
      languages: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/></svg>`,
      smartphone: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/></svg>`,
      keyboard: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2" ry="2"/><path d="M6 8h.001"/><path d="M10 8h.001"/><path d="M14 8h.001"/><path d="M18 8h.001"/><path d="M8 12h.001"/><path d="M12 12h.001"/><path d="M16 12h.001"/><path d="M7 16h10"/></svg>`,
      clock: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`,
      settings: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>`
    };
    const tabs = [
      { id: "yakalama", label: "Yakalama", icon: icons.camera },
      { id: "ocr", label: "OCR", icon: icons.type },
      { id: "ceviri", label: "Çeviri", icon: icons.languages },
      { id: "mobil", label: "Mobil sunucu", icon: icons.smartphone },
      { id: "kisayollar", label: "Kısayollar", icon: icons.keyboard },
      { id: "gecmis", label: "Geçmiş", icon: icons.clock },
      { id: "uygulama", label: "Uygulama", icon: icons.settings }
    ];
    let captureMode = "manuel";
    let regionMemory = false;
    let grayscale = false;
    let binarization = false;
    let contrast = "kapali";
    let scale = "2x";
    let copyToClipboard = true;
    let showInGui = false;
    let lastRegion = "";
    async function saveConfig() {
      return;
    }
    {
      saveConfig();
    }
    $$renderer2.push(`<div class="layout svelte-1uha8ag"><aside class="sidebar svelte-1uha8ag"><div class="sidebar-header svelte-1uha8ag"><span class="title svelte-1uha8ag">AYARLAR</span></div> <nav class="nav-menu svelte-1uha8ag"><!--[-->`);
    const each_array = ensure_array_like(
      // Server toggle changed
      // TXT
      // Invoke once immediately
      // TODO: Connect to Rust backend to run uv pip install rapidocr-onnxruntime / easyocr
      tabs
    );
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let tab = each_array[$$index];
      $$renderer2.push(`<button${attr_class(`nav-item ${stringify(activeTab === tab.id ? "active" : "")}`, "svelte-1uha8ag")}><div class="nav-icon-wrapper">${html(tab.icon)}</div> <span>${escape_html(tab.label)}</span></button>`);
    }
    $$renderer2.push(`<!--]--></nav></aside> <main class="content-area svelte-1uha8ag"><div class="content-scroll svelte-1uha8ag">`);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="tab-content svelte-1uha8ag"><div class="action-header svelte-1uha8ag"><button${attr_class(`btn ${stringify("btn-primary")} btn-large w-100`, "svelte-1uha8ag")}>${html(icons.camera)} ${escape_html("Ekranı Yakala")}</button> `);
      {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<div class="row svelte-1uha8ag" style="margin-top: 15px; flex-direction: column; align-items: stretch; gap: 5px;"><div class="label-desc text-center" style="opacity: 0.8; font-size: 0.85em;">Son yakalanan bölge koordinatları (Düzenleyebilirsiniz)</div> <input type="text" class="form-input code-font text-center svelte-1uha8ag"${attr("value", lastRegion)} placeholder="Örn: 1000,300 200x50"/></div>`);
      }
      $$renderer2.push(`<!--]--></div> <section class="section svelte-1uha8ag"><h3 class="section-title svelte-1uha8ag">YAKALAMA MODU</h3> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Mod</label> <div class="label-desc">Ekranın nasıl yakalanacağını seçin.</div></div> `);
      $$renderer2.select(
        { value: captureMode, class: "form-select w-auto" },
        ($$renderer3) => {
          $$renderer3.option({ value: "manuel" }, ($$renderer4) => {
            $$renderer4.push(`Manuel`);
          });
          $$renderer3.option({ value: "interval" }, ($$renderer4) => {
            $$renderer4.push(`Interval (Aralıklı)`);
          });
          $$renderer3.option({ value: "degisim" }, ($$renderer4) => {
            $$renderer4.push(`Değişim algılama`);
          });
        },
        "svelte-1uha8ag"
      );
      $$renderer2.push(`</div> `);
      {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--> `);
      {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Bölge hafızası</label> <div class="label-desc">Son seçilen koordinatları kaydeder, tekrar seçmek gerekmez.</div></div> <label class="toggle-wrapper"><input type="checkbox"${attr("checked", regionMemory, true)} class="toggle-input"/> <div class="toggle-bg"><div class="toggle-dot"></div></div></label></div></section> <section class="section svelte-1uha8ag"><h3 class="section-title svelte-1uha8ag">GÖRÜNTÜ ÖN İŞLEME</h3> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Gri tonlama (Grayscale)</label> <div class="label-desc">Görüntüyü siyah-beyaza çevirerek OCR doğruluğunu artırır.</div></div> <label class="toggle-wrapper"><input type="checkbox"${attr("checked", grayscale, true)} class="toggle-input"/> <div class="toggle-bg"><div class="toggle-dot"></div></div></label></div> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Binarization</label> <div class="label-desc">Gürültüyü azaltır, metni keskinleştirir.</div></div> <label class="toggle-wrapper"><input type="checkbox"${attr("checked", binarization, true)} class="toggle-input"/> <div class="toggle-bg"><div class="toggle-dot"></div></div></label></div> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Kontrast artırma</label></div> `);
      $$renderer2.select(
        { value: contrast, class: "form-select w-auto" },
        ($$renderer3) => {
          $$renderer3.option({ value: "kapali" }, ($$renderer4) => {
            $$renderer4.push(`Kapalı`);
          });
          $$renderer3.option({ value: "hafif" }, ($$renderer4) => {
            $$renderer4.push(`Hafif`);
          });
          $$renderer3.option({ value: "guclu" }, ($$renderer4) => {
            $$renderer4.push(`Güçlü`);
          });
        },
        "svelte-1uha8ag"
      );
      $$renderer2.push(`</div> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Ölçekleme</label> <div class="label-desc">Küçük fontları büyütmek için.</div></div> `);
      $$renderer2.select(
        { value: scale, class: "form-select w-auto" },
        ($$renderer3) => {
          $$renderer3.option({ value: "1x" }, ($$renderer4) => {
            $$renderer4.push(`1x`);
          });
          $$renderer3.option({ value: "2x" }, ($$renderer4) => {
            $$renderer4.push(`2x (Önerilen)`);
          });
          $$renderer3.option({ value: "3x" }, ($$renderer4) => {
            $$renderer4.push(`3x`);
          });
        },
        "svelte-1uha8ag"
      );
      $$renderer2.push(`</div></section> <section class="section svelte-1uha8ag"><h3 class="section-title svelte-1uha8ag">ÇIKTI</h3> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>Sonucu panoya kopyala</label></div> <label class="toggle-wrapper"><input type="checkbox"${attr("checked", copyToClipboard, true)} class="toggle-input"/> <div class="toggle-bg"><div class="toggle-dot"></div></div></label></div> <div class="row svelte-1uha8ag"><div class="row-info svelte-1uha8ag"><label>GUI'de göster</label></div> <label class="toggle-wrapper"><input type="checkbox"${attr("checked", showInGui, true)} class="toggle-input"/> <div class="toggle-bg"><div class="toggle-dot"></div></div></label></div></section></div>`);
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div></main></div>`);
  });
}
export {
  _page as default
};
