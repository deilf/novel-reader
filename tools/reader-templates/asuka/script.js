(function () {
  'use strict';
  // Semantic decoration happens before pagination and preserves source IDs,
  // original characters, image actions and paragraph-comment anchors.
  readerTemplate.on('beforeLayout', function () {
    var first = true;
    Array.prototype.forEach.call(readerTemplate.source.querySelectorAll('.reader-paragraph'), function (p) {
      var text = p.textContent.trim(), scene = /^[*＊※·•—_\-\s]{3,}$/.test(text);
      p.classList.toggle('as-scene', scene);
      p.classList.toggle('as-dialogue', /^[“「『"]/.test(text));
      p.classList.toggle('as-lead', first && !!text && !scene);
      if (text && !scene) first = false;
    });
    Array.prototype.forEach.call(readerTemplate.source.querySelectorAll('.reader-chapter-title'), function (title) {
      var match = title.textContent.match(/第\s*([^章回节卷\s]{1,16})\s*[章回节卷]/);
      title.setAttribute('data-as-chapter', match ? match[1] : 'NEW CHAPTER');
    });
  });
  var active = null, animations = [];
  function stop() {
    animations.forEach(function (a) { a.cancel(); });
    animations = []; active = null;
  }
  function sync(page, index, count) {
    if (!page) return;
    var progress = (index + 1) / Math.max(1, count);
    var value = progress.toFixed(6);
    if (page.getAttribute('data-as-progress') === value) return;
    page.setAttribute('data-as-progress', value);
    page.style.setProperty('--as-progress', value);
    var label = page.querySelector('.as-sync-label');
    if (label) label.textContent = (progress * 100).toFixed(1) + '%';
  }
  function motion(node, frames, duration, delay) {
    if (node && typeof node.animate === 'function') animations.push(node.animate(frames,
      {duration: duration, delay: delay || 0, iterations: Infinity, easing: 'ease-in-out'}));
  }
  function start(page) {
    active = page;
    if (readerTemplate.viewport.height <= 440) return;
    motion(page.querySelector('.as-scan'), [{transform:'translateY(0)',opacity:0},
      {transform:'translateY(9px)',opacity:.12},{transform:'translateY(105px)',opacity:0}], 7400, -1500);
    motion(page.querySelector('.as-orbit-line'), [{transform:'scale(.84)',opacity:.35},
      {transform:'scale(1)',opacity:.8},{transform:'scale(.84)',opacity:.35}], 4800);
    Array.prototype.forEach.call(page.querySelectorAll('.as-wave i'), function (bar, index) {
      motion(bar, [{transform:'scaleY(.6)',opacity:.5},{transform:'scaleY(1)',opacity:.9},
        {transform:'scaleY(.6)',opacity:.5}], 1900 + (index % 3) * 430, -index * 230);
    });
    Array.prototype.forEach.call(page.querySelectorAll('.as-live-dot'), function (dot, index) {
      motion(dot, [{opacity:.5},{opacity:1},{opacity:.5}], 3200, -index * 700);
    });
  }
  readerTemplate.on('afterLayout', function (event) {
    stop();
    event.pages.forEach(function (page, index) { sync(page, index, event.pages.length); });
  });
  readerTemplate.on('pageChange', function (event) { sync(event.page, event.pageIndex, readerTemplate.pageCount); });
  readerTemplate.on('motionChange', function (event) {
    if (active && active !== event.page) stop();
    if (!event.page || event.state === 'settled' || event.reducedMotion) { stop(); return; }
    if (event.state === 'paused') { animations.forEach(function (a) { a.pause(); }); return; }
    if (active === event.page) animations.forEach(function (a) { a.play(); });
    else start(event.page);
  });
  readerTemplate.on('dispose', stop);
})();
