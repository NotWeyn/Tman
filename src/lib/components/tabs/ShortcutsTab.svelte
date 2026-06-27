<script>
  import { t } from '$lib/i18n';
</script>

<div class="tab-content">
  <section class="section">
    <h3 class="section-title">{$t('shortcuts.title')}</h3>
    <p class="section-desc">{@html $t('shortcuts.desc')}</p>
    
    <div class="code-block">
<pre><code>bind = $mainMod, T, exec, screen-translator --capture
bind = $mainMod, R, exec, screen-translator --repeat
bind = $mainMod, I, exec, screen-translator --toggle-interval</code></pre>
    </div>
    <p class="note">{$t('shortcuts.note')}</p>

    <div class="shortcuts-list">
      <div class="shortcut-item">
        <span class="action-name">{$t('shortcuts.select_translate')}</span>
        <kbd>$mod + T</kbd>
      </div>
      <div class="shortcut-item">
        <span class="action-name">{$t('shortcuts.repeat_last')}</span>
        <kbd>$mod + R</kbd>
      </div>
      <div class="shortcut-item">
        <span class="action-name">{$t('shortcuts.toggle_interval')}</span>
        <kbd>$mod + I</kbd>
      </div>
      <div class="shortcut-item">
        <span class="action-name">{$t('shortcuts.copy_clipboard')}</span>
        <kbd>$mod + C</kbd>
      </div>
    </div>
  </section>
</div>
